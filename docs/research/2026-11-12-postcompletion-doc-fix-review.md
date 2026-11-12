# F1/F2 documentation-fix re-review — SwitchBridge

- Reviewer: Claude Code `t2q` (`switchbridge-final-audit`), parent `t9`. This is a scoped re-review of the F1/F2 fixes only, not a new broad audit. No implementation, commits, remote actions or subagents.
- Baseline: originals at `c23c2cf426c30923831b91665d0e746660c9b763`. The original audit `docs/research/2026-11-12-postcompletion-claude-audit.md` is unchanged (SHA256 `91d4964d6811435794994de98dcd1f60df384f6d74e798675105802d4d64715a`).
- Inputs: only the local copies in ignored `.evidence/claude-audit/doc-fix-review/`. I read no sibling worktrees.

## Verdict

**PASS. No material findings.** F1 and F2 are resolved. Three Low, non-blocking observations are listed below. No hardware observation is claimed or implied.

## Reviewed bytes

| File | SHA256 (copy = manifest source = manifest copy) |
| --- | --- |
| `README.md` | `f75dee49b8903ec0f377744a4fc03ac97d846994b636163fc6df92848fa3b706` |
| `docs/usb-baseline.md` | `f7941867ed2cab4423baaf5c7227c4d04aad577333d1567d197f563763b3cc6f` |
| `evidence/resume/README.md` | `f22527cbe3a55679a668eb1a5d12b504b1a7fdba2f9c57882144f52e5dc1331f` |
| `patch.diff` | `57cace9e586cd4cc4f69857629a5eb6b8a54939f412ef77ec889b7b8c94cb614` |

Provenance check: I extracted the three `c23c2cf` originals with `git archive` into a scratch directory, and `git apply --check` plus `git apply` of `patch.diff` succeeded. The three results are byte-identical (`cmp`) to the reviewed copies. The patch touches only these three documentation files: no runtime code, tests, workflow, architecture package, frozen source baseline or manifest.

## F1 — private USB collection: RESOLVED

- **No tracked collection path remains.** `grep 'evidence/trial-\|"evidence/'` over the patched `README.md` and `docs/usb-baseline.md` returns nothing. Every snapshot, diff and log targets `"$SESSION"` (`.evidence/lea-8/<unique>`). Tracked `evidence/lea-8/<session>/` is reserved for sanitized copies after an explicit privacy review, with reviewer/date, a sanitized mapping and `SHA256SUMS`.
- **Guards run as intended.** I ran the exact snippet in a synthetic scratch git repo whose `.gitignore` contained `/.evidence/`, under both bash and zsh:
  - `umask 077` produced `drwx------` for the session directory and `-rw-------` for the file.
  - A second `>` redirect under `noclobber` was refused (exit 1) and the original content was kept.
  - Reusing an existing session name made `mkdir "$SESSION"` fail, and `|| exit 1` stopped the script.
  - `git check-ignore` outside a repo exits nonzero, so the procedure stops.
  - From inside the repo, the root-anchored `/.evidence/` rule does not cover a subdirectory's `.evidence/`. Running from the wrong working directory therefore also fails closed (my reading of the rule; I did not run this case).
- **Handoff.** Both documents link `docs/usb-trial-record-template.md`, which exists. They keep Copy to PC records separate and keep private originals and failed attempts. Linear sharing still requires authorization, and the result label is still "USB enumeration only; video UNVERIFIED". The new text makes no claim of a physical trial.

## F2 — status and navigation: RESOLVED

- The new "Current navigation and provenance boundary" section in `evidence/resume/README.md` states that PR2 and PR3 are merged. It keeps `merged-main-ci.md` (`e1b81f7` / run `43178062662`) distinct from this audit's metadata-only observation of `c23c2cf` / run `43181069547`. It does not claim downloaded-byte QA for either newer artifact, and the `ce8aeac` downloaded proof stays the only byte-validated artifact. All three statements match my original audit.
- Historical sections are relabelled rather than rewritten: "Initial deferred acceptance (historical)", "records the historical parent verification", and "was open at this hosted-verification snapshot and is now merged". The architecture views are now linked from both the root README and the index. Every link target exists in the tree, except the one in observation O1.
- Leaving the frozen 20-file source baseline and the 75-file architecture manifest unchanged is correct. The index explicitly says these are historical `e1b81f7` proof, not a claim about later HEAD hashes.

## Low observations (non-blocking)

- **O1 — conditional link.** `evidence/resume/README.md` links `../../docs/research/2026-11-12-postcompletion-claude-audit.md` twice. In this worktree that file is untracked (`git ls-files --error-unmatch` fails). The link resolves only if the unchanged audit file is committed in the same change.
- **O2 — architecture prose is pin-relative.** `docs/architecture/README.md:23` ("Root README was left unchanged, so its frozen hash still matches") and `docs/architecture/evidence.md:5` ("including unchanged root README") remain true for the `e1b81f7` pin, but no longer for HEAD once this patch lands. The new index boundary paragraph covers this. Keeping the architecture package byte-stable is a defensible choice; an optional one-line "as of `e1b81f7`" clarification could go in a later architecture revision.
- **O3 — `|| exit 1` closes an interactive shell.** In an interactive terminal the guard exits that shell rather than just stopping. It fails safe, and the text already asks for a dedicated shell, so no change is required.

## Not verified

- The rendered GitHub markdown.
- Any runtime, CI, artifact or hardware behaviour (outside this scope; runtime code is unchanged).
