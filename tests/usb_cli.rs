use std::fs;
use std::process::Command;

#[test]
fn usb_diff_cli_reads_paths_and_redacts_private_identifiers() {
    let directory =
        std::env::temp_dir().join(format!("switchbridge-usb-cli-{}", std::process::id()));
    fs::create_dir(&directory).unwrap();
    let before = directory.join("before.txt");
    let after = directory.join("after.txt");
    fs::write(&before, "USB:\n  Mouse:\n    Serial Number: OLD_SECRET\n").unwrap();
    fs::write(&after, "USB:\n  Console:\n    Vendor ID: 0x057e\n    Serial Number: NEW_SECRET\n    Device UUID: PRIVATE_UUID\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_switchbridge"))
        .args([
            "usb-diff",
            before.to_str().unwrap(),
            after.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    fs::remove_dir_all(directory).unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("-  Mouse:"));
    assert!(text.contains("+  Console:"));
    assert!(text.contains("+    Vendor ID: 0x057e"));
    assert!(text.contains("[REDACTED]"));
    for secret in ["OLD_SECRET", "NEW_SECRET", "PRIVATE_UUID"] {
        assert!(!text.contains(secret));
    }
}

#[test]
fn usb_snapshot_cli_reaches_usb_argument_validation() {
    let output = Command::new(env!("CARGO_BIN_EXE_switchbridge"))
        .arg("usb-snapshot")
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("usage: switchbridge usb-snapshot PATH")
    );
}

#[test]
fn help_exposes_usb_commands_alongside_transport() {
    let output = Command::new(env!("CARGO_BIN_EXE_switchbridge"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    for command in ["usb-snapshot", "usb-diff", "send", "receive"] {
        assert!(text.contains(command), "missing {command}");
    }
}
