# LEA-8 USB trial record — blank template

**TEMPLATE / NOT OBSERVED.** No hardware collection, enumeration result, identifiers or hashes are supplied by this document. Blank fields mean not recorded, never success. Complete a private copy during a separately authorized physical session.

Spec: [LEA-8](https://linear.app/trip123/issue/LEA-8/m1-switch-2-usb-baseline-on-mac-snapshots-and-repeatable-enumeration). Procedure and acceptance: [reviewed plan](superpowers/plans/2026-11-12-lea-8-usb-baseline.md), [collector reference](usb-baseline.md).

## Session and human gate

- Session label: __________
- Operator alias / date: __________
- Private record: `.evidence/lea-8/ACTUAL-UNIQUE-SESSION/session.md`
- Hardware availability confirmed by / time: __________
- Human authorization for disconnect/reconnect and scope: __________
- Missing equipment, authorization or other blocker: __________
- Human responsible for cable and console operations: __________

If hardware or authorization is unavailable, record BLOCKED and stop. This template itself grants no permission for device actions or publication. Do not include serial numbers in notes.

| Actual metadata | Recorded value / evidence reference |
| --- | --- |
| Mac model (`/usr/sbin/sysctl -n hw.model`) | |
| macOS version (`/usr/bin/sw_vers -productVersion`) | |
| macOS build (`/usr/bin/sw_vers -buildVersion`) | |
| Architecture (`uname -m`) | |
| Source SHA (`git rev-parse HEAD`) | |
| Source matches planned PR2 SHA `ce8aeac1eab18cd54c494e7b8d8445da061a7e66`? | |
| Actual executable path | |
| Executable SHA256 (`shasum -a 256 <actual-executable-path>`) | |
| Build or verified package provenance, command and exit | |
| Stock console firmware / basis for stock state | |
| Cable make/model and evidence of data capability (charging alone insufficient) | |
| Exact Mac port / console port | |
| Adapter/hub topology, or direct connection | |
| Fixed peripheral inventory / known background changes | |
| Chosen connect-to-snapshot wait interval | |
| Profiler data type recorded in actual snapshot | |

Record actual metadata; never copy another QA machine's values as this session's observations.

## Private setup and command log

Replace `ACTUAL-UNIQUE-SESSION` with a fresh unique label. The binary below assumes an existing verified source build; use and record the actual executable. These are future commands, not commands already run by this template author.

```sh
BIN=./target/release/switchbridge
SESSION=.evidence/lea-8/ACTUAL-UNIQUE-SESSION
umask 077
mkdir -p "$SESSION"
chmod 700 "$SESSION"
git check-ignore "$SESSION/session.md"
set -o noclobber
```

- Ignore check output / exit: __________ (stop if the private path is not ignored)
- Private directory permissions verified: __________
- Exact private session path / executable used: __________

Keep all collection, metadata, stderr, logs, review notes and failed attempts under `$SESSION`. Even automatically redacted CLI output is unreviewed private collection. Never collect directly into `evidence/`.

Use `$SESSION/commands.log` to record every command separately, including metadata commands, snapshot and diff. Record its actual exit immediately; later commands must not replace that exit record. Use `date -u +%Y-%m-%dT%H:%M:%SZ` for UTC times.

| UTC start / end | Exact command | Actual exit | stdout file or result | stderr file or result |
| --- | --- | --- | --- | --- |
| | | | | |

## Three controlled HOME pairs

Human sequence for **each** trial: physically disconnect console; record disconnected state/time; capture before; connect the same data cable and ports; keep stock console at HOME; record visible state/time; wait the recorded interval; capture after; generate diff. Disconnect again before the next trial. Keep firmware, cable, ports and peripherals fixed.

Execute individually for trial 1; repeat with `trial-2` and `trial-3` in every filename. Log each command and exit.

```sh
"$BIN" usb-snapshot "$SESSION/trial-1-before.txt"
# Human connects console at HOME and records connection time/state/wait.
"$BIN" usb-snapshot "$SESSION/trial-1-after.txt"
"$BIN" usb-diff "$SESSION/trial-1-before.txt" "$SESSION/trial-1-after.txt" > "$SESSION/trial-1-diff.txt"
```

| Human observation / result | Trial 1 | Trial 2 | Trial 3 |
| --- | --- | --- | --- |
| Attempt label; record path if rerun | | | |
| Disconnected UTC / human confirmation | | | |
| Before snapshot start/end UTC | | | |
| Before exit / nonempty file confirmed | | | |
| Connect UTC / exact ports unchanged? | | | |
| Visible HOME state confirmed by human | | | |
| Wait interval actually used | | | |
| After snapshot start/end UTC | | | |
| After exit / nonempty file confirmed | | | |
| Diff UTC / exit / file | | | |
| Observed added VID/PID and inventory context (with file references only) | | | |
| Unrelated changes / delay / errors | | | |
| Pair valid, negative, inconclusive or invalid; reason | | | |

Expected successful snapshot message: `Saved redacted USB inventory. Enumeration and video remain UNVERIFIED.` Require actual exit 0 and a nonempty new file. A diff exit 0 only means comparison succeeded. `No USB inventory changes.` and a static bus inventory do not establish console enumeration.

Preserve failed or disturbed attempts under new names; do not overwrite or quietly replace them. Record rerun label, cause and connection to original attempt: __________

## Failure and acceptance record

- [ ] All three HOME pairs are complete, with successful commands and attributable reconnect observations.
- [ ] Same console/firmware, state, cable, ports and topology maintained; confounders explained.
- [ ] Repeatable additions attributed to console across all three pairs, with snapshot/diff references.
- [ ] Every shared artifact has passed the privacy gate below.

Result (leave blank until observed; choose enumeration PASS / bounded negative / inconclusive / BLOCKED): __________

Reason and references across all three pairs: __________

Missing/failed pairs, metadata or next controlled test: __________

Do not claim PASS for command errors, empty output, reused evidence paths, missing trials, unrelated peripheral changes, a self-diff or an unattributable stable inventory. Preserve evidence and keep acceptance pending when repeatability is not established. A negative HOME observation applies only to the tested firmware/state/topology.

**USB enumeration only; video UNVERIFIED.** No fixture or guessed identifiers, protocol or endpoint classification belongs in this record.

## Separate Copy to PC context and descriptor limitation

Copy to PC tested? __________ (if not, explicitly write “Copy to PC not tested” in the completed record)

Separate console-navigation authorization / human / time: __________

If authorized and performed, use `$SESSION/copy-to-pc/` for its own disconnected control, before/after/diff, state-transition times and command log. Never mix these observations into the three HOME pairs. No file transfer or live capture is required by this template.

Observed profiler fields / references: __________

Fields not exposed by this collection: __________

Read-only profiler inventories may omit device/configuration/interface/alternate-setting/endpoint descriptors. Missing fields mean “not exposed by this collection,” not endpoint absence. LEA-9 owns descriptor sufficiency and classification; enumeration cannot prove a usable video endpoint.

## Privacy review and reviewed handoff

Private originals and attempt history remain in `.evidence/lea-8/<session>/`. Preserve the collector-redacted originals unchanged. Review every inventory, diff, metadata field, stdout/stderr log and note for serials, UUID/UDID, names and identifying values. Unknown private labels and embedded short values may escape automatic redaction. Do not share raw unredacted profiler output.

- Reviewer alias / UTC review date: __________
- Review scope / files reviewed / exclusions: __________
- Residual limitations and useful fields masked by redaction: __________
- Explicit approval for the listed local share copies: __________

Only after this review, create sanitized copies in `evidence/lea-8/<session>/`. Record full mapping in private `$SESSION/transformations.md`; include only a sanitized mapping in the share set, never removed private values.

| Private source-relative file | Private source SHA256 | Reviewed destination-relative file | Transformation or “unchanged after review” | Reviewed SHA256 / reviewer |
| --- | --- | --- | --- | --- |
| | | | | |

If a snapshot is further sanitized, regenerate its share diff with `usb-diff` from the reviewed before/after copies and review that diff too. Preserve originals and source hashes privately. The table above stays private until its own sanitization review.

Reviewed handoff checklist (no files implied to exist yet):

- [ ] `session.md` and `commands.log`, sanitized actual metadata and command/exit/time records.
- [ ] `trial-{1,2,3}-{before,after,diff}.txt`, reviewed HOME evidence.
- [ ] `summary.md`, three-pair result, observed identifiers with references, limitations and required video label.
- [ ] `transformations.md`, sanitized mapping plus privacy-review scope/date.
- [ ] Separately labelled `copy-to-pc/` evidence if observed and reviewed, or explicit not-tested statement.
- [ ] `SHA256SUMS` for every reviewed share artifact, excluding itself.

Hash reviewed files using `shasum -a 256 <each-reviewed-file>` from the share directory; store results in `SHA256SUMS`. Verify there with `shasum -a 256 -c SHA256SUMS`.

- Exact reviewed share directory: __________
- Hash generation command / UTC / exit: __________
- Hash verification command / UTC / exit / output reference: __________
- LEA-9 handoff date / reviewed manifest / remaining gaps: __________

LEA-9's default input is only the reviewed `evidence/lea-8/<session>/` set. Private `.evidence/` originals are not its default input; exceptional access needs explicit scoped authorization. Local review is not permission to publish, attach to Linear, change tickets or perform remote actions.
