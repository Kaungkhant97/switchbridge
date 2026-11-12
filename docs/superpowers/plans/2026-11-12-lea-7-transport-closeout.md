# LEA-7 Transport Closeout Implementation Plan

> **For agentic workers:** Execute the checkboxes through the user-selected Herdr Core parallel ticket owners. This is a planning-only handoff: no subagents, implementation, commits, pushes, PR/Linear changes, or device operations are authorized here. Do not invoke a different execution workflow.

**Goal:** Close acceptance and integration of the already implemented Mac-side prerecorded SWB1 transport, with a precise **PASS — host prerecorded transport only** verdict.

**Architecture:** Retain the bounded SWB1 packet reader/writer, numeric-loopback TCP CLI, and Annex B byte-chunk sender/receiver. Reuse existing tests and attributable source/artifact evidence; do not redesign transport or duplicate C4/dataflow research owned by the documentation lead.

**Tech Stack:** Rust 1.96.0, standard-library I/O/TCP, macOS ARM/Intel; FFmpeg/ffprobe and SHA256 for synthetic media verification.

**Durable spec:** [LEA-7](https://linear.app/trip123/issue/LEA-7/m0-rust-swb1-local-transport-framing-sender-receiver-and-unit-tests), with its acceptance contract reproduced in the self-contained table below. Frozen `/tmp/switchbridge-planning-_cksan7d/LEA-7.json` (updated 2026-11-12T02:30:35.260Z) is local draft provenance only; future execution does not depend on that temporary file.

## Global constraints

- Input is prerecorded H.264 Annex B; timestamps are **not** frame timing and provide no proof of Switch 2 capture.
- Source is OPEN [PR2](https://github.com/Kaungkhant97/switchbridge/pull/2), `ce8aeac1eab18cd54c494e7b8d8445da061a7e66`; primary main lacks this implementation. Never treat PR2 as merged.
- Planning owner t20 owns only this file in `.worktrees/plan-lea7`; preserve other owners, primary checkout and original source worktrees.
- No physical dependency for LEA-7. Console enumeration, live capture and projector observations remain separate LEA-8/9/10 gates.
- No added ACK, stream digest, end marker, full decoder, capture source, or frame scheduler. No artificial failing tests for completed software.

## Acceptance and current gaps

| Frozen requirement / closeout gate | Observed state | Exact evidence / remaining action |
| --- | --- | --- |
| Strict length-prefixed SWB1 validation | Complete | `src/framing.rs`; 24-byte big-endian header, magic SWB1, version/kind 1, zero flags; payload 1–1,048,576 bytes validated before allocation |
| Localhost sender/receiver, prerecorded Annex B | Complete | `src/transport.rs`, `src/main.rs`; numeric loopback only, nonzero port, 64 KiB sender chunks, three/four-byte Annex B prefix check |
| Round trip, clean EOF, malformed/truncated input, limits | Complete | Seven `tests/framing.rs`, six `tests/transport.rs`, three `tests/network.rs`, two `tests/cli.rs` tests; existing-output protection and original-error-preserving cleanup included |
| `cargo test --all-targets` on macOS | PASS | Locked equivalent passes: 46 total integrated tests (18 transport/framing/network/CLI plus 28 USB), local `evidence/resume/release-qa.log`; hosted ARM and Intel check jobs pass |
| FFmpeg input through TCP, byte-identical SHA256 | PASS | Hosted artifact validation: 85,967 bytes, input/output `487f583802823c66ba464d1a94067e7a77dd0abe037ec14be473dc8efd14b1b2`; successful decode, 160×90, 60 frames |
| Chunk timing and capture disclaimer | Complete | README SWB1 section, `Packet.timestamp_us` comment and CLI HELP; `help_describes_synthetic_transport_and_chunk_timing` pins the copy |
| Local quality threshold | PASS | `release-qa.log`: 90.98% overall source lines; framing 93.18%, main 90.38%, transport 91.67%; fmt/clippy pass |
| Hosted/native Intel proof | PASS, supersedes stale pending prose | Run 43141901934 at exact PR2 SHA: ARM/Intel checks, universal package, both native packaged smoke jobs all success |
| Durable evidence and accurate status text | Locally reconciled; publication pending | Parent reports hosted-ci.md/json preserved and audit/index/PR draft reconciled locally in integration worktree; those edits are not pushed and are absent from frozen ce8aeac PR contents. Publish under existing evidence-branch authorization; ticket edits remain gated |
| Main integration / ticket completion | Pending approval | PR2 OPEN; no merge or Linear mutation authorized by this plan |

## Existing files and interfaces (retain)

| File | Contract and proof |
| --- | --- |
| `src/framing.rs` | `Packet { sequence: u32, timestamp_us: u64, payload: Vec<u8> }`; `MAX_PAYLOAD: usize = 1024 * 1024`; `read_packet(reader: &mut impl Read) -> io::Result<Option<Packet>>`; `write_packet(writer: &mut impl Write, packet: &Packet) -> io::Result<()>` |
| `src/transport.rs` | `validate_endpoint(address: &str) -> io::Result<SocketAddr>`; `send(reader: &mut impl Read, writer: &mut impl Write) -> io::Result<()>`; `receive(reader: &mut impl Read, writer: &mut impl Write) -> io::Result<()>`; receiver requires sequence zero then consecutive checked increments |
| `src/main.rs` | CLI `switchbridge send IP:PORT INPUT.h264` / `receive IP:PORT OUTPUT.h264`; private `receive_file(listener: TcpListener, path: &Path) -> io::Result<()>`; one connection, create-new output, ten-second connected I/O timeout, partial-file cleanup on handled errors |
| `tests/framing.rs` | Independent wire fixture, every truncated nonempty prefix, header fields, size bounds/max round trip, propagated I/O errors |
| `tests/transport.rs` | Multi-chunk equality, three-byte start code, invalid input, empty stream, wrong/duplicate/skipped sequence, invalid first packet, numeric endpoint restrictions |
| `tests/network.rs`, `tests/cli.rs` | Real TCP equality/no overwrite, malformed cleanup, original protocol error surviving cleanup failure, invalid command/endpoint and disclaimer checks |
| `README.md`, `evidence/resume/*` | Existing wire/usage and evidence references; future edits only by explicitly assigned integration/docs owners |

## Review focus and limitations

No new reproduced transport defect was found in this read-only planning pass; this is not a fresh independent runtime certification. Existing tests cover the frozen acceptance inputs. Preserve these documented limits rather than adding unrequested behavior:

- Clean EOF between packets cannot detect omitted complete trailing packets. Repro if disputed: serialize valid sequences 0 and 1, remove the entire second packet, then receive; the remaining valid prefix can succeed. External input/output hashes detect the experiment's mismatch; SWB1 has no end marker/digest.
- Sender success is not receiver acknowledgement or proof of durable output. No ACK protocol exists; inspect both process results and hashes.
- Annex B prefix checking does not establish valid/playable H.264. Use the existing real FFmpeg decode proof for the synthetic fixture.
- Receiver accept waits until connection/interruption; ten-second timeouts apply to connected socket I/O. Process termination may leave a partial file; handled failures clean up, and existing output is never overwritten.
- Chunk elapsed timestamps are not presentation times. A decoded 60-frame synthetic clip does not demonstrate console capture or physical display output.

If an actual acceptance regression appears, report its exact source SHA, minimal bytes/CLI invocation, expected versus observed result, process errors and affected existing test to parent. Obtain a scoped behavior-fix assignment before changing code; then add a meaningful failing regression test, implement minimally and rerun the relevant plus canonical suites. Do not patch during planning.

## Task 1: Freeze acceptance evidence (transport owner; parallelizable)

**Files:** Read the source/tests above, `evidence/resume/release-qa.log`, `verification-metadata.json`, `codex-final.json`, `claude-final.json`; no source changes.

**Consumes:** Durable ticket URL and acceptance table above, PR2 SHA, recorded QA and source hashes. **Produces:** Acceptance verdict with source/evidence attribution.

- [ ] Confirm the intended integration head still equals the frozen SHA with `git rev-parse HEAD`; if it changed, identify the diff and reassess affected acceptance before reusing results.
- [ ] Reuse recorded canonical `cargo test --locked --all-targets`, `cargo fmt --check`, `cargo clippy --locked --all-targets -- -D warnings`, and `cargo llvm-cov --locked --all-targets --fail-under-lines 80 --summary-only` results: 46 pass, no lint diagnostics, 90.98% lines. No rerun needed solely to plan closeout. If future source changes invalidate these results, run in that owner's isolated worktree; focused command is `cargo test --locked --test framing --test transport --test network --test cli` (18 tests), followed by the canonical suite.
- [ ] Record that this planning pass independently matched SHA256 for all seven owned source/test files against `verification-metadata.json`; original local QA source `d927173a9c76a381e2141b5ab6285f663653632c` is not mislabeled as a newly executed PR2 test run. Hosted run separately tests the exact PR2 SHA.

## Task 2: Preserve hosted proof and reconcile stale evidence (integration/docs owners; parallelizable with Task 1)

**Files:** Read sibling `resume-integration/evidence/resume/hosted-ci.md` and `hosted-ci.json`; future authorized destinations `evidence/resume/hosted-ci.md`, `hosted-ci.json` and existing evidence index/README status prose. Packaging owner retains package/workflow scope; documentation lead retains C4/dataflow.

**Consumes:** [Run 43141901934](https://github.com/Kaungkhant97/switchbridge/actions/runs/43141901934), exact source SHA and artifact identity. **Produces:** Durable, attributable evidence references, without repeating hardware claims.

- [ ] Reuse the parent-reported local preservation of both hosted reports and reconciliation of audit/index/PR draft. Confirm those edits before publication; they are not yet pushed and must not be attributed to ce8aeac. Maintenance/publication of the same reviewed evidence branch uses the existing authorization for the prepared push/open-PR/CI proposal, which was already executed; do not request that authorization again. This planning agent edits only its assigned plan. Retain historical logs with their original scope.
- [ ] Retain all five success conclusions: Check (macos-15), Check (macos-15-intel), Universal executable, Packaged smoke (macos-15-intel), Packaged smoke (macos-15). Intel smoke architecture derives from runner/image and matching check-job toolchain; it does not contain a smoke `uname` measurement.
- [ ] Retain [artifact 13909373980](https://github.com/Kaungkhant97/switchbridge/actions/runs/43141901934/artifacts/13909373980), expiry `2026-11-26T02:31:45Z`, ZIP SHA256 `dd2619f27c52750f6c9f62e97b40c42d556739d9aa44c276d3316c3b17e442e5`, tar SHA256 `cae0c6daca0e827bccf6f6f4082f3fb515e101f348a026b70f73f341b3ed1c51`, executable SHA256 `d9bfbae8a694c1c1371f2e1eb2faedbdbc1246b0d058b2aed14a763380b4d94c`. Preserve hashes separately from the older local build manifest.
- [ ] Cite the existing downloaded-artifact FFmpeg validation and matching media hashes in the acceptance table. Optional future reproduction after a changed build: `bash scripts/package-macos.sh`, then `python3 evidence/resume/verify-synthetic.py` and `python3 -O evidence/resume/verify-synthetic.py`; expected byte-identical hashes, 85,967 bytes for the recorded tool versions and 60 decoded frames. FFmpeg/tool changes may change the fixture hash; equality, valid decode and expected geometry/frame count are the acceptance properties. No need to rebuild the passing artifact for this plan.

## Task 3: Approval-gated integration and LEA-7 closeout (parent/human; depends on Tasks 1–2)

**Files/resources:** PR2, existing evidence index and LEA-7; no changes authorized now.

**Consumes:** Reviewed evidence bundle and retained limitations. **Produces:** Approved integration and ticket disposition, separately from hardware tickets.

- [ ] Parent presents the concrete acceptance/evidence bundle and PR2 state for approval of the actual merge only. Continuing maintenance of the same reviewed evidence branch is already authorized. Existing final Codex/Claude reviews apply to their recorded snapshots; do not invent a new review of later edits.
- [ ] Only after explicit authorization, the integration owner performs the agreed PR2 merge workflow and verifies integrated source/check results. A changed source or merge conflict requires affected tests/review; the original PR-head result alone is not evidence of a different merged tree.
- [ ] Only after explicit Linear-change authorization, replace LEA-7's stale compile/runtime UNVERIFIED status with the scoped PASS, source/run/artifact links, matching synthetic hashes and timing/capture disclaimer, and move to the approved completion state. No LEA-8/9/10 hardware claims or dependency on their physical acceptance.

## Planning provenance / handoff

Read full frozen contract, actual source/tests, README/workflow/helper, local QA/metadata/final review reports and both supplied hosted reports. No tests/builds/device operations or remote queries were run by this planning agent. Fast-context was attempted first and failed because its Windsurf API key is unavailable; direct file inspection and Python hash/JSON checks succeeded without changing credentials. Herdr identity verified via installed `hc --skill` and `hc get "$HC_TERMINAL_ID"`: t20, parent t9.

**READY dependencies:** Integration owner publishes the locally reconciled evidence under existing branch-maintenance authorization; explicit human authorization remains required for actual merge and Linear edits. No implementation or physical-device blocker. Self-review checked all frozen acceptance clauses, interfaces and task dependencies; no additional transport feature is needed.
