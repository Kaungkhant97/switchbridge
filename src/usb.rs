//! Redacted macOS USB evidence collection. Enumeration does not verify video.

use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

const MAX_INVENTORY_BYTES: u64 = 16 * 1024 * 1024;

pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    match args {
        [command, path] if command == "usb-snapshot" => {
            let inventory = collect_inventory()?;
            write_inventory(Path::new(path), &inventory)?;
            println!("Saved redacted USB inventory. Enumeration and video remain UNVERIFIED.");
            Ok(())
        }
        [command, before, after] if command == "usb-diff" => {
            let before = read_inventory(Path::new(before))?;
            let after = read_inventory(Path::new(after))?;
            print!("{}", inventory_diff(&before, &after)?);
            Ok(())
        }
        _ => Err("usage: switchbridge usb-snapshot PATH | usb-diff BEFORE AFTER".into()),
    }
}

fn command_text(program: &str, args: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = Command::new(program)
        .args(args)
        .env("LC_ALL", "C")
        .env("LANG", "C")
        .output()?;
    if !output.status.success() {
        return Err(format!("{program} failed with {}", output.status).into());
    }
    if output.stdout.len() as u64 > MAX_INVENTORY_BYTES {
        return Err("USB inventory exceeds 16 MiB limit".into());
    }
    let text = String::from_utf8(output.stdout)?;
    if text.trim().is_empty() {
        return Err(format!("{program} returned empty output").into());
    }
    Ok(text)
}

fn collect_inventory() -> Result<String, Box<dyn Error>> {
    if !cfg!(target_os = "macos") {
        return Err("USB snapshots require macOS".into());
    }
    let supported = command_text("/usr/sbin/system_profiler", &["-listDataTypes"])?;
    let kind = select_usb_type(&supported)?;
    let inventory = command_text("/usr/sbin/system_profiler", &[kind, "-detailLevel", "full"])?;
    let model = command_text("/usr/sbin/sysctl", &["-n", "hw.model"])?;
    let os = command_text("/usr/bin/sw_vers", &["-productVersion"])?;
    Ok(format!(
        "SwitchBridge USB snapshot v1\nMac model: {}\nmacOS: {}\nProfiler data type: {kind}\nVideo capture: UNVERIFIED\n\n{}",
        model.trim(),
        os.trim(),
        inventory
    ))
}

fn redact_inventory(inventory: &str) -> Result<String, Box<dyn Error>> {
    Ok(replace_private_values(
        inventory,
        &collect_private_values(inventory)?,
    ))
}

fn collect_private_values(inventory: &str) -> Result<Vec<&str>, Box<dyn Error>> {
    let mut private_values = Vec::new();
    let mut private_indent = None;
    for line in inventory.lines().filter(|line| !line.trim().is_empty()) {
        let indent = line.len() - line.trim_start().len();
        if let Some(previous_indent) = private_indent
            && (indent > previous_indent || !line.contains(':'))
        {
            return Err("USB inventory contains a private field continuation".into());
        }
        private_indent = None;
        if let Some((key, value)) = line.split_once(':') {
            if sensitive_key(key) {
                let value = value.trim();
                if value.is_empty() {
                    return Err("USB inventory contains an empty or multiline private field".into());
                }
                private_values.push(value);
                private_indent = Some(indent);
            }
        } else if malformed_private_field(line) {
            return Err("USB inventory contains a malformed private field".into());
        }
    }
    private_values.sort_by_key(|value| std::cmp::Reverse(value.len()));
    private_values.dedup();
    Ok(private_values)
}

// Preserve structural field labels and VID/PID evidence. Empty-value lines are
// device headers, whose private identifiers must still be redacted.
fn replace_private_values(inventory: &str, private_values: &[&str]) -> String {
    inventory
        .split_inclusive('\n')
        .map(|line| match line.split_once(':') {
            Some((key, _)) if matches!(key.trim(), "Vendor ID" | "Product ID") => line.to_owned(),
            Some((key, value))
                if sensitive_key(key)
                    || (!line.trim_end().ends_with(':') && !value.trim().is_empty()) =>
            {
                format!("{key}:{}", replace_private_tokens(value, private_values))
            }
            _ => replace_private_tokens(line, private_values),
        })
        .collect()
}

