# USB enumeration evidence (LEA-8)

These commands collect USB inventories. They do not establish a live video
endpoint or prove Switch 2 capture. Snapshots preserve profiler inventory and
Mac model/macOS metadata while replacing serial/UUID values, including repeated
values in device names. Review each file before sharing it.

Redaction recognizes the supported Serial Number/Serial and UUID/UDID labels,
plus normalized keys ending in `UUID`/`UDID` or containing `Serial Number`.
Matching is case-insensitive and collapses whitespace. Synthetic regression
fixtures include `Volume UUID`, `Interface UDID`, and qualified Serial Number
labels; these fixtures are not claims of observed hardware output. Ordinary
device headings such as `USB Serial:` are accepted; empty, malformed, or
multiline actual private fields fail closed. Canonical `Vendor ID` and
`Product ID` fields and recognized private-field labels are preserved. Other
colon-terminated headings are redacted as whole lines, including identifiers
containing colons; ordinary value lines retain their structural property names.
Known private values of four or more characters are removed even when embedded
in device names or ordinary values. Values of one to three characters are removed
only as complete tokens, so a serial `0` does not corrupt hexadecimal IDs.
Adjacent letters, digits, `_`, or `-` make a short value part of a longer token.
Diffing uses known private values from both inputs. Unknown field labels, values
absent from private fields in both files, and embedded short values still need
human review before sharing. This collector does not certify that a file is
free of all identifying information. Coincidental matches can also hide unrelated
data, such as a version fragment matching a short serial; inspect other fields
before relying on them as evidence.

On supported macOS versions, `system_profiler` uses either `SPUSBHostDataType`
(newer systems) or `SPUSBDataType`. The collector detects the available type,
rejects empty output, and refuses to overwrite existing evidence.

## Three physical trials

Use the [blank USB trial record](usb-trial-record-template.md) for metadata,
command exits, failed attempts and privacy review. Build `switchbridge` if
needed; run from the repository root in a dedicated shell. Replace the marker
with a fresh unique session label. This setup stops on a failed private-directory
or ignore check; do not continue in another location:

```sh
SESSION=.evidence/lea-8/ACTUAL-UNIQUE-SESSION
umask 077
mkdir -p .evidence/lea-8 || exit 1
mkdir "$SESSION" || exit 1
chmod 700 "$SESSION" || exit 1
git check-ignore -q -- "$SESSION/trial-1-before.txt" || exit 1
set -o noclobber
```

For each trial 1–3, execute each command separately and record its actual exit
immediately in private `$SESSION/commands.log`; stop on any command failure.

1. Human disconnects the console from the Mac and records the time/state.
2. Run `switchbridge usb-snapshot "$SESSION/trial-1-before.txt"`.
3. Human connects a stock Switch 2 at HOME using a confirmed data-capable USB-C cable; record connection time and wait interval before capture.
4. Run `switchbridge usb-snapshot "$SESSION/trial-1-after.txt"`.
5. Run `switchbridge usb-diff "$SESSION/trial-1-before.txt" "$SESSION/trial-1-after.txt" > "$SESSION/trial-1-diff.txt"`. Keep `noclobber` enabled so an existing diff is not replaced.
6. Human disconnects again before the next trial; substitute `trial-2` and `trial-3` in every filename. Keep cable, ports, firmware, HOME state and other peripherals fixed.

Record privately: Mac model, macOS version/build, actual executable path/source
SHA/hash, cable brand/model and confirmed data capability, Switch 2 firmware,
console state, connection ports, UTC times and wait intervals for each trial.
Record unexpected devices or mode changes and preserve failed attempts under
new names. Avoid serial numbers in notes. Copy to PC observations require a
separate state-labelled record; never mix them into the three HOME pairs.

Compare all three diffs and attribute added vendor/product identifiers to the
console through the controlled reconnects. Require successful exits and nonempty
snapshots; a diff exit 0 only means comparison succeeded. A stable bus inventory,
self-diff or missing/failed pair does not count as console enumeration. Do not
invent identifiers or infer endpoint/protocol classifications from absent fields.
Read-only profiler inventory may not expose full descriptors; record that limit.

Preserve original collector-redacted snapshots, diffs, metadata, logs and failed
attempts in ignored `.evidence/lea-8/<session>/`. Automatic redaction does not
replace an explicit artifact-by-artifact privacy review. After that review only,
create sanitized share copies in `evidence/lea-8/<session>/`, record reviewer/date,
review scope and a sanitized transformation mapping, and hash the reviewed files
with `shasum -a 256`; verify their manifest with `shasum -a 256 -c SHA256SUMS`.
Keep private originals and full transformation records unchanged. Regenerate
share diffs from sanitized snapshots if further redaction changes them, and
review those diffs too. Sharing/attaching to Linear LEA-8 requires authorization.

Leave acceptance pending until all three controlled trials are observed and
attributable. Label any successful result **USB enumeration only; video
UNVERIFIED**. An absent device in one mode is not proof that every mode lacks
an endpoint. No live video or physical trial is claimed by these instructions.
