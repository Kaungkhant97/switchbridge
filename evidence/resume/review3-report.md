# USB and receiver review cycle 3

Baseline: `948a5ccff6a08e7f41eacf373ccd0e0d544c3ae1`.
Implementation changes remain unstaged. No commits, remote actions, subagents,
source-worktree edits, or edits to parent-owned metadata/review reports.

## Changes and regression evidence

N1: `review3-red-labels.log` reproduces a synthetic `Volume UUID: 1234-ABCD`
leak. Recognition now accepts existing exact private labels, normalized keys
ending `UUID`/`UDID`, and keys containing `Serial Number`. Normalization collapses
whitespace and ignores ASCII case. Regression fixtures cover Volume UUID,
Interface UDID, qualified Serial Number, whitespace/case normalization, and
unchanged ordinary `USB Serial:` device headers. Each new private label rejects
empty, missing-colon and multiline forms. `review3-green-labels.log` records the
first passing label fix; `review3-green-usb.log` records all 25 final USB tests.
These are synthetic fixtures, not claims of observed hardware/profiler output.

N2: `review3-red-colons.log` reproduces a colon-bearing serial left in snapshot
headings and both diff directions. Non-property device headings ending in a
colon are now scanned as whole lines, so first-colon splitting cannot hide the
beginning of a private value in a heading's apparent key. Canonical Vendor ID /
Product ID fields and recognized private-field labels retain precedence and are
preserved. The snapshot regression writes and reads an actual redacted inventory
file; the serial-in-heading and serial-in-private-field fixtures include colons,
including a private value itself ending in a colon. Existing longest-first,
marker, short-value and cross-inventory protections remain covered.

N4: `review3-red-cleanup.log` reproduces loss of the protocol error: after
readiness the network test unlinks the receiver's still-open output, sends a
full invalid 24-byte SWB1 header, and gets only ENOENT from cleanup. The receiver
now reports a cleanup failure separately and returns the original receive error.
The test retains the stderr reader after readiness, waits for bounded process
termination, and verifies both `invalid SWB1 header` and `cleanup failed` survive.
The existing child drop guard kills/reaps failed or timed-out receivers.
`review3-green-network.log` records all three network tests passing.

N5: synthetic verification now uses explicit RuntimeError checks for receiver
status, SHA256 equality and 60 decoded frames. These validations remain active
under Python optimization. No mirrored tests added for this verification script;
real functional runs pass both normally and under `python3 -O`.

Changed files: `src/usb.rs`, `src/main.rs`, `tests/network.rs`,
`evidence/resume/verify-synthetic.py`, `docs/usb-baseline.md`, and new
`evidence/resume/review3-*` evidence. The USB baseline documents the actual label
pattern and whole-heading behavior.

## Compatibility and boundaries

UUID/UDID suffix and Serial Number phrase recognition is deliberately
conservative: an ordinary heading that matches a private-field pattern may be
rejected as empty private data. Ordinary USB Serial headers remain accepted.
Other unrelated unknown identifiers still need human review. Whole-heading
redaction can remove coincidental known identifiers from apparent labels in
colon-terminated non-private lines; canonical VID/PID and private-field labels
are explicitly preserved. Four-or-more-character private values retain embedded
substring matching; one-to-three-character values retain identifier-boundary
matching. Known private values from both diff inputs are still shared. Cleanup
failure is diagnostic and no longer replaces the original I/O error.

## Final verification

All commands exited 0 on the final source:

| Command/check | Evidence |
| --- | --- |
| `cargo test --locked --all-targets` | `review3-tests.log`: 46 tests pass |
| `cargo fmt --check` | `review3-fmt.log` |
| `cargo clippy --locked --all-targets -- -D warnings` | `review3-clippy.log` |
| `cargo llvm-cov --locked --all-targets --fail-under-lines 80 --summary-only` | `review3-coverage.log`: 90.98% lines |
| `actionlint` | `review3-actionlint.log` |
| `shellcheck scripts/package-macos.sh` | `review3-shellcheck.log` |
| `bash scripts/package-macos.sh` | `review3-package.log` |
| extracted archive/binary checksums, mode 0755, arm64+x86_64 slices, native ARM packaged TCP smoke | `review3-packaged-smoke.log` |
| `python3 evidence/resume/verify-synthetic.py` | `review3-synthetic.log` |
| `python3 -O evidence/resume/verify-synthetic.py` | `review3-synthetic-optimized.log` |

Both real FFmpeg runs use the refreshed extracted universal package: 85,967
bytes, matching SHA256
`487f583802823c66ba464d1a94067e7a77dd0abe037ec14be473dc8efd14b1b2`,
successful decoding, 60 H.264 frames at 160×90. No console capture or projector
observation was performed. Native Intel/remote CI and physical acceptance remain
deferred. Parent owns final QA, frozen metadata/index updates and commits.