// Long identifiers may be embedded in device names. Only ambiguous values of
// fewer than four Unicode characters require identifier boundaries.
fn replace_private_tokens(text: &str, private_values: &[&str]) -> String {
    let mut offset = 0;
    let mut redacted = String::new();
    while offset < text.len() {
        let remaining = &text[offset..];
        let boundary_before = text[..offset]
            .chars()
            .next_back()
            .is_none_or(|c| !identifier_char(c));
        if let Some(value) = private_values.iter().find(|value| {
            remaining.starts_with(**value)
                && (value.chars().count() >= 4
                    || (boundary_before
                        && remaining[value.len()..]
                            .chars()
                            .next()
                            .is_none_or(|c| !identifier_char(c))))
        }) {
            redacted.push_str("[REDACTED]");
            offset += value.len();
        } else {
            let character = remaining
                .chars()
                .next()
                .expect("remaining text is nonempty");
            redacted.push(character);
            offset += character.len_utf8();
        }
    }
    redacted
}

fn identifier_char(character: char) -> bool {
    character.is_alphanumeric() || matches!(character, '_' | '-')
}

const PRIVATE_LABELS: &[&str] = &[
    "serial number",
    "serial",
    "uuid",
    "udid",
    "device uuid",
    "device udid",
    "hardware uuid",
    "usb serial number",
    "device serial number",
];

fn sensitive_key(key: &str) -> bool {
    let key = key
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase();
    PRIVATE_LABELS.contains(&key.as_str())
        || key.ends_with("uuid")
        || key.ends_with("udid")
        || key.contains("serial number")
}

fn malformed_private_field(line: &str) -> bool {
    sensitive_key(line)
        || line
            .char_indices()
            .any(|(offset, character)| character.is_whitespace() && sensitive_key(&line[..offset]))
}

fn inventory_diff(before: &str, after: &str) -> Result<String, Box<dyn Error>> {
    // Validate both inputs before producing output, then share known identifiers.
    let mut private_values = collect_private_values(before)?;
    private_values.extend(collect_private_values(after)?);
    private_values.sort_by_key(|value| std::cmp::Reverse(value.len()));
    private_values.dedup();
    let before = replace_private_values(before, &private_values);
    let after = replace_private_values(after, &private_values);
    if before == after {
        return Ok("No USB inventory changes.\n".into());
    }
    let old: Vec<&str> = before.lines().collect();
    let new: Vec<&str> = after.lines().collect();
    let prefix = old.iter().zip(&new).take_while(|(a, b)| a == b).count();
    let suffix = old[prefix..]
        .iter()
        .rev()
        .zip(new[prefix..].iter().rev())
        .take_while(|(a, b)| a == b)
        .count();
    let start = prefix.saturating_sub(3);
    let old_end = (old.len() - suffix + 3).min(old.len());
    let new_end = (new.len() - suffix + 3).min(new.len());
    let mut output = format!(
        "--- before\n+++ after\n@@ -{},{} +{},{} @@\n",
        start + 1,
        old_end - start,
        start + 1,
        new_end - start
    );
    for line in &old[start..prefix] {
        output.push_str(&format!(" {line}\n"));
    }
    for line in &old[prefix..old.len() - suffix] {
        output.push_str(&format!("-{line}\n"));
    }
    for line in &new[prefix..new.len() - suffix] {
        output.push_str(&format!("+{line}\n"));
    }
    for line in &old[old.len() - suffix..old_end] {
        output.push_str(&format!(" {line}\n"));
    }
    Ok(output)
}

fn select_usb_type(supported: &str) -> Result<&'static str, Box<dyn Error>> {
    ["SPUSBHostDataType", "SPUSBDataType"]
        .into_iter()
        .find(|kind| supported.lines().any(|line| line.trim() == *kind))
        .ok_or_else(|| "USB profiler unavailable".into())
}

fn read_inventory(path: &Path) -> Result<String, Box<dyn Error>> {
    let mut bytes = Vec::new();
    File::open(path)?
        .take(MAX_INVENTORY_BYTES + 1)
        .read_to_end(&mut bytes)?;
    if bytes.is_empty() || bytes.len() as u64 > MAX_INVENTORY_BYTES {
        return Err("USB inventory must contain 1 byte to 16 MiB".into());
    }
    Ok(String::from_utf8(bytes)?)
}

