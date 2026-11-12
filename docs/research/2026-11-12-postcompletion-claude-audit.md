# Post-completion independent Claude audit — SwitchBridge

- Reviewer: Claude Code, Herdr Core terminal `t2q` (`switchbridge-final-audit`), parent `t9`; identity verified with `hc get`. Fresh reviewer: did not implement or previously review this work. Interactive; no subagents.
- Worktree / pin: `.worktrees/claude-final-audit`, branch `audit/claude-postcompletion`, HEAD `c23c2cf426c30923831b91665d0e746660c9b763` (clean). Remote `main` was still `c23c2cf` at audit time.
- Mode: read-only product audit. Only this file and ignored `.evidence/claude-audit/` logs were written, along with the normal ignored `target/`. No commits, pushes, PR/Linear edits, workflow dispatches, artifact downloads or device/hardware operations.

## Verdict

**Goal satisfied within its stated limits.** LEA-7 transport, LEA-8 USB tooling, LEA-9 feasibility documentation and LEA-10 macOS packaging are integrated on `main`. Independent reviews and their dispositions are recorded. Physical Switch 2 USB enumeration, live capture and projector acceptance are explicitly and consistently left pending. That matches the goal and is not a software blocker.

I found no correctness defect in framing, transport, CLI file handling, redaction logic, packaging or CI. I found one **Medium documentation/privacy-process inconsistency** that should be fixed before the human LEA-8 trials (F1). I also found one **Low stale-status/navigation issue** (F2). Green CI was not treated as proof of the whole goal; each row below cites the evidence used.

## Checks performed

