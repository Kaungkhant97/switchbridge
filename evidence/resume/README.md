# Resume integration verification

Date: 2026-11-12. Branch: `feat/resume-integration`. Local host: macOS ARM64,
Rust 1.96.0. Source attribution and SHA256 values are in [provenance.md](provenance.md).
No historical reviews were imported as approval. All changes remain unstaged.

## Test-first evidence

- [red-cli.log](red-cli.log): copied LEA-7 transport CLI and TCP tests pass;
  all three new USB CLI tests fail before wiring (diff treats a path as an
  endpoint, snapshot misses USB validation, help misses USB commands).
- [green-tests.log](green-tests.log): after exposing `usb` and dispatching USB
  commands before transport endpoint parsing, all 32 tests pass. Existing
  framing, transport and TCP tests retained. USB module tests cover serial/UUID
  redaction and refusal to replace existing evidence.
- [red-package.log](red-package.log): actual universal build fails because
  Apple's lipo parses the trailing executable as another architecture.
- [package.log](package.log): actual build succeeds after placing the binary
  before `-verify_arch` in script, CI and docs.

## Canonical checks

Each command exited 0; empty logs indicate no diagnostics.

| Command | Evidence |
| --- | --- |
| `cargo test --locked --all-targets` | [32 passing tests](green-tests.log) |
| `cargo fmt --check` | [fmt.log](fmt.log) |
| `cargo clippy --locked --all-targets -- -D warnings` | [clippy.log](clippy.log) |
| `cargo llvm-cov --locked --all-targets --fail-under-lines 80 --summary-only` | [coverage.log](coverage.log), 87.25% lines |
| `actionlint` | [actionlint.log](actionlint.log) |
| `shellcheck scripts/package-macos.sh` | [shellcheck.log](shellcheck.log) |
| `bash scripts/package-macos.sh` | [package.log](package.log) |

[packaged-smoke.log](packaged-smoke.log) records extracted archive and executable
checksum verification, executable mode, both architecture slices, and native ARM
packaged CLI sender/receiver smoke. No native Intel execution is claimed.

[synthetic-roundtrip.log](synthetic-roundtrip.log) records a real FFmpeg-generated
2-second, 160×90, 30 fps H.264 Annex B clip: 85,967 bytes, identical input/output
SHA256, successful FFmpeg decode and 60 decoded frames. This uses the extracted
universal package. Reproduce after packaging with
`python3 evidence/resume/verify-synthetic.py`. Generated media and extracted
binaries use temporary directories and are not committed as evidence.
Chunk timestamps remain transport elapsed times, not frame timing.

## Deferred acceptance

- LEA-8: no physical console test performed; three observed controlled reconnect
  pairs and attributable enumeration are still required.
- LEA-9: direct USB live capture remains BLOCKED until real attributable live
  console frames are demonstrated; no synthetic data substitutes for this.
- LEA-10: native ARM and Intel test/fmt/clippy CI and both packaged smoke jobs are
  configured; remote workflow execution is deferred. Native Intel CI is required.
  Physical moving-image projector observation remains pending.
- Parent will perform fresh independent QA/Codex/Claude review and commit.
  No pushes, PRs, ticket changes, credential changes or paid jobs were performed.

The fast-context search attempt failed due to an unavailable Windsurf API key;
direct local inspection was used without changing credentials.
