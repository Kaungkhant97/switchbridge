# LEA-10 macOS CI/CD Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans for approved execution. Steps use checkbox (`- [ ]`) syntax. The LEA-10 owner executes natively; no subagents. This document is a planning deliverable, not authorization for new implementation or remote release actions.

**Goal:** Complete original LEA-10 through observed synthetic H.264 projector playback, preserving completed native macOS CI and universal artifact delivery.

**Architecture:** Preserve the existing `check` → `package` → `smoke` workflow: two native check jobs, one universal packaging job and two native extracted-package smoke jobs. Human hardware acceptance consumes the verified downloaded artifact. Optional GitHub Releases are separate, pending user choice and design approval.

**Tech Stack:** Rust 1.96.0, GitHub Actions, macos-15 ARM / macos-15-intel, Bash/lipo/SHA256, Python smoke tooling, separately installed FFmpeg/ffplay.

**Spec:** [LEA-10 — macOS universal binary, CI checks and projector playback](https://linear.app/trip123/issue/LEA-10/m3-macos-universal-binary-ci-checks-and-projector-playback). Purpose: “Deliver reproducible Mac binary for the independent host receiver.” Required evidence: “Link to passing Actions run and downloadable binary artifact.” and “Real Mac playback test with synthetic H.264 and projector.” Known non-goal: “no native Switch 2 capture source yet.” Local draft provenance only: `/tmp/switchbridge-planning-_cksan7d/LEA-10.json`; the canonical ticket and quoted criteria are the durable specification.

## Global Constraints

- Existing CI and packaging are complete; do not replace them with a generic template. Repeatable documented builds exist; byte-for-byte deterministic rebuilds have not been established.
- Preserve unsigned, unnotarized macOS 11+ universal CLI distribution. No service deployment, crates.io publishing, signing credentials or invented secrets.
- Synthetic playback does not establish physical Switch capture, USB enumeration or attributable live frames.
- Preserve other agents/user edits and source worktrees. Current instruction permits revising this plan only: no workflow/code/remote changes, commits, tags, releases or ticket updates in this turn.
- Existing authorization covers maintaining current evidence/PR branch; it does not authorize merging, tagging or releasing.
- Optional release behavior remains the user's choice. Conditional notes below are an option sketch, not an approved implementation plan or selected design.

## Review Focus

1. Expired artifact: Task 1 stops rather than substituting an unverified local build.
2. Wrong binary/source: Task 1 verifies run/artifact identity, checksums and both slices.
3. Player success without projection: Task 2 requires human observation of moving imagery on the physical projector.
4. Missing or private hardware metadata: Task 2 records useful setup details without serial numbers/UUIDs.
5. Scope inflation: Task 3 keeps release choices separate from original acceptance and physical Switch gates.

## Completed CI and artifact evidence

- [x] All five jobs passed in [run 43141901934](https://github.com/Kaungkhant97/switchbridge/actions/runs/43141901934) at source `ce8aeac1eab18cd54c494e7b8d8445da061a7e66`: native ARM/Intel fmt/test/clippy, universal packaging, and native extracted sender/receiver smoke on both architectures.
- [x] [Artifact 13909373980](https://github.com/Kaungkhant97/switchbridge/actions/runs/43141901934/artifacts/13909373980) was downloaded and independently verified. Name: `switchbridge-macos-universal-ce8aeac1eab18cd54c494e7b8d8445da061a7e66`. Expiry: **2026-11-26T02:31:45Z**; availability must be rechecked before hardware acceptance.
- [x] Archive SHA256: `cae0c6daca0e827bccf6f6f4082f3fb515e101f348a026b70f73f341b3ed1c51`. Executable SHA256: `d9bfbae8a694c1c1371f2e1eb2faedbdbc1246b0d058b2aed14a763380b4d94c`. Outer/internal checksums, mode `0755` and both slices passed. Fresh downloaded bytes also passed native ARM TCP smoke and real 85,967-byte synthetic H.264 transport/decode with 60 frames; local Intel execution is not claimed.

Fresh evidence is owned by parent in sibling `../resume-integration/evidence/resume/hosted-ci.md` and `.json`; intended integrated repository paths are `evidence/resume/hosted-ci.md` and `.json`. Do not modify the sibling. The issue's earlier connector-403/CI-trigger blocker is historical; passing hosted CI does not establish connector write access.

## Remaining original LEA-10 work

### Task 1: Prepare the exact hosted executable for observation

**Files:** Read `.github/workflows/macos.yml`, `scripts/package-macos.sh`, `scripts/smoke-packaged-macos.py`, `docs/macos-playback.md` and hosted evidence above. No source/workflow changes needed. Parent coordinates evidence integration and any current-branch documentation maintenance.

**Interfaces:** Consumes run/source/artifact identity above; produces a checksum-verified extracted executable for Task 2.

- [ ] At execution time check artifact expiry. If unavailable, report the evidence gap to parent for a replacement run at the intended source under applicable authorization. Do not silently substitute a local rebuild or claim stale bytes are downloadable.
- [ ] Download into a new scratch directory using `gh run download 43141901934 -R Kaungkhant97/switchbridge -n switchbridge-macos-universal-ce8aeac1eab18cd54c494e7b8d8445da061a7e66` while available.
- [ ] Run `shasum -a 256 -c switchbridge-macos-universal.tar.gz.sha256`, extract into a fresh `unpacked` directory, then run `shasum -a 256 -c SHA256SUMS`, `test -x switchbridge` and `lipo switchbridge -verify_arch arm64 x86_64`. Expected checksums OK and all exits 0; match the recorded hashes.
- [ ] Run existing `python3 scripts/smoke-packaged-macos.py ABSOLUTE_EXTRACTED_BINARY`. Expected loopback SHA256 output and exit 0. This is preparation, not projector evidence.

### Task 2: Human synthetic playback and projector acceptance

**Files:** Follow `docs/macos-playback.md`; proposed result location `evidence/resume/projector-acceptance.md`, created only from actual operator results during authorized evidence maintenance.

**Interfaces:** Consumes Task 1 executable, Mac/projector/adapter/operator and installed FFmpeg/ffplay; produces a redacted observed pass/fail record.

- [ ] Record operator/date/timezone, Mac model/architecture/macOS, FFmpeg/ffplay versions, projector model, connection/adapter, resolution/refresh and mirror configuration; include source SHA/run/artifact ID and archive/executable hashes. Exclude personal serials/UUIDs.
- [ ] Generate the documented five-second `testsrc2` clip (`640x360`, 30 fps, libx264, yuv420p, Annex B). Start extracted `./switchbridge receive 127.0.0.1:7878 received.h264`, wait for Listening, then send `synthetic.h264` using the extracted binary. Record sender/receiver exits 0 and equal input/output SHA256. Use fresh output paths; the receiver refuses existing files.
- [ ] Run `ffplay -fs -autoexit -framerate 30 -f h264 received.h264`, mirror the Mac to the connected projector and have the operator observe moving synthetic imagery on the physical projector. Record actual fullscreen/mirroring observation and player exit/status. If q ends playback early, record that explicitly.
- [ ] Record failure/pending if disconnected, wrong display, image absent/static or no operator observation. A successful player process, decode log or screenshot alone does not pass the criterion.
- [ ] Review the redacted record and link observed results with hosted evidence in the existing docs/PR context under parent coordination. Do not update the ticket or perform remote actions in this planning phase. Keep Switch capture, three controlled reconnects and attributable live frames pending independently.

No new software tests are needed for a hardware-only observation. Task 1's existing checks and Task 2's physical observation are the acceptance tests. If a procedure/code defect is discovered, return to parent with the exact failure before expanding scope.

## Optional CD extension: unapproved option sketch

### Task 3: Obtain release choice before designing implementation

**Files:** Reference `evidence/resume/cicd-assessment.md`. No new design, helper, dependency or workflow is authorized by this sketch.

- [ ] Parent obtains the user's choice: **draft releases on version tags (recommended)** for human publication review; **automatic public releases** for immediate distribution on successful valid tags; or **artifacts-only** preserving current 14-day downloads and no release write permission.
- [ ] If artifacts-only is selected, retain existing delivery and document expiry. Original Task 2 still determines remaining LEA-10 acceptance.
- [ ] If releases are selected, agree on version/tag/prerelease policy, allowed source history, asset names, rerun/recovery behavior and provenance needs; then write a short concrete design for user approval before a new implementation plan. No choice is assumed here.

Proposed boundaries for that future design:

- Reuse `.github/workflows/macos.yml` and existing package/smoke jobs. A final tag-only release job depends on both successful native smoke jobs and promotes the exact same-run SHA-named artifact, with **no rebuild** or latest/cross-run artifact lookup. Actions artifacts are uploaded before smoke today; only an entirely passing run is suitable for delivery approval.
- Validate tag/Cargo version equality and tagged commit ancestry on main before release mutation. Stable `vMAJOR.MINOR.PATCH` is a proposal, not approved policy. Reject invalid/mismatched/untrusted inputs and pass refs as data. Exact validator interface/parser/dependencies remain for approved design.
- Keep global `contents: read`; any `contents: write` belongs only to the guarded release job. No privileged PR trigger, PAT, signing secret or invented credential. Main ancestry is not proof of branch protection/reviewer approval.
- Reverify checksums and exact archive/sidecar asset set. Existing filenames preserve the checksum's reference; versioned renaming is a separate decision. Record source SHA, tag, run URL and digests as traceability, not cryptographic attestation.
- Failed checks or either smoke must block release delivery. Serialize per tag, avoid canceling an active delivery, fail closed on existing releases, and document deliberate recovery rather than clobbering published assets.
- Any future implementation needs focused negative tests for bad tag/version/ancestry, failed smoke, wrong/missing artifact, checksum mismatch, existing-release rerun and non-tag/PR permission isolation; preserve actionlint and current native validation. Mock success must not be presented as live release proof. Real tag/release tests need separate authorization.
- Likely scope is `.github/workflows/macos.yml`, `docs/macos-playback.md` and focused CI/CD tests/docs; helper changes only if approved design requires them. No package/source rewrite is proposed.

## Completion and review

Original LEA-10 closes only with both the already-satisfied hosted run/download evidence and a real synthetic projector observation. Optional release delivery is reported separately and cannot substitute for that observation. LEA-8 physical reconnect and LEA-9 live-capture gates do not depend on adopting releases.

Self-review: canonical quoted criteria map to completed CI evidence and Tasks 1–2; exact artifact identity/expiry and physical metadata/failure conditions are explicit. Task 3 is a proposal gate, not implementation approval. No parser, dependency, release choice, signing identity, physical observation or remote write authorization has been invented. Plan is ready for parent/user review; implementation remains paused as instructed.
