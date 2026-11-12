# LEA-8 USB Baseline Evidence Plan

> **For agentic workers:** Planning only. Preserve the user's chosen Herdr Core parallel ticket owners and separate worktrees; no subagents or implementation under this assignment. Future execution requires the human hardware gate below.

**Goal:** Obtain three attributable disconnected/connected HOME inventory pairs, with privacy-reviewed evidence consumable by LEA-9; conclude enumeration only.

**Architecture:** Reuse the existing macOS inventory collector and text diff. A human controls the stock console and cable; the operator records observations and metadata. Keep HOME baseline and any Copy to PC observations separate; neither implies live video.

**Tech stack:** Existing Rust CLI, macOS `system_profiler`, `sysctl`, `sw_vers`; Markdown evidence notes.

**Spec:** Canonical [LEA-8](https://linear.app/trip123/issue/LEA-8/m1-switch-2-usb-baseline-on-mac-snapshots-and-repeatable-enumeration) and the durable acceptance table below. Local drafting provenance: full frozen contract `/tmp/switchbridge-planning-_cksan7d/LEA-8.json`, read 2026-11-12; future execution does not depend on that temporary file.

## Constraints and ownership

- This planner owns only `docs/superpowers/plans/2026-11-12-lea-8-usb-baseline.md`; no code, workflow, source-worktree, primary-checkout, ticket, PR, commit, push, credential, or device changes.
- Future LEA-8 execution owner produces physical evidence; human owner must confirm Mac + stock Switch 2 availability, a confirmed data-capable cable, console firmware and safe permission to disconnect/reconnect/change console screens. Availability is currently unconfirmed. Parent t9 coordinates these gaps; no direct user questions from this planner.
- Source is PR2 OPEN at `ce8aeac1eab18cd54c494e7b8d8445da061a7e66`, also this planning worktree's observed HEAD. Main lacks this implementation. Do not rebuild it on main or require PR merge to collect evidence from the pinned source.
- No software changes are required. Meaningful TDD applies only if a real future defect or behavior change is separately assigned; no artificial failing tests or repeated full CI for evidence collection.
- Enumeration PASS is never video PASS. Required final label: **USB enumeration only; video UNVERIFIED**.

## Acceptance status and source evidence

| Requirement | Observed status | Remaining proof |
| --- | --- | --- |
| Collector/diff commands, redaction and evidence overwrite protection | Complete in `src/usb.rs`, dispatch in `src/main.rs`; unit coverage plus `tests/usb_cli.rs` | Reuse existing interfaces |
| Software verification | Existing `evidence/resume/review3-coverage.log`: 46 tests pass; 90.98% total line coverage | No new test run performed by this planner |
| Hosted ARM/Intel checks, universal package and native smoke | Complete: run 43141901934, all five jobs success at pinned SHA | No hardware conclusion follows |
| Actual host snapshot/self-diff | Prior `evidence/resume/verification-metadata.json` records host-only smoke | Not controlled console evidence |
| Three HOME disconnected/connected pairs and repeatable console identifiers | Missing; no physical trials observed | Task 2 |
| Actual Mac/OS, cable, firmware, console state, port and timestamps | Missing for these trials | Tasks 1–2; do not copy prior QA machine metadata as experiment metadata |
| Raw redacted inventories, diffs and privacy review | Missing physical artifacts | Task 3 |
| Copy to PC observations / full descriptors | Unobserved; additional LEA-9 context, not a substitute for HOME pairs | Separate state-labelled records or explicit limitation |
| Live gameplay / video endpoint | UNVERIFIED | Outside LEA-8 enumeration acceptance |

Fresh hosted proof (read locally, no remote action): `/Users/kaungkhantthu/Desktop/playground/rustSwitch/.worktrees/resume-integration/evidence/resume/hosted-ci.md` and sibling `hosted-ci.json`; [run](https://github.com/Kaungkhant97/switchbridge/actions/runs/43141901934). Those supersede older metadata's pending hosted/Intel statements. Physical hardware gates remain pending. Historical cloud-only blocker text does not negate observed local macOS software results.

Read references: `docs/usb-baseline.md`; `src/usb.rs` (`run`, `collect_inventory`, `select_usb_type`, `write_inventory`, `inventory_diff`); `tests/usb_cli.rs`; `docs/research/stock-switch2-usb-feasibility.md` (LEA-9 dependency and separately labelled mode requirement). No C4/dataflow research belongs here.

## Review focus

1. A static bus inventory is not an attributable console addition: compare all three controlled reconnects.
2. Unknown identifier labels or embedded short identifiers can evade redaction: human review every shared artifact.
3. Background peripheral changes confound attribution: hold cable/port/peripherals/state constant, record disturbances and rerun affected trials.
4. Empty/error output or a reused path is not a successful snapshot: preserve exit/result records and use fresh names.
5. HOME absence is bounded to that configuration: keep Copy to PC and incomplete descriptor evidence separate, with no inferred protocol/endpoint values.

### Task 1: Prepare one controlled session (hardware gate)

**Files:** Read `docs/usb-baseline.md`, `src/usb.rs`; future create private `.evidence/lea-8/<session>/session.md`. Evidence paths below are proposed deliverables, not existing observations.

**Interfaces:** Consumes pinned PR2 source or a verified executable from that SHA. Produces a session metadata record with no serial numbers.

- [ ] Parent obtains human hardware availability and scoped approval for physical actions. If unavailable, mark BLOCKED with missing equipment/action and stop physical work; do not replace it with synthetic evidence.
- [ ] Use an execution-owned worktree and fresh ignored private `.evidence/lea-8/<session>/` directory, consistent with README evidence policy. Verify `git rev-parse HEAD` equals the pinned SHA; record actual executable path and `shasum -a 256 <actual-executable-path>`. If building is necessary, `cargo build --locked --release`, then use `./target/release/switchbridge`; record command and exit. Do not modify an existing source worktree.
- [ ] Record actual `/usr/sbin/sysctl -n hw.model`, `/usr/bin/sw_vers -productVersion`, `/usr/bin/sw_vers -buildVersion`, and `uname -m` output, source SHA, executable hash and operator/session alias. Capture command, timestamp, exit and stderr for subsequent commands.
- [ ] Human records actual stock console firmware, HOME state, cable make/model and basis for confirmed data capability, exact Mac and console ports, adapters/hubs if any, and fixed peripheral layout. Prefer direct connection; record the actual topology. Never infer data capability from charging alone.

### Task 2: Capture three HOME pairs, then separate mode context

**Files:** Future create private `.evidence/lea-8/<session>/trial-{1,2,3}-{before,after,diff}.txt`, `session.md` and `commands.log`. All attempts, stderr and mode follow-ups stay here until review.

**Interfaces:** `switchbridge usb-snapshot PATH` writes a new redacted inventory; refuses existing files. `switchbridge usb-diff BEFORE AFTER` writes text to stdout; exit 0 means comparison succeeded, not enumeration PASS.

- [ ] Human disconnects console. Record UTC time (`date -u +%Y-%m-%dT%H:%M:%SZ`), observed disconnected state and unrelated USB changes. Set a fresh session directory and actual binary once; example after source build:

```sh
BIN=./target/release/switchbridge
SESSION=.evidence/lea-8/ACTUAL-UNIQUE-SESSION
umask 077
mkdir -p "$SESSION"
chmod 700 "$SESSION"
git check-ignore "$SESSION/session.md"
set -o noclobber
```

Replace the session marker with a real unique session label. Confirm `git check-ignore` prints the private path; stop if it is not ignored. `umask 077` makes subsequent files private to the operator. Execute each following command separately and record its actual exit immediately; do not continue an errored trial as a pass.

```sh
"$BIN" usb-snapshot "$SESSION/trial-1-before.txt"
# Human connects confirmed data cable to the recorded ports; keep stock console at HOME.
# Record connection time, visible state and elapsed wait before snapshot.
"$BIN" usb-snapshot "$SESSION/trial-1-after.txt"
"$BIN" usb-diff "$SESSION/trial-1-before.txt" "$SESSION/trial-1-after.txt" > "$SESSION/trial-1-diff.txt"
```

- [ ] Repeat the same sequence for trial 2 and trial 3, substituting `trial-2` / `trial-3` in all paths and physically disconnecting before each before-snapshot. Keep firmware, cable, ports, state and peripheral topology fixed. Record start/end times and wait interval for each pair; reuse that interval. If enumeration seems delayed or a peripheral changes, preserve the attempt and explain a separately named rerun.
- [ ] Successful snapshot output is `Saved redacted USB inventory. Enumeration and video remain UNVERIFIED.` with exit 0 and nonempty new file. Collector calls `system_profiler -listDataTypes`, selects `SPUSBHostDataType` or `SPUSBDataType`, then uses `-detailLevel full`; Mac model/macOS are embedded. A self-diff or `No USB inventory changes.` is not positive console enumeration evidence.
- [ ] With separate human approval for console navigation, record Copy to PC as a distinct optional follow-up series under `$SESSION/copy-to-pc/`, with its own disconnected control, exact state transition/times and before/after/diff files. Do not relabel HOME trials or mix modes to manufacture three repeatable HOME pairs. If unavailable, state “Copy to PC not tested” for LEA-9. No file-transfer or live-capture operation is required here.

### Task 3: Review, decide and hand off to LEA-9

**Files:** Preserve private originals and every failed/superseded attempt in `.evidence/lea-8/<session>/`. Record private review notes and `transformations.md` there. Only after explicit privacy review, create reviewed/sanitized share copies under `evidence/lea-8/<session>/`: `session.md`, `commands.log`, `trial-{1,2,3}-{before,after,diff}.txt`, `summary.md`, sanitized `transformations.md`, and `SHA256SUMS`; include separately labelled `copy-to-pc/` copies only if reviewed.

**Interfaces:** LEA-9 receives only the explicitly reviewed `evidence/lea-8/<session>/` handoff: session record, three pairs/diffs, observed identifiers by state, command/exit/time log, privacy sign-off, transformation mapping, hashes and descriptor limitations. `.evidence/lea-8/<session>/` is private raw collection (including automatically redacted but unreviewed CLI output), not a default LEA-9 input or shareable deliverable. Any exceptional private-original access requires explicit scoped authorization; do not copy private originals into LEA-9 tracked evidence. No new CLI/API or descriptor collector is proposed.

- [ ] Compare all three HOME diffs side by side. Record only actually observed Vendor ID / Product ID and device context with file references. Attribute additions via disappearance/reappearance across controlled trials, accounting for unrelated devices. Do not insert fixture IDs, guess manufacturer IDs, or infer interface class, protocol, endpoint direction/type, alternate settings or live-video support.
- [ ] Record enumeration PASS only if all three controlled pairs support repeatable console attribution. Otherwise record bounded negative/inconclusive results, missing/failed trials and exact tested firmware/state/topology. Keep acceptance pending rather than turning a stable host inventory into PASS.
- [ ] Review every inventory, diff, log and note for serials, UUID/UDID, device names and other identifying values before any sharing. Existing automatic redaction preserves canonical VID/PID and recognized private labels but is not a complete privacy guarantee; unknown labels and embedded short values require review. Keep raw unredacted profiler output out of shared evidence. Preserve explanatory context if a redaction masks a version or other useful field.
- [ ] After an explicit artifact-by-artifact privacy review, copy only approved/sanitized files into fresh `evidence/lea-8/<session>/`; never collect directly there. Record reviewer/date, each private source-relative filename, reviewed destination-relative filename and transformations (including “unchanged after review”) in private `transformations.md`. Share only a sanitized mapping without removed sensitive values. Preserve private originals; if extra sanitization alters a snapshot, regenerate the corresponding share diff from reviewed snapshots with the existing `usb-diff` command and review it too. Log source and reviewed hashes privately; public hashes cover reviewed artifacts only.
- [ ] Record read-only inventory limitations explicitly: profiler output may not expose full device/configuration/interface/alternate-setting/endpoint descriptors. Mark absent fields “not exposed by this collection”; inventory absence is not endpoint absence. LEA-9 owns descriptor classification and any later authorized live-output experiment.
- [ ] Hash reviewed share artifacts with `shasum -a 256 <each-reviewed-file>` and store output as `SHA256SUMS`; verify with `shasum -a 256 -c SHA256SUMS` from `evidence/lea-8/<session>/` (exclude `SHA256SUMS` itself). Summary includes result, three-pair matrix, actual metadata, command/results, review scope and **USB enumeration only; video UNVERIFIED**. Do not overwrite original attempts.
- [ ] Hand local reviewed paths and gaps to parent/LEA-9. Attaching to Linear, publishing evidence or changing ticket status requires explicit authorization; this plan grants none. A later human wizard may assist execution but is not built here.

## Dependencies and parallel work

Task 1 gates Task 2; all three HOME pairs gate Task 3's positive enumeration decision. Privacy review can begin after each pair; final attribution waits for all three. Copy to PC is separately labelled and never blocks recording an honest HOME result. LEA-9 can continue source research in parallel but cannot classify unobserved interfaces from this plan. LEA-7 host transport, LEA-10 packaging and the documentation lead's C4/dataflow work are independent; no edits to their files are proposed.

Plan self-review: all durable acceptance requirements map to Tasks 1–3; physical availability, human actions, actual identifiers and descriptor visibility remain explicit unknowns. No device operations, builds, tests or remote mutations were performed to draft this plan. Fast-context lookup was attempted but unavailable (configured Windsurf key missing); direct source/docs reads supplied the evidence.
