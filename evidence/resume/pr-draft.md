## Summary

Integrate the host transport, USB evidence tooling and macOS distribution work
into one executable. Direct stock Switch 2 USB live capture remains BLOCKED;
synthetic transport does not establish console capture.

```text
switchbridge
├── send / receive       SWB1 prerecorded H.264 over numeric loopback endpoints
├── usb-snapshot         redacted macOS inventory; refuses existing output
├── usb-diff             validates/redacts both inputs using known identifiers
└── macOS workflow       native ARM/Intel checks → universal archive → native smoke
```

Refs LEA-7, LEA-8 and LEA-10. Preserve already-merged LEA-9 research/PR #1 and
its physical evidence gate. No third-party runtime dependencies added.

## Evidence

- **Before:** USB commands were unreachable from the transport CLI; serial
  headings could fail collection, private values could leak across diff inputs,
  and short serials could corrupt USB identifiers. Actual universal packaging
  failed at lipo argument ordering.
  **After:** Regression fixtures fail before each fix and pass afterward;
  actual package creation/extraction/checksums/architectures/native ARM TCP pass.
- `cargo test --locked --all-targets`: **46 pass**. Formatting, Clippy with warnings
  denied, actionlint and shellcheck pass; line coverage **90.98%**. Extracted
  package transports 85,967 bytes of real FFmpeg-generated H.264 byte-for-byte
  and decodes 60 frames, with normal and optimized Python validation.
- Independent Codex and Claude final reviews clear medium-or-higher on frozen
  source; execution restrictions and residual low observations are documented.
  See `evidence/resume/review-log.md`, `release-qa.log` and
  `verification-metadata.json` for exact source/artifact hashes and tool versions.
- Hosted CI/native Intel execution and downloadable GitHub artifact links are
  **pending this PR's run**. Three physical console reconnects, attributable USB
  gameplay frames and projector observation remain open; do not close their
  hardware acceptance gates based on these host tests.

## Merge Danger

**Door:** two-way

Changes can be reverted. No console write/control protocol or native capture
source is introduced; network transport remains loopback-only. Inventories still
require human review before sharing. The development binary is unsigned.

**Blast Radius:** host

Adds CLI transport/evidence collection and macOS build artifacts. PR CI tests
the exact PR head on ARM and Intel; local cross-compilation is not native Intel
execution. SWB1 lacks a final stream digest/acknowledgment; proof retains explicit
file hash comparisons.
