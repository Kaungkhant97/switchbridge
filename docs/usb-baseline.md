# USB enumeration evidence (LEA-8)

These commands collect USB inventories. They do not establish a live video
endpoint or prove Switch 2 capture. Snapshots preserve profiler inventory and
Mac model/macOS metadata while replacing serial/UUID values, including repeated
values in device names. Review each file before sharing it.

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
