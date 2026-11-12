# Resume integration verification

Date: 2026-11-12. Branch: `feat/resume-integration`. Local host: macOS ARM64,
Rust 1.96.0. Source attribution and SHA256 values are in [provenance.md](provenance.md).
No historical reviews were imported as approval. The developer handed off unstaged
changes; parent saved baseline `b3a02d8` and separate review fixes.

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

## Initial integration checks (`b3a02d8`)

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
- Final independent review is tracked below. No pushes, PRs, ticket changes,
  credential changes or paid jobs were performed.

The fast-context search attempt failed due to an unavailable Windsurf API key;
direct local inspection was used without changing credentials.

## Review cycle 2 QA (`ab2035e`)

Parent reran the full canonical suite and package checks after the USB fixes:
[final-qa.log](final-qa.log) records **41 passing tests**, **90.09% source-line
coverage**, formatting, Clippy with warnings denied, actionlint, shellcheck,
universal packaging, archive/internal checksums, executable mode, both
architecture slices, and actual extracted sender/receiver TCP smoke on ARM.
The extracted package also preserved 85,967 bytes of FFmpeg-generated H.264,
matched SHA256, decoded successfully, and yielded 60 frames at 160×90.
A single local USB snapshot and self-diff exercised the actual collector;
raw output stays ignored. This is host-tool smoke, not a controlled console trial.

[Review cycle 2](review2-report.md) documents fixes, red-before-green regression
logs, and the precise privacy policy. It resolves device-heading rejection,
short-serial corruption of IDs/keys, and leakage across the two diff inputs.
Additional tests prevent regressions for long identifiers embedded in names.
The initial [Claude report](claude-preliminary.md) and
[Codex report](codex-round1.json) are findings on earlier sources, not final
approval. [CI reference checks](ci-reference-check.md) validate official action
commit references and runner labels without claiming a hosted run.

Frozen revision, executable/archive/source hashes, UTC timestamp, and tool
versions are stored in [verification-metadata.json](verification-metadata.json).
Final reviewer results and the local convergence verdict are indexed in
[review-log.md](review-log.md). Those files
identify the exact snapshot reviewed and retain the hardware/hosted-CI gates.

## Release candidate QA after cycle 3

[release-qa.log](release-qa.log) is the current parent verification: **46 tests
pass**, **90.98% source-line coverage**, formatting, Clippy, workflow/shell lint,
fresh universal packaging, verified archive/internal checksums and architecture
slices, and actual native ARM extracted-package TCP smoke. Real 60-frame H.264
transport/decode passes normally and under `python3 -O`; its checks now use
explicit exceptions. The host USB collector and self-diff also pass; raw host
snapshots remain ignored and are not controlled console experiments.

[Cycle 3](review3-report.md) preserves failing-then-passing UUID-label,
colon-bearing heading, and receive-cleanup regressions. Its generalized private
label fixtures are synthetic, not observed legacy-profiler output. The
[second Claude review](claude-round2.md) motivated these changes. The
[previous clean Codex review](codex-round2.json) applies to snapshot `948a5cc`,
not these subsequent changes. The current frozen metadata and final two-reviewer
verdict supersede those earlier snapshot results. Native Intel, hosted CI,
console reconnect/live-frame, and projector acceptance remain pending.

## Reviewable handoff and remaining gate

Final [Codex](codex-final.json) and [Claude](claude-final.json) reviews agree
there are no medium-or-higher findings on source snapshot `d927173`, reviewed
at `2db17bd`. Low/speculative observations and all execution limitations are
retained. The later merge of already-landed LEA-9 research preserves every
reviewed source/doc/helper hash; historical reports are not new approvals.

The [PR draft](pr-draft.md) is prepared for publication after explicit approval.
Hosted ARM/Intel CI, native Intel execution and a downloadable GitHub artifact
link remain unverified. The full goal has not been declared achieved while
that remote validation remains. Console reconnect/live-frame and physical
projector gates remain separately pending as requested.

## Hosted CI verification

User authorized publication; [PR #2](https://github.com/Kaungkhant97/switchbridge/pull/2)
is open. [Run 43141901934](https://github.com/Kaungkhant97/switchbridge/actions/runs/43141901934)
passed all five jobs at exact source `ce8aeac`: ARM and Intel checks, universal
package, extracted-package smoke on ARM and Intel. [Hosted QA](hosted-ci.md)
records actual artifact ID, expiry, ZIP/archive/executable hashes and downloaded
package runtime validation; [structured results](hosted-ci.json) retain job and
command conclusions. This supersedes earlier pending hosted/native Intel notes;
physical acceptance is still pending. A newly requested CI/CD release extension
is being assessed separately; no release or tag has been published.

## Parallel work handoff

The [consolidated roadmap](../../docs/superpowers/plans/2026-11-12-switchbridge-parallel-work.md)
links the four ticket-owner plans and architecture pair's reviewed plan. The
[import manifest](parallel-planning.json) records exact reviewed document hashes
and ownership. Primary-source [C4/data-flow research](../../docs/research/2026-11-12-c4-dataflow-practices.md)
includes the independent review and corrections. These are planning artifacts;
final architecture views wait for actual merged main. Hardware observations and
release policy remain pending.


Practical follow-up records: [LEA-7 closeout draft](lea7-closeout-draft.md),
[LEA-9 closeout draft](lea9-closeout-draft.md),
[blank USB trial record](../../docs/usb-trial-record-template.md) and
[blank projector record](../../docs/projector-trial-record-template.md).
Their import hashes are in [acceptance-handoff.json](acceptance-handoff.json).
Templates contain no physical observations; closeout text has not been posted.
