# Physical acceptance template QA

Review date: 2026-11-12. Reviewed immutable integrated commit `9113cd17ca892f9401a90f2f2fcc5bb4a96b4738` (`9113cd1`), using `git show` without changing checkout. Scope: USB and projector trial templates, CLI/help source and referenced playback/evidence instructions. Static document/source review only: no hardware execution, downloads, remote queries, compilation or device commands.

**Current verdict: PASS after corrected-template re-review below.** The original Medium finding at `9113cd1` is retained as historical evidence and is resolved for the corrected bytes identified below. No fabricated hardware results or command-only physical acceptance were found.

## Finding

**Medium — projector collection lacks the repository's private staging and explicit review boundary.**

References at reviewed commit:

- `docs/projector-trial-record-template.md:24–27` asks for a redacted log/evidence location and excludes identifying details, but supplies no private collection path, artifact review sign-off or reviewed-copy handoff.
- Its synthetic-transfer section follows `docs/macos-playback.md`, which generates `synthetic.h264` and receives `received.h264` in the extracted binary directory. “Fresh scratch directory” in the template does not specify ignored/private storage.
- `README.md:67–69` requires local raw inventories and generated videos in ignored `.evidence/` until reviewed for private identifiers.

Consequence: an operator can follow the projector template literally and collect generated videos, environment logs or supporting photographs in a shareable/nonignored location, without an explicit artifact-by-artifact privacy gate. The instruction to omit serials is useful but does not establish the storage/review transition already made explicit in the USB template.

Requested documentation correction (not applied by this reviewer): require an ignored private `.evidence/lea-10/<trial>/` scratch/collection location, actual path/ignore verification, private logs and optional photos/video; add explicit reviewer/date/scope and sanitized-copy mapping before any `evidence/lea-10/<trial>/` handoff. Preserve private originals and hash only the reviewed share set for handoff. Keep binary/archive baseline verification distinct from hashes of locally produced trial artifacts. No implementation or new collection software is needed.

## Checks that passed

| Area | Source comparison and conclusion |
| --- | --- |
| USB CLI and output | `src/main.rs` HELP/dispatch and `src/usb.rs::run` agree with `usb-snapshot PATH` and `usb-diff BEFORE AFTER` examples and exact success message. Snapshot writes new evidence; diff exit 0 is explicitly not enumeration success. |
| USB collector scope | `collect_inventory` uses macOS profiler type discovery, USB full-detail inventory and model/OS metadata. Template correctly requires separately recorded firmware/cable/ports/time, actual identifiers and three human-controlled HOME pairs; full descriptors/protocol classification are not inferred. |
| USB privacy and identity | Private `.evidence/lea-8/<session>/`, restrictive umask, ignore check, actual executable SHA/source metadata, preserved attempts, transformation mapping and reviewed `evidence/lea-8/<session>/` handoff are explicit. No raw default LEA-9 input. |
| Transport CLI | `src/main.rs` accepts send/receive with numeric loopback endpoint and file path. `src/transport.rs` validates loopback and H.264 Annex B prefix; received output is created new. Template delegates exact commands to matching playback documentation and requires sender/receiver exits plus equal actual hashes. |
| Receiver readiness | `receive_file` emits `Listening on ...` before accepting a connection. Projector template requires readiness and successful receiver completion before playback. No physical result is inferred from those software events. |
| Synthetic format and player | Five-second 640×360/30 fps libx264/yuv420p H.264 generation and `ffplay -fs -autoexit -framerate 30 -f h264 received.h264` exactly match `docs/macos-playback.md`. This review checks consistency, not actual FFmpeg or projector operation. |
| Packaged smoke boundary | `scripts/smoke-packaged-macos.py` checks sender/receiver byte transport using an Annex B sentinel, not decoded imagery. Template correctly keeps packaged smoke, actual synthetic transfer and human moving-image observation separate. |
| Projector artifact identity | Source SHA/run/artifact ID/name/expiry/archive and executable digests agree with integrated `evidence/resume/hosted-ci.md`/`.json`. Template requires actual checksums/mode/slices and complete workflow result, distinguishes replacement artifacts and rejects substituting a local rebuild as hosted bytes. Reference values are explicitly not new trial observations; availability must be rechecked later. |
| Physical acceptance | USB requires three attributable pairs; projector requires a human observing moving received synthetic imagery on the physical mirrored projector. Blank fields, static imagery, wrong display, errors and missing observation cannot silently pass. Player exit, screenshot, decoded output and CI success alone are explicitly insufficient. |
| Claims | Both templates mark themselves unobserved. Synthetic projector success cannot establish USB enumeration or live console capture; video remains UNVERIFIED for LEA-8. No fabricated identifiers, hardware metadata or trial hashes are supplied. |

## Method and limits

Read integrated templates, `docs/macos-playback.md`, `README.md`, `src/main.rs`, USB collection source, transport source and packaged smoke source through `git show 9113cd1:<path>`. Hosted reference identities were compared with the already reviewed local hosted proof; no new availability claim is made. No current runtime help invocation was needed: the exact HELP constant and dispatch at the immutable reviewed commit were inspected directly. No test suite was rerun for this static documentation assessment.

Only this report was created for the QA assignment. Source/template edits are left to their owners. Hardware acceptance remains pending regardless of this review.

## Corrected-template re-review — PASS

Re-review date: 2026-11-12. Inspected the complete actual corrected file and a unified diff against `git show 9113cd1:docs/projector-trial-record-template.md`; owner reports were not treated as proof.

Corrected file: `/Users/kaungkhantthu/Desktop/playground/rustSwitch/.worktrees/resume-cicd/docs/projector-trial-record-template.md`.

Exact corrected SHA256: `00cb8edd66d52b508a5a9e2292772a654bc01f9760209388aad658ecc5370061`. This verdict applies to these file bytes, not to an unspecified future commit.

The original Medium is **RESOLVED**:

- Private collection precedes downloads, metadata and recording; a fresh ignored `.evidence/lea-10/<trial>/` directory, absolute path, per-terminal `umask 077`, owner-only permissions and ignore-result fields are explicit. Any failed gate stops collection.
- Downloads/extraction, generated and received media, raw logs, metadata, draft records and optional recordings remain private. Uncontrolled recording destinations stop that step.
- Actual verified absolute binary paths and private absolute input/output paths override the playback guide's relative examples. Generation/playback remain inside the private trial directory.
- Artifact-by-artifact review covers both visible and embedded identifying information. Reviewer/date/scope, omissions, transformations and final sanitized-copy approval have blank record fields. Originals remain unchanged; share mapping cannot expose private paths.
- Only reviewed sanitized copies enter `evidence/lea-10/<trial>/`; their manifest and verification result are distinct from CI package digests and synthetic transport hashes. Handoff does not authorize publication or ticket updates.
- Diff retains all existing package identity, transfer exits, human moving-image requirements, failure/pending criteria and unobserved-template labels. Adding privacy review does not allow a player exit, screenshot, checksum or software test to claim physical success.

No remaining findings in this review scope. USB template checks from the initial review remain unchanged; no USB/template/source edits were made. This is documentation QA only, not physical acceptance. No hardware, runtime, remote, commit or subagent operations occurred.