fn write_inventory(path: &Path, inventory: &str) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent().filter(|path| !path.as_os_str().is_empty()) {
        fs::create_dir_all(parent)?;
    }
    let redacted = redact_inventory(inventory)?;
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(redacted.as_bytes())?;
    file.sync_all()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redacts_serial_values_everywhere_while_preserving_device_identifiers() {
        let raw = "USB:\n    Switch ABC123:\n      Product ID: 0x201d\n      Vendor ID: 0x057e\n      Serial Number: ABC123\n      Device UUID: DEAD-BEEF\n      Related: DEAD-BEEF\n";
        let clean = redact_inventory(raw).unwrap();
        assert!(!clean.contains("ABC123"));
        assert!(!clean.contains("DEAD-BEEF"));
        assert!(clean.contains("Vendor ID: 0x057e"));
        assert!(clean.contains("Product ID: 0x201d"));
        assert!(clean.contains("Serial Number: [REDACTED]"));
    }

    #[test]
    fn retains_inventory_without_private_identifiers() {
        assert_eq!(
            redact_inventory("USB:\n  Vendor ID: 0x057e\n").unwrap(),
            "USB:\n  Vendor ID: 0x057e\n"
        );
    }

    #[test]
    fn diff_preserves_changed_device_context_and_redacts_both_inputs() {
        let before = "USB:\n  Mouse:\n    Product ID: 0x0001\n";
        let after = "USB:\n  Switch:\n    Product ID: 0x201d\n    Serial Number: SECRET42\n";
        let diff = inventory_diff(before, after).unwrap();
        assert!(diff.contains("-  Mouse:"));
        assert!(diff.contains("+  Switch:"));
        assert!(diff.contains("+    Product ID: 0x201d"));
        assert!(!diff.contains("SECRET42"));
        assert!(diff.contains("@@ -1,3 +1,4 @@"));
    }

    #[test]
    fn identical_inventory_reports_no_changes() {
        assert_eq!(
            inventory_diff("USB:\n", "USB:\n").unwrap(),
            "No USB inventory changes.\n"
        );
    }

    #[test]
    fn detects_current_and_legacy_usb_profiler_types() {
        assert_eq!(
            select_usb_type("SPUSBDataType\nSPUSBHostDataType\n").unwrap(),
            "SPUSBHostDataType"
        );
        assert_eq!(select_usb_type("SPUSBDataType\n").unwrap(), "SPUSBDataType");
        assert!(select_usb_type("SPHardwareDataType\n").is_err());
    }
    fn temp_dir() -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "switchbridge-usb-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&dir).unwrap();
        dir
    }

    #[test]
    fn rejects_missing_and_extra_command_arguments() {
        for args in [
            vec![],
            vec!["usb-snapshot".into()],
            vec!["usb-diff".into(), "one".into()],
            vec!["unknown".into()],
        ] {
            assert!(run(&args).is_err());
        }
    }

    #[test]
    fn writes_redacted_inventory_and_refuses_to_replace_existing_evidence() {
        let dir = temp_dir();
        let path = dir.join("nested/before.txt");
        write_inventory(&path, "USB:\n  Serial Number: SECRET\n").unwrap();
        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            "USB:\n  Serial Number: [REDACTED]\n"
        );
        assert!(write_inventory(&path, "replacement").is_err());
        assert!(std::fs::read_to_string(&path).unwrap().contains("USB:"));
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn rejects_empty_invalid_utf8_and_oversized_inventory_inputs() {
        let dir = temp_dir();
        let path = dir.join("input");
        for bytes in [Vec::new(), vec![255], vec![b'x'; 16 * 1024 * 1024 + 1]] {
            std::fs::write(&path, bytes).unwrap();
            assert!(read_inventory(&path).is_err());
        }
        std::fs::write(&path, "USB:\n  Vendor ID: 0x057e\n").unwrap();
        assert!(read_inventory(&path).unwrap().contains("0x057e"));
        std::fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn redacts_longest_private_values_first() {
        let clean = redact_inventory("Serial Number: AB\nDevice UUID: ABCDEF\n").unwrap();
        assert_eq!(
            clean,
            "Serial Number: [REDACTED]\nDevice UUID: [REDACTED]\n"
        );
    }

    #[test]
    fn fails_closed_for_malformed_sensitive_fields() {
        for raw in [
            "Serial Number SECRET42\n",
            "Serial Number:\n    SECRET42\n",
            "Device UUID SECRET42\n",
        ] {
            assert!(redact_inventory(raw).is_err());
            assert!(inventory_diff("USB:\n", raw).is_err());
        }
    }
    #[test]
    fn rejects_sensitive_continuations_even_after_nonempty_first_line() {
        for raw in [
            "Serial Number: FIRST\n    SECOND\n",
            "Serial Number: FIRST\n    SECOND: MORE\n",
            "Serial Number: FIRST\n\n    SECOND\n",
        ] {
            assert!(redact_inventory(raw).is_err());
        }
    }

    fn assert_cross_inventory_redaction(reverse: bool) {
        let identified =
            "USB:\n  Device:\n    Serial Number: SECRET42\n    Device UUID: PRIVATE-UUID\n";
        let repeated = "USB:\n  Device SECRET42 (PRIVATE-UUID):\n";
        let (before, after) = if reverse {
            (repeated, identified)
        } else {
            (identified, repeated)
        };
        let diff = inventory_diff(before, after).unwrap();
        assert!(!diff.contains("SECRET42"), "{diff}");
        assert!(!diff.contains("PRIVATE-UUID"), "{diff}");
        assert!(diff.contains("Device [REDACTED] ([REDACTED]):"));
    }

    #[test]
    fn diff_redacts_private_values_known_only_to_before_inventory() {
        assert_cross_inventory_redaction(false);
    }

    #[test]
    fn diff_redacts_private_values_known_only_to_after_inventory() {
        assert_cross_inventory_redaction(true);
    }

    #[test]
    fn long_private_values_are_redacted_inside_device_names_and_values() {
        let raw = "USB:\n  Device-ABC123:\n  DeviceABC123:\n  DevicePRIVATE-UUIDsuffix:\n    Serial Number: ABC123\n    Device UUID: PRIVATE-UUID\n    Related: prefixABC123suffix prefixPRIVATE-UUIDsuffix\n";
        let clean = redact_inventory(raw).unwrap();
        assert!(!clean.contains("ABC123"), "{clean}");
        assert!(!clean.contains("PRIVATE-UUID"), "{clean}");
        assert!(clean.contains("Device-[REDACTED]:"));
        assert!(clean.contains("Device[REDACTED]:"));
        assert!(clean.contains("Device[REDACTED]suffix:"));
        assert!(clean.contains("prefix[REDACTED]suffix"));
    }

    fn assert_cross_inventory_embedded_redaction(reverse: bool) {
        let identified =
            "USB:\n  Device:\n    Serial Number: ABC123\n    Device UUID: PRIVATE-UUID\n";
        let repeated = "USB:\n  Device-ABC123:\n  DeviceABC123:\n  DevicePRIVATE-UUIDsuffix:\n";
        let (before, after) = if reverse {
            (repeated, identified)
        } else {
            (identified, repeated)
        };
        let diff = inventory_diff(before, after).unwrap();
        assert!(!diff.contains("ABC123"), "{diff}");
        assert!(!diff.contains("PRIVATE-UUID"), "{diff}");
    }

    #[test]
    fn diff_redacts_embedded_values_known_only_to_before_inventory() {
        assert_cross_inventory_embedded_redaction(false);
    }

    #[test]
    fn diff_redacts_embedded_values_known_only_to_after_inventory() {
        assert_cross_inventory_embedded_redaction(true);
    }

    #[test]
    fn private_label_variants_redact_values_and_fail_closed() {
        for label in [
            "Volume UUID",
            "Interface UDID",
            "Qualified Serial Number (system)",
            "  volume   uuid  ",
        ] {
            let raw = format!("{label}: 1234-ABCD\n");
            assert_eq!(
                redact_inventory(&raw).unwrap(),
                format!("{label}: [REDACTED]\n")
            );
            for malformed in [
                format!("{label}:\n"),
                format!("{label} SECRET\n"),
                format!("{label}: FIRST\n  SECOND\n"),
            ] {
                assert!(
                    redact_inventory(&malformed).is_err(),
                    "accepted {malformed:?}"
                );
                assert!(inventory_diff("USB:\n", &malformed).is_err());
            }
        }
        let ordinary = "USB:\n  USB Serial:\n    Product ID: 0x7523\n";
        assert_eq!(redact_inventory(ordinary).unwrap(), ordinary);
    }

    #[test]
    fn colon_bearing_private_values_are_redacted_in_snapshot_headings() {
        let directory = temp_dir();
        let path = directory.join("snapshot.txt");
        let raw = "USB:\n  Dev 00:11:22:33:44:55:\n    Serial Number: 00:11:22:33:44:55\n    Vendor ID: 0x057e\n    Product ID: 0x201d\n";
        write_inventory(&path, raw).unwrap();
        let clean = fs::read_to_string(&path).unwrap();
        fs::remove_dir_all(directory).unwrap();
        assert_eq!(
            clean,
            "USB:\n  Dev [REDACTED]:\n    Serial Number: [REDACTED]\n    Vendor ID: 0x057e\n    Product ID: 0x201d\n"
        );
        assert_eq!(
            redact_inventory("Serial Number: 00:11:22:33:44:55:\n").unwrap(),
            "Serial Number: [REDACTED]\n"
        );
    }

    fn assert_colon_bearing_cross_inventory_redaction(reverse: bool) {
        let identified = "USB:\n  Dev:\n    Serial Number: 00:11:22:33:44:55\n";
        let repeated = "USB:\n  Dev 00:11:22:33:44:55:\n";
        let (before, after) = if reverse {
            (repeated, identified)
        } else {
            (identified, repeated)
        };
        let diff = inventory_diff(before, after).unwrap();
        assert!(!diff.contains("00:11:22:33:44:55"), "{diff}");
        assert!(diff.contains("Dev [REDACTED]:"));
    }

    #[test]
    fn colon_bearing_private_values_known_only_before_are_redacted_in_diff() {
        assert_colon_bearing_cross_inventory_redaction(false);
    }

    #[test]
    fn colon_bearing_private_values_known_only_after_are_redacted_in_diff() {
        assert_colon_bearing_cross_inventory_redaction(true);
    }

    #[test]
    fn ordinary_serial_device_headers_are_not_private_fields() {
        let raw = "USB:\n  USB Serial:\n    Product ID: 0x7523\n  USB-Serial Controller D:\n    Vendor ID: 0x1a86\n";
        assert_eq!(redact_inventory(raw).unwrap(), raw);
    }

    #[test]
    fn known_private_labels_still_fail_closed_without_single_line_values() {
        for label in [
            "Serial Number",
            "Serial",
            "UUID",
            "UDID",
            "Device UUID",
            "Device UDID",
            "Hardware UUID",
            "USB Serial Number",
        ] {
            for raw in [
                format!("{label}:\n"),
                format!("{label} SECRET\n"),
                format!("{label}: FIRST\n  SECOND\n"),
            ] {
                assert!(redact_inventory(&raw).is_err(), "accepted {raw:?}");
            }
        }
    }

    #[test]
    fn short_serials_redact_complete_occurrences_without_corrupting_ids_or_keys() {
        let raw = "USB:\n  USB Serial 0 (0):\n    Vendor ID: 0x057e\n    Product ID: 0x201d\n    Serial Number: 0\n    Related: 0 / 10 / x0 / 0x057e / 0_1 / 0-1\n";
        assert_eq!(
            redact_inventory(raw).unwrap(),
            "USB:\n  USB Serial [REDACTED] ([REDACTED]):\n    Vendor ID: 0x057e\n    Product ID: 0x201d\n    Serial Number: [REDACTED]\n    Related: [REDACTED] / 10 / x0 / 0x057e / 0_1 / 0-1\n"
        );
        let keys = "USB:\n  Number:\n    Serial Number: Number\n    Related: Number\n";
        assert_eq!(
            redact_inventory(keys).unwrap(),
            "USB:\n  [REDACTED]:\n    Serial Number: [REDACTED]\n    Related: [REDACTED]\n"
        );
    }

    #[test]
    fn complete_private_tokens_preserve_unicode_neighbors_and_identifier_evidence() {
        let raw = "Serial Number: A\nDevice UUID: AB-CD\nRelated: A (AB-CD) A1 éA Aé _A A-B\nVendor ID: A\nProduct ID: AB-CD\n";
        assert_eq!(
            redact_inventory(raw).unwrap(),
            "Serial Number: [REDACTED]\nDevice UUID: [REDACTED]\nRelated: [REDACTED] ([REDACTED]) A1 éA Aé _A A-B\nVendor ID: A\nProduct ID: AB-CD\n"
        );
    }

    #[test]
    fn short_private_values_do_not_modify_inserted_redaction_markers() {
        assert_eq!(
            redact_inventory("Serial Number: A\nDevice UUID: ABCDEF\n").unwrap(),
            "Serial Number: [REDACTED]\nDevice UUID: [REDACTED]\n"
        );
    }
}
