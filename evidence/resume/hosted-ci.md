# Hosted CI artifact QA

Run [43141901934](https://github.com/Kaungkhant97/switchbridge/actions/runs/43141901934) completed successfully for source `ce8aeac1eab18cd54c494e7b8d8445da061a7e66`. All five job checkout logs show that exact source SHA. No rerun or remote mutation was performed.

| Job | ID | Result |
|---|---|---|
| Check (macos-15) | 130930614886 | success |
| Check (macos-15-intel) | 130930614925 | success |
| Universal executable | 130930849696 | success |
| Packaged smoke (macos-15-intel) | 130931086800 | success |
| Packaged smoke (macos-15) | 130931086823 | success |

Hosted logs: `.evidence/ci/run.log`; full job/step results: `.evidence/ci/run.json` and `hosted-ci.json`.
ARM check and package use image `macos-15-arm64` and host toolchain `aarch64-apple-darwin` (log lines 17/130, 554/668). Intel check uses image `macos-15`, native host toolchain `x86_64-apple-darwin` (285/398). Packaged Intel smoke uses configured `macos-15-intel`, image `macos-15` (771), and executes the extracted executable directly: archive/internal checksum OK and loopback hash at lines 914–916. ARM smoke uses `macos-15-arm64` (948) and checksum/loopback at 1091–1093. The smoke workflow does not print uname; Intel architecture attribution comes from runner/image and its matching check-job host toolchain, rather than an explicit smoke uname measurement. Both extracted-package native smoke steps succeeded. No native Intel execution was performed on this local ARM machine.

Artifact `switchbridge-macos-universal-ce8aeac1eab18cd54c494e7b8d8445da061a7e66` has ID `13909373980`, expiry `2026-11-26T02:31:45Z`, expired=false. [Download page](https://github.com/Kaungkhant97/switchbridge/actions/runs/43141901934/artifacts/13909373980); [API ZIP download](https://api.github.com/repos/Kaungkhant97/switchbridge/actions/artifacts/13909373980/zip). Metadata is in `.evidence/ci/artifacts.json`. ZIP bytes downloaded separately match the API digest and their members match the files obtained with `gh run download`.

| Downloaded object | Actual SHA256 |
|---|---|
| GitHub artifact ZIP | `dd2619f27c52750f6c9f62e97b40c42d556739d9aa44c276d3316c3b17e442e5` |
| Universal tar.gz | `cae0c6daca0e827bccf6f6f4082f3fb515e101f348a026b70f73f341b3ed1c51` |
| Extracted switchbridge executable | `d9bfbae8a694c1c1371f2e1eb2faedbdbc1246b0d058b2aed14a763380b4d94c` |

The tar.gz checksum passes its downloaded sidecar; executable checksum passes extracted `SHA256SUMS`. No equality against a local build manifest was assumed. Extracted executable mode is `0755`; lipo reports `x86_64 arm64` and verifies both slices.

Local runtime observation: `uname -m` = `arm64`. `scripts/smoke-packaged-macos.py` passed against `.evidence/ci/extracted/switchbridge`, sentinel loopback SHA256 `60a6fd49ac08f5dc8c9e87d566881dc1c899b2fffbc83fdd05dedf1def6e2118`. A scratch adaptation of `evidence/resume/verify-synthetic.py` changed only the archive path to this downloaded artifact. Real FFmpeg generated and transported 85,967 bytes of H.264; input/output SHA256 both `487f583802823c66ba464d1a94067e7a77dd0abe037ec14be473dc8efd14b1b2`. FFmpeg decoded successfully and ffprobe observed H.264, 160×90, 60 frames. Every validation subprocess exited 0; command/output/exit records are in `.evidence/ci/validation.json` and embedded in `hosted-ci.json`.

Herdr watcher t1r exit 0; artifact download t1s exit 0; hosted log download t1t exit 0; ZIP download t1x exit 0; corrected validation t1w exit 0. Initial validation launch t1v exited 2 because the scratch validator had not yet been created; it executed no artifact checks. This was corrected before the successful validation. Metadata/result retrieval and ZIP digest/content assertions exited 0. Hosted steps expose success conclusions, not raw numeric shell exits; those conclusions are preserved without inventing exit codes.

This closes the hosted/native Intel CI evidence gap. Physical console, projector, USB live capture, and three reconnects were neither observed nor required here; hardware gates remain pending. Source, tracked validation scripts, prior reports, PR, remote resources, and credentials were unchanged.
