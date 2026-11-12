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

Build `switchbridge`, then use a fresh evidence directory. For each trial 1–3:

1. Disconnect the console from the Mac.
2. Run `switchbridge usb-snapshot evidence/trial-1-before.txt`.
3. Connect a stock Switch 2 at the home screen using a data-capable USB-C cable.
4. Run `switchbridge usb-snapshot evidence/trial-1-after.txt`.
5. Run `switchbridge usb-diff evidence/trial-1-before.txt evidence/trial-1-after.txt > evidence/trial-1-diff.txt`.
6. Disconnect again before the next trial; substitute `trial-2` and `trial-3`.

Record separately: Mac model, macOS version, cable brand/model and confirmed data
capability, Switch 2 firmware, console state, connection port, and time of each
trial. Record unexpected devices or mode changes. Avoid serial numbers in notes.

Compare all three diffs and attribute added vendor/product identifiers to the
console through the controlled reconnects. A stable bus inventory alone does not
count as console enumeration. Keep the raw redacted snapshots with their diffs.

Attach the evidence and commands/results to Linear LEA-8. Leave acceptance
pending until all three controlled trials are observed. Label any successful
result **USB enumeration only; video UNVERIFIED**. An absent device in one mode
is not proof that every mode lacks an endpoint.