| Check | Result |
| --- | --- |
| `cargo test --locked --all-targets` (fresh, own worktree, rustc/cargo 1.96.0) | Exit 0. **46 passed**, 0 failed: lib 25, main 0, cli 2, framing 7, network 3, transport 6, usb_cli 3. Log: `.evidence/claude-audit/cargo-test-all-targets.log`, SHA256 `0d2cbf8755073b293b41e9d60477ce527025ca0420149914c3ef999b09757c2d` |
| Code delta, PR2 head `ce8aeac` → `c23c2cf`, over `src tests scripts .github Cargo.*` | Empty. `ce8aeac` is an ancestor. Historical artifact proof at `ce8aeac` therefore covers identical runtime code |
| `docs/architecture/review/source-baseline.json`: re-hashed 20 frozen files | 20/20 match |
| `docs/architecture/review/manifest.json` `file_hashes`: re-hashed | 75/75 match, none missing |
| `gh pr view 1/2/3` (read-only) | All MERGED. PR2 → `e1b81f7` at 07:49:03Z; PR3 → `c23c2cf` at 08:12:34Z |
| `gh api …/runs/43181069547` and its jobs/artifacts | push on `main`, head `c23c2cf`, conclusion success. All 5 jobs success: Check macos-15 and macos-15-intel, Universal executable, Packaged smoke on both. Artifact `13925903782`, ZIP digest `sha256:1d2add20d6bcfd8e913529bab4bc0d3dd6d4c63097fdc16bfbe013b807301f53`, expires 2026-11-26T08:13:59Z. **Metadata only**: not downloaded or executed by me |
| `gh api …/runs/43141901934/artifacts` | Artifact `13909373980` not expired (expires 2026-11-26T02:31:45Z). ZIP digest matches recorded `dd2619f2…42e5` |
| Annotations on run 43181069547 | Node 20 deprecation warnings (actions forced to Node 24) on all jobs, plus an ARM queue notice; all jobs succeeded |
| `git check-ignore` on `evidence/trial-1-before.txt` vs `.evidence/…` | `evidence/` is **not** ignored (exit 1); `.evidence/` is ignored (exit 0). Log: `.evidence/claude-audit/ignore-check.log` |
| Primary source: [GitHub Node 20 deprecation changelog](https://github.blog/changelog/2025-09-19-deprecation-of-node-20-on-github-actions-runners/) | Node 24 default from 2026-06-16; Node 20 removed from runners 2026-09-23 |

Not run: fmt, clippy, coverage, actionlint, shellcheck, packaging and artifact downloads. These would not have answered any open question: hosted runs cover fmt/test/clippy natively, and no code changed since the recorded local runs. I did not view the PNGs myself because I had no visual finding to test. The visual PASS claims rest on the recorded human/peer reviews (`lead-visual-review.json`, `peer-review.md`), not on my inspection.

## Requirement-by-requirement evidence

| Requirement (own checklist) | Status | Evidence |
| --- | --- | --- |
| LEA-7 strict SWB1 framing | Met | `src/framing.rs:15-62`: magic, version/kind, zero flags, 1..=1 MiB length validated before allocation; clean EOF only at packet boundary. Covered by 7 `tests/framing.rs` tests |
| LEA-7 loopback transport, Annex B prefix, ordered chunks | Met | `src/transport.rs:9-80`, `src/main.rs:19-67`: numeric loopback and nonzero port; peer loopback check; create-new output; cleanup on handled error. Multi-chunk (150 KB) round trip and sequence/first-packet negatives are tested |
| LEA-7 limits disclosed | Met (declared) | No ACK/end marker/digest, unbounded accept, Ctrl-C partial file, chunk timestamps ≠ frame timing: README "SWB1 v1 wire format", HELP text, LEA-7 plan "Review focus" |
| LEA-8 snapshot/diff, redaction, no overwrite | Met (software) | `src/usb.rs`: fail-closed private fields, cross-input value sharing, short-token boundaries, `create_new`, redaction before file creation; 25 unit + 3 CLI tests. Residual-identifier limits are declared in `docs/usb-baseline.md` |
| LEA-8 three controlled reconnects | Pending (by goal) | Consistently stated in README, usb-baseline, feasibility report, architecture views |
| LEA-9 feasibility gate | Met (documentary BLOCKED/NO-GO) | `docs/research/stock-switch2-usb-feasibility.md:7`; PR1 merged `460e8e8` |
| LEA-10 universal package, checksums, native ARM + Intel CI | Met | `.github/workflows/macos.yml` check → package → smoke; `lipo -verify_arch`; outer/inner SHA256; native smoke on `macos-15` and `macos-15-intel`. Verified run 43181069547 (head `c23c2cf`) and historical downloaded-byte proof for run 43141901934 at identical code |
| LEA-10 projector observation | Pending (by goal) | `docs/macos-playback.md` "Projector acceptance evidence"; template exists |
| Independent review and findings resolved | Met | `evidence/resume/claude-final.json`, `codex-final.json` (no ≥medium on reviewed snapshot), architecture `paired-review.md` dispositions |
| Architecture deliverables (C4 context/container, two DFDs, CI workflow, Mermaid, receipts, source matrix) | Met | `docs/architecture/*` (5 views + evidence matrix). Hashes re-verified (above). Automated `visualReview: pending` (10 receipts) is kept separate from the human/peer visual PASS records. Evidence-matrix line anchors spot-checked (usb.rs 22/48/65/188/240/251, macos.yml 19/23/46/84, macos-playback.md 8/57/84) |
| Merge observed, main CI green | Met | PR2/PR3 MERGED; run 43181069547 all five success at `c23c2cf` |

## Findings (most severe first)

### F1 — Medium — Primary USB procedure collects unreviewed inventories into the git-tracked `evidence/` directory, contradicting the private-staging rule (NEW)

- **Where:** `README.md:74-76`; `docs/usb-baseline.md:38-41`. These contradict `docs/usb-trial-record-template.md:58` ("Never collect directly into `evidence/`") and the LEA-8 plan, Task 3.
- **Trigger:** An operator follows the README or the linked USB baseline procedure for the pending three-reconnect trials.
- **Impact:** Snapshots and diffs land in `evidence/trial-N-*.txt`, which is not git-ignored. A routine `git add -A` stages them. The docs themselves say automatic redaction "does not certify that a file is free of all identifying information." The step-5 shell redirect also overwrites an existing diff, unlike the snapshot's `create_new`. Neither procedure links the stricter template, which is reachable only from `evidence/resume/README.md:148`. The projector template had the same class of gap, rated Medium and fixed in `physical-template-qa.md`; the USB procedure did not get the same correction.
- **Reproduction:** Read the cited lines, then run `git check-ignore -v evidence/trial-1-before.txt` (exit 1) and `git check-ignore -v .evidence/lea-8/x/trial-1-before.txt` (exit 0, `.gitignore:2`).
- **Prior status:** Not found in prior review records (grep across `evidence/`, `docs/research/`, `docs/architecture/review/`).

### F2 — Low — Evidence index states stale current status and omits merged-main/architecture links (NEW)

- **Where:** `evidence/resume/README.md:125-126` ("PR #2 is open"), `:141-142` ("final architecture views wait for actual merged main"). The index does not link `merged-main-ci.md` or `docs/architecture/`. The root `README.md`, which points readers to this index, also has no link to `docs/architecture/`.
- **Impact:** Navigation and claim accuracy only. A reader starting from the root README cannot find the architecture views or the merged-main CI proof, and sees a stale "open" PR state. No runtime effect.
- **Note:** `docs/architecture/README.md` deliberately left the root README unchanged to preserve its frozen hash. Editing it requires refreshing `source-baseline.json`/manifest under the architecture "Maintenance" rule, or adding the link only from the evidence index.

### Considered and not raised

These are declared limitations or not defects:
- Trailing-packet loss, no ACK, unbounded accept, partial file on Ctrl-C.
- Subprocess output memory before the 16 MiB check; non-transactional write/sync failure (declared in `dataflow-usb-evidence.md`).
- Property names on value lines kept verbatim; case-sensitive value matching (covered by the declared "unknown labels / human review" limit).
- PR CI tests the head SHA rather than the merge ref. This is documented, and the post-merge push run covers it.
- The Node 20 annotation, already classified nonblocking in `latest-ci-status.md`. Per the primary changelog, Node 20 left runners on 2026-09-23, and the pinned actions already run successfully under forced Node 24.
- The `tests/network.rs` port-reservation race. Hypothetical flakiness, not observed.

## Ranked next actions

1. **REQUIRED before LEA-8 trials (small):** Change `README.md:74-76` and `docs/usb-baseline.md:38-41` to collect into a private ignored `.evidence/lea-8/<session>/`, using `umask 077`, `set -o noclobber` and a `git check-ignore` guard, as the template already does. Link `docs/usb-trial-record-template.md`, and state that only privacy-reviewed copies go to `evidence/lea-8/<session>/`.
   - *Validation:* `grep -rn 'evidence/trial-' README.md docs` returns only `.evidence/` paths; run a doc review of the two files. If the README hash changes, refresh the architecture baseline/manifest.
2. **HUMAN evidence (pending by goal):**
   - Run the three controlled HOME reconnects for LEA-8.
   - Do the LEA-10 projector observation. Use an artifact while it is still available: `13909373980` has independently verified bytes; `13925903782` at `main` has metadata only and needs its own checksum/smoke verification before it is cited. Both expire on 2026-11-26.
3. **OPTIONAL (small):** Refresh `evidence/resume/README.md` status: PR2/PR3 merged, link `merged-main-ci.md` and `docs/architecture/README.md`. Optionally add a metadata-only record for run 43181069547. Keep historical sections intact.
4. **OPTIONAL (small):** Clear the Node 20 warnings by moving checkout/upload/download-artifact to releases that target Node 24. Choose exact tags and commit SHAs only after checking the official `actions/*` release pages; this audit does not propose specific versions.

No other material issue found.
