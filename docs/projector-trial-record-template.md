# LEA-10 projector trial record template

**BLANK TEMPLATE — NOT OBSERVED. No hardware acceptance is claimed.**

Copy this record for an actual human trial; replace blank fields with observed
results and retain failed checks. Follow [macOS playback](macos-playback.md).
[LEA-10](https://linear.app/trip123/issue/LEA-10/m3-macos-universal-binary-ci-checks-and-projector-playback)
requires “Link to passing Actions run and downloadable binary artifact.” and
“Real Mac playback test with synthetic H.264 and projector.” A player exit or
screenshot alone cannot establish physical mirroring or moving-image acceptance.

## Trial identity and setup

| Field | Actual value (leave blank until observed) |
| --- | --- |
| Trial ID / operator | |
| Start / end date and time, timezone | |
| Mac model / architecture / macOS version | |
| Projector model | |
| Cable, port, adapter model and connection path | |
| Mirroring configuration / selected display | |
| Mac and projector resolution / refresh rate | |
| FFmpeg version / ffplay version | |
| Redacted command log or evidence location | |

Exclude serial numbers, UUIDs and personal device/account details. Record model
and setup information sufficient to reproduce the test.

## Verified CI baseline (reference values, not this trial's results)

All five hosted jobs passed at the source below: native ARM/Intel checks,
universal packaging and extracted native smoke on both. Fresh hosted evidence
is recorded in `evidence/resume/hosted-ci.md` and `.json` when integrated.

| Baseline field | Verified reference |
| --- | --- |
| Source SHA | `ce8aeac1eab18cd54c494e7b8d8445da061a7e66` |
| Run | [43141901934](https://github.com/Kaungkhant97/switchbridge/actions/runs/43141901934) |
| Artifact | [13909373980](https://github.com/Kaungkhant97/switchbridge/actions/runs/43141901934/artifacts/13909373980) |
| Artifact name | `switchbridge-macos-universal-ce8aeac1eab18cd54c494e7b8d8445da061a7e66` |
| Recorded expiry | `2026-11-26T02:31:45Z` |
| Archive SHA256 | `cae0c6daca0e827bccf6f6f4082f3fb515e101f348a026b70f73f341b3ed1c51` |
| Executable SHA256 | `d9bfbae8a694c1c1371f2e1eb2faedbdbc1246b0d058b2aed14a763380b4d94c` |
| Archive / sidecar | `switchbridge-macos-universal.tar.gz` / `switchbridge-macos-universal.tar.gz.sha256` |

Recheck availability before downloading: artifacts expire or may be deleted.
If unavailable, report the gap to the coordinator for an authorized replacement
run. Do not substitute a local rebuild and label it these hosted bytes. If a
replacement is used, preserve this baseline and record its distinct run, source,
artifact, expiry and digests below; verify its entire workflow passed.

| Preparation check | Actual result / timestamp / exit status |
| --- | --- |
| Availability checked at / actual expiry | |
| Actual run URL / complete workflow conclusion | |
| Actual source SHA / artifact ID and name | |
| Download location / time | |
| Outer checksum check / measured archive SHA256 | |
| Internal SHA256SUMS check / measured executable SHA256 | |
| Executable path / executable mode / both architecture slices | |
| Existing packaged TCP smoke result / exit | |
| Matches baseline, or explicit replacement provenance | |

Use a fresh scratch directory. Verify the outer sidecar before extraction,
internal `SHA256SUMS` after extraction, `test -x switchbridge`, and
`lipo switchbridge -verify_arch arm64 x86_64`. Record actual outputs and exits.
These checks establish package identity/preparation, not projector observation.
The binary is unsigned and unnotarized; no signing or release choice is implied.

## Synthetic transfer identity

Follow the documented five-second `testsrc2` generation (640×360, 30 fps,
libx264, yuv420p, H.264 Annex B). Use the verified extracted executable for both
sender and receiver. Use fresh output paths; existing outputs are refused.
Wait for receiver readiness before sending and its successful exit before playback.

| Transfer field | Actual value / output / exit status |
| --- | --- |
| Generation command / generation exit | |
| Input absolute path / byte count / SHA256 | |
| Receiver executable path / command / loopback endpoint | |
| Receiver readiness observed at | |
| Sender executable path / command / exit | |
| Receiver output absolute path / exit | |
| Received byte count / SHA256 | |
| Input and received hashes equal? | |
| Failures, interruptions or partial files | |

Matching hashes establish synthetic byte transport only, not live console capture
or H.264 decoding. Record any additional decode checks separately if performed.

## Human projector observation

Expected playback command, from the received file's directory:
`ffplay -fs -autoexit -framerate 30 -f h264 received.h264`.

| Observation field | Actual human observation |
| --- | --- |
| Observer / observation date and time | |
| Actual player command / input absolute path | |
| Player exit status / stderr details | |
| Fullscreen window observed on Mac? | |
| Physical projector connected and mirroring confirmed how? | |
| Moving synthetic imagery directly observed on projector? Describe motion. | |
| Observation duration / full clip or early q/interruption | |
| Static/absent image, wrong display, scaling or playback issues | |
| Optional redacted supporting photo/video/log references | |

## Acceptance decision (blank until evaluated)

- [ ] Entire hosted workflow passed; actual artifact/source identity recorded.
- [ ] Archive/internal checksums, executable mode and both slices verified.
- [ ] Synthetic generation, sender and receiver succeeded; hashes match.
- [ ] Human directly observed received synthetic video moving fullscreen through
      the Mac's mirrored display on the physical projector; setup metadata recorded.
- [ ] Early termination, failures and limitations are accurately retained.

**Verdict (PASS / FAIL / PENDING):** ____________________

**Observer statement and reason:** ____________________

**Reviewer / review date / follow-up:** ____________________

Leave PENDING when no person has observed the physical test or required identity
checks are missing. Record FAIL for an attempted trial with absent/static imagery,
wrong/unmirrored display, checksum/transport failure or unsuccessful playback;
retain diagnostics and retry as a new trial. A player process exit 0, automated
smoke, screenshot or decode result alone cannot pass. Record early q honestly;
if it prevents establishing the required moving-image observation, acceptance
remains pending or fails according to the observed result.

A passed synthetic projector trial closes only this LEA-10 observation criterion.
Physical Switch reconnects, USB enumeration attribution and live Switch capture
remain separate pending gates. This template performs no test, updates no ticket,
and approves no merge, tag, release or signing workflow.
