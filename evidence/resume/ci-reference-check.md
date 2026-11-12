# CI reference check

Parent checked these official GitHub sources on 2026-11-12. This is static configuration validation, not a hosted workflow execution.

- The [runner image matrix](https://github.com/actions/runner-images#available-images) lists `macos-15` as ARM64 and `macos-15-intel` as x64. The workflow requests these labels for native checks and extracted-package smoke.
- The pinned [checkout commit](https://github.com/actions/checkout/commit/11d5960a326750d5838078e36cf38b85af677262) exists on the official actions repository's v4 release line.
- The pinned [upload-artifact commit](https://github.com/actions/upload-artifact/commit/ea165f8d65b6e75b540449e92b4886f43607fa02) and [download-artifact commit](https://github.com/actions/download-artifact/commit/d3f86a106a0bac45b974a628896c90dbdf5c8093) exist on their official repositories.

Local `actionlint` passed. Actual runner scheduling, native Intel execution, workflow success, and downloadable GitHub artifact links remain unverified until an authorized remote run. Local cross compilation and archive verification do not close those gates.
