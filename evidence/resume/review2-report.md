# USB review cycle 2

Based on parent baseline `b3a02d83d17995124577aed0e453165f52baf6c7`.
Only `src/usb.rs` implementation/tests and this cycle's evidence were changed
by the integration developer. Parent-owned review artifacts were preserved.
No source worktrees were changed; no commits or remote actions were performed.

## Design and compatibility

- Recognize private field labels case-insensitively after trimming: Serial Number,
  Serial, UUID, UDID, Device UUID, Device UDID, Hardware UUID, USB Serial Number,
  Device Serial Number. Ordinary `USB Serial:` and `USB-Serial Controller D:`
  device headers are accepted. Exact private labels with empty values still
  fail closed; missing-colon label/value text and indented continuations of
  recognized private fields still fail closed. Ambiguous headers named exactly
  `Serial:`/`UUID:` are rejected to preserve privacy. New profiler label variants
  need explicit support; no generic substring match on every device name.
- Preserve nonempty fields' structural keys, and preserve canonical `Vendor ID`
  and `Product ID` lines as evidence even if their entire value coincides with a
  private identifier. Private values in their own fields are always redacted.
- Known values of **four or more Unicode characters** are redacted as substrings
  in device headers and ordinary field value text, including `Device-ABC123`,
  `DeviceABC123` and an embedded UUID. This retains the original protection of
  long private values embedded in names. It can hide coincidental occurrences
  of those long strings in other ordinary values.
- Known values of **one to three Unicode characters** require boundaries on
  both sides: adjacent Unicode alphanumeric characters, `_`, or `-` prevent a
  match. Standalone short values in headers/values are redacted, including
  parenthesized and slash-separated occurrences. Serial `0` no longer corrupts
  `0x057e`/`0x201d`; serial `A` does not corrupt `A1`, `éA`, `Aé`, `_A`, `A-B`.
  Embedded occurrences of these short ambiguous values deliberately remain.
- Longest-value-first matching and single-pass output construction preserve
  existing overlap and marker-protection tests. Inserted markers are not
  scanned again. Both snapshots are validated, then their known private values
  are combined before redacting either diff side, preventing either-direction
  cross-inventory leakage. Identifiers absent from private fields in both inputs
  cannot be inferred. Human review before sharing inventories remains required.

## Red/green evidence

- `review2-red-usb.log`: three failures reproduce device-header rejection,
  identifier corruption and structural-key corruption. Existing privacy guards
  and explicit malformed-label guards pass.
- `review2-red-union.log`: both directions leak values identified only in the
  opposite snapshot before the union fix.
- `review2-red-embedded.log`: three failures reproduce the overly broad boundary
  policy leaving embedded long serials/UUIDs visible, including both diff
  directions. The final policy fixes those regressions.
- `review2-green-usb.log`: all 21 USB unit tests pass, including existing privacy
  tests. `review2-tests.log`: canonical full suite, 41 tests pass.

## Final validation

All commands exited 0 on the final Rust source:

| Command | Log |
| --- | --- |
| `cargo test --locked --all-targets` | `review2-tests.log` |
| `cargo fmt --check` | `review2-fmt.log` |
| `cargo clippy --locked --all-targets -- -D warnings` | `review2-clippy.log` |
| `cargo llvm-cov --locked --all-targets --fail-under-lines 80 --summary-only` | `review2-coverage.log` — 90.09% line coverage |
| `actionlint` | `review2-actionlint.log` |
| `shellcheck scripts/package-macos.sh` | `review2-shellcheck.log` |
| `bash scripts/package-macos.sh` | `review2-package.log` |
| extracted checksums, executable mode, both lipo slices and native ARM packaged sender/receiver | `review2-packaged-smoke.log` |
| `python3 evidence/resume/verify-synthetic.py` | `review2-synthetic.log` |

Real FFmpeg synthetic proof via the refreshed extracted package: 85,967 bytes,
SHA256 `487f583802823c66ba464d1a94067e7a77dd0abe037ec14be473dc8efd14b1b2`
for both input and output; successful decode; 60 H.264 frames at 160×90.

Native Intel/remote CI, three console reconnects, real USB live capture and
physical projector observation remain deferred. Parent will record frozen
snapshot metadata and perform fresh independent QA/review before committing.
