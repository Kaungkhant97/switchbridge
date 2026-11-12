# LEA-9 Feasibility Gate Plan

> **For agentic workers:** Execute only separately authorized tasks through the user-selected Herdr parallel owners, one owner per ticket and isolated worktree. This handoff authorizes planning only; no subagents or execution.

**Goal:** Preserve the completed, honest BLOCKED feasibility decision and define the evidence required to reopen a direct USB live-gameplay gate.

**Architecture:** LEA-8 supplies physical inventory and descriptor evidence. LEA-9 classifies only observed interfaces, then requires an auditable USB-bytes-to-decoded-gameplay chain before PASS. Independent host transport remains usable while console feasibility is blocked.

**Tech Stack:** Existing Rust CLI, macOS USB inventories, documentary evidence; a decoder only after the observed payload format is established.

**Spec:** [Canonical LEA-9 Linear contract](https://linear.app/trip123/issue/LEA-9/m2-investigate-stock-switch-2-live-stream-feasibility-stopgo-decision) and `docs/research/stock-switch2-usb-feasibility.md`. Durable acceptance quotes from the contract:

> PASS only if we can record real live game frames and attribute them to console USB traffic, with repeatable steps.
>
> Otherwise write a documented **NO-GO / BLOCKED** decision and narrow feasible alternatives.

The frozen `/tmp/switchbridge-planning-_cksan7d/LEA-9.json` was read in full as drafting provenance only; future reviewers can use the canonical contract and acceptance quotes above without that temporary file.

## Global constraints and ownership

- Stock Switch 2 → macOS, without dock/capture card or console modification is the question being gated.
- PASS requires recorded real live gameplay frames attributable to console USB traffic with repeatable steps. The contract explicitly accepts documented NO-GO / BLOCKED plus narrow alternatives.
- Current owner t22 writes only this plan. Preserve other agents/user edits; no primary/source-worktree edits, implementation, commits, pushes, ticket/PR changes, device activity or remote mutations.
- Future research owner updates the existing feasibility report and an explicitly approved evidence location. LEA-8 owns baseline collection; documentation lead owns C4/dataflow. Do not duplicate their work.
- Human hardware access, consent for any future read-only experiment and capture tooling permissions are prerequisites. No console writes/modification, undocumented activation commands or arbitrary endpoint probing. Permission failures go to parent t9.

## Observed acceptance and source state

| Requirement | Completed evidence | Missing / decision |
|---|---|---|
| Documented fallback gate | Report records BLOCKED/UNVERIFIED and NO-GO for committing to a direct USB capture product; PR1 merged (`460e8e8`, report commit `efa25fb`) | Documentary deliverable exists; authorized human may reconcile ticket closeout separately, with no claim of hardware PASS |
| Interface/descriptor classification from M1 | Report lists exact evidence gaps; `docs/usb-baseline.md` defines collection | LEA-8 three physical reconnects and full enough descriptors; inventory alone cannot classify every interface |
| Distinguish saved transfer, network sharing and camera | Report S1–S4 distinguish MTP saved media, Internet GameChat, camera input and dock HDMI | No PTP label without protocol evidence; none establishes USB live output |
| Reproducible real live gameplay | Report defines attributable repeated capture and controls | No console descriptors, traffic or decoded gameplay received; PASS remains unverified |
| Feasible alternatives | Existing report narrows saved MTP media, dock display, proposed HDMI capture and host-only work | Compatibility/latency/physical results remain separate experiments |

Source PR2 is OPEN, not main: `ce8aeac1eab18cd54c494e7b8d8445da061a7e66`. This planning worktree contains that source; primary main lacks its implementation. `evidence/resume/verification-metadata.json` records 46 passing tests and 90.98% source line coverage, plus a byte-identical 60-frame synthetic H.264 round trip. Its old pending hosted/Intel entries are superseded by fresh `hosted-ci.md` and `hosted-ci.json` at `/Users/kaungkhantthu/Desktop/playground/rustSwitch/.worktrees/resume-integration/evidence/resume/`: [run 43141901934](https://github.com/Kaungkhant97/switchbridge/actions/runs/43141901934) passed all five ARM/Intel checks, universal packaging and native packaged smoke jobs at the exact PR2 SHA. These are observed software results, not console capture or projector evidence. No rebuild is needed for this documentation plan.

## Review focus

- Saved clips resembling gameplay: require contemporaneous scripted changes and source provenance (Task 3).
- A newly enumerated device without descriptors: retain unclassified/BLOCKED (Task 2).
- Idle screens still emitting frames: test absence of scripted changes, not absence of bytes (Task 3).
- Decoder input silently substituted from synthetic, album or network sources: retain payload hashes and isolated source chain (Task 3).
- Negative results from one state/firmware generalized to impossibility: bound conclusions to tested conditions (Tasks 2–3).

## Task 1: Review completed report for documentary closeout

**Files:** Read existing `docs/research/stock-switch2-usb-feasibility.md`, `evidence/lea-9/claude-review-round1.json`, `evidence/lea-9/claude-review-round2.json`; no code changes.
**Interface:** Produces a closeout recommendation for parent/human: `decision=BLOCKED`, `product_commitment=NO-GO`, `live_capture=UNVERIFIED`, with PR1/report references and unresolved physical evidence list.

- [ ] Compare the frozen contract's fallback acceptance with the merged report and the table above; confirm that missing hardware does not invalidate its explicitly permitted blocked deliverable.
- [ ] Hand parent the existing report/PR1 reference and recommend separate documentary closeout review. Do not update ticket state or reopen implementation. Further product-risk research is optional future work, not a prerequisite invented for this completed report.

## Task 2: Consume LEA-8 evidence and classify observed interfaces

**Files:** Read `docs/usb-baseline.md`, `src/main.rs`, `src/usb.rs`, `tests/usb_cli.rs`; future report update only in `docs/research/stock-switch2-usb-feasibility.md`, with reviewed evidence paths supplied by LEA-8. Producer plan: `../plan-lea8/docs/superpowers/plans/2026-11-12-lea-8-usb-baseline.md` (sibling worktree reference).
**Interface:** Consumes only privacy-reviewed share artifacts under `evidence/lea-8/<session>/`: three redacted before/after/diff sets plus Mac/macOS, firmware, state, cable/port, timestamps, tool/source version and human observation. Produces a state-specific interface table with reviewed artifact references, device/configuration/interface/alternate-setting/endpoint fields when observed, transfer classification and explicit unknowns. Original collection inputs and command logs remain private in ignored `.evidence/lea-8/<session>/`; automatic collector redaction alone does not authorize sharing. Reviewed handoff includes `session.md`, `commands.log`, the three trial sets, `summary.md`, sanitized `transformations.md` and verified `SHA256SUMS`, with explicit privacy sign-off. Full source-hash/transformation mapping and failed/superseded attempts stay private; exceptional private-original access requires separate scoped authorization. If sanitization changes snapshots, consume their regenerated and reviewed share diffs.

- [ ] After hardware authorization, LEA-8 reuses existing commands for trials 1–3 (not executed by this planning task): `switchbridge usb-snapshot .evidence/lea-8/ACTUAL-UNIQUE-SESSION/trial-1-before.txt`, `switchbridge usb-snapshot .evidence/lea-8/ACTUAL-UNIQUE-SESSION/trial-1-after.txt`, and `switchbridge usb-diff .evidence/lea-8/ACTUAL-UNIQUE-SESSION/trial-1-before.txt .evidence/lea-8/ACTUAL-UNIQUE-SESSION/trial-1-after.txt > .evidence/lea-8/ACTUAL-UNIQUE-SESSION/trial-1-diff.txt`. LEA-8 first uses `umask 077`, verifies the private directory is ignored with `git check-ignore`, and uses a fresh session; repeat with trial-2/trial-3. These are private collection paths, not publication paths. Only after privacy review does LEA-8 create separately reviewed share copies in `evidence/lea-8/ACTUAL-UNIQUE-SESSION/`, retaining the private originals. Expected evidence is controlled added identifiers repeatable across reconnects; not a predetermined VID/PID. Separate HOME and Nintendo Copy to PC states.
- [ ] Check the privacy-reviewed snapshot copies, review record and collection limitations; private originals are not the shared handoff. Verify reviewed handoff hashes with `shasum -a 256 -c SHA256SUMS` from its share directory. The CLI wraps profiler inventory; it is not a full descriptor dumper or live endpoint reader. Have the hardware owner select and document an available authorized read-only descriptor tool and its exact version/command before running it. If none exposes sufficient descriptors, record BLOCKED rather than invent a command or values.
- [ ] Classify only supported observations; keep saved album transfer distinct from live output. Record no-device, incomplete-descriptor and permission outcomes by tested state/firmware. Publish no identifiers until human redaction review.

## Task 3: Conditional live-output experiment and decision

**Files:** Future update to the existing feasibility report; a human-approved evidence directory under `evidence/lea-9/` may hold session manifests, redacted traffic, payload, decoder logs and frame hashes. Do not create those artifacts during planning.
**Interface:** Requires Task 2 plus an observed authorized output candidate and documented activation/read method. Produces either auditable PASS evidence or bounded BLOCKED/NO-GO with next falsifiable test. Existing `send`/`receive` CLI accepts prerecorded H.264 Annex B on loopback only; it is not a USB capture interface.

- [ ] Before acquisition, hardware owner records exact tool commands, supported endpoint/read procedure, observed payload format, decoder version/options, monotonic and wall-clock correlation, and capture topology. Parent/human must approve any added tooling or changed scope. If the method is unknown or requires console writes, stop at BLOCKED; do not guess packet operations.
- [ ] In three independent reconnect sessions, log deliberately chosen visible gameplay changes and their time windows. Retain timestamped USB endpoint traffic, extracted received bytes, extraction method, decoder input hash, output/frame hashes and synchronized observations. Use `shasum -a 256 PATH` for each retained file; only select a decoder command once the actual format is known. Successful decoding alone is insufficient.
- [ ] Repeat disconnected and connected-idle controls with the same acquisition/decoder setup, clear prior buffers and isolate saved-media/synthetic/network inputs. Disconnected runs must yield no matching frames; idle runs must not reproduce the scripted changes. Record network state and source exclusions; do not treat GameChat as USB proof.
- [ ] Review the traffic → payload → decoder → contemporaneous gameplay chain across all sessions and controls. PASS only if it survives attribution review. Otherwise preserve failure/limitation details and tested conditions, maintain BLOCKED/UNVERIFIED and product NO-GO, and name the next observation that could change the result. No assertion of impossibility.

## Parallel work and human gates

Task 1 can finish now independently of hardware. LEA-8 baseline work can proceed alongside LEA-7 host transport and LEA-10 packaging/playback; those owners reuse completed PR2 software rather than rebuild it. Task 2 awaits LEA-8 physical evidence; Task 3 awaits Task 2 and an authorized candidate. C4/dataflow remains with the documentation lead. If constraints are relaxed, the human product owner chooses a separate scoped investigation: offline MTP media (S1), dock/HDMI display (S4), or proposed dock/HDMI Mac capture with independent compatibility/protection/latency validation. GameChat network sharing and USB camera input (S2/S3) remain distinct paths, not substitutes for PASS.

## Verification and references

Planning checks: inspect only this file's diff and `git status --short`; check every contract bullet against the acceptance table/tasks. No artificial failing tests for documentary changes. Any later behavior change requires a separate scoped TDD plan and meaningful failing regression, then relevant Rust tests; existing completed tests are not new hardware evidence.

Reuse the report's already-inspected Nintendo primary citations S1 (USB MTP transfer), S2 (GameChat), S3 (USB camera), S4 (dock/HDMI); this plan adds no external factual claim requiring new browsing. Supporting implementation/docs: `src/main.rs`, `src/usb.rs`, `docs/usb-baseline.md`, `docs/macos-playback.md`. Historical review artifacts establish their own reviewed snapshots only. Fast-context search was attempted but unavailable (missing Windsurf API key); direct file inspection completed without accessing credentials.
