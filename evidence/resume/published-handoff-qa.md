# Published handoff QA — immutable head and current follow-up patch

**Reviewer:** t20, 2026-11-12. **Current patch verdict: PASS — safe to commit as documentation/evidence-only follow-up within existing branch-maintenance authorization.** This is not merge, Linear, hardware, release or implementation approval.

## Scope and snapshots

Read-only coordinator worktree: `/Users/kaungkhantthu/Desktop/playground/rustSwitch/.worktrees/resume-integration`.

- Published immutable head: `9113cd17ca892f9401a90f2f2fcc5bb4a96b4738`; parent `ce8aeac1eab18cd54c494e7b8d8445da061a7e66`.
- Published commit: 19 changed files, all under `docs/` or `evidence/`; no runtime/source/test/package/workflow change.
- Separately audited current uncommitted patch: corrected projector template; evidence README, acceptance manifest, LEA-7 draft publication update, PR draft; new Claude handoff review, latest-CI record and physical-template QA. These eight files remain documentation/evidence only. Verdict applies to those inspected bytes, not later edits or a future unspecified commit.
- No builds, runtime execution, device operations, source edits, commits, remote mutations or subagents. Only this report was written in the assigned plan-lea7 worktree. Read-only GitHub connector and CLI metadata queries verified the requested run; no further remote research was needed.

## Verification results

| Check | Published 9113cd1 | Current patch |
| --- | --- | --- |
| Frozen source/doc/helper SHA256 manifest | **20/20 MATCH**, computed from exact committed blobs | **20/20 MATCH**, recomputed against working files |
| Planning import SHA256 manifest | **7/7 MATCH** against committed blobs | **7/7 MATCH** |
| Acceptance import SHA256 manifest | **4/4 MATCH** against committed blobs | **4/4 MATCH**, including refreshed LEA-7 draft and corrected projector template |
| Relative Markdown links | **59 occurrences resolve** across all 16 changed Markdown files; sole fragment `#sources` matches the research heading | **64 occurrences resolve** across published docs plus follow-up patch; no missing file target |
| Whitespace | `git diff --check ce8aeac 9113cd1` exits 0 | `git diff --check` exits 0 |
| Scope/claims/ownership | Plans, imports, blank templates and closeout drafts remain distinct from implemented software and actual observations | Correction adds private collection/review only; existing lane ownership, physical/merge/release gates retained |

Link inspection covered inline Markdown links and reference-style links, excluding fenced examples and external URLs; fragment heading was checked directly. Local sibling paths and temporary drafting paths in code spans are provenance pointers, not repository links or prerequisites replacing canonical ticket criteria.

## Exact latest CI compared with earlier runtime proof

Read-only connector queries returned run 43164099151 associated with the requested commit and all five successful jobs. Read-only `gh run view 43164099151 --repo Kaungkhant97/switchbridge --json headSha,status,conclusion,jobs` independently confirmed full `headSha=9113cd17ca892f9401a90f2f2fcc5bb4a96b4738`, `status=completed`, `conclusion=success`.

| Latest job | Job ID | Conclusion |
| --- | --- | --- |
| Check (macos-15) | 131001808501 | success |
| Check (macos-15-intel) | 131001808737 | success |
| Universal executable | 131002000086 | success |
| Packaged smoke (macos-15) | 131002172365 | success |
| Packaged smoke (macos-15-intel) | 131002172368 | success |

[Latest run](https://github.com/Kaungkhant97/switchbridge/actions/runs/43164099151) proves hosted native checks/package/smoke success for the published handoff head. `latest-ci-status.md` correctly distinguishes its GitHub-reported artifact metadata (artifact 13918893069, API ZIP digest) from independently downloaded or locally executed bytes; this audit did not download that artifact or verify its inner hashes.

[Earlier run 43141901934](https://github.com/Kaungkhant97/switchbridge/actions/runs/43141901934), source `ce8aeac`, artifact 13909373980 retains the independent downloaded ZIP/tar/executable checks and ARM FFmpeg 85,967-byte equality/decode/60-frame proof in `hosted-ci.md/json`. It must not be relabeled as fresh local validation of artifact 13918893069. Local 46-test/90.98%-coverage evidence retains its recorded source/toolchain attribution. All 20 preserved hashes match, so runtime evidence has not been invalidated by these document changes. No CI result is invented for the yet-uncommitted follow-up patch.

## Findings and dispositions

1. **Historical MEDIUM at 9113cd1 — projector private staging/review missing.** `docs/projector-trial-record-template.md:24–27` only instructed exclusion of private details; the original scratch/relative-path procedure did not require ignored/private collection or explicit artifact review before share handoff. This conflicts with README's generated-media/private-evidence practice. It is a real documentation defect in the immutable published snapshot, not a failed transport feature.

   **RESOLVED in current patch:** Actual imported corrected template SHA256 is `00cb8edd66d52b508a5a9e2292772a654bc01f9760209388aad658ecc5370061`, matching the independently re-reviewed bytes in `physical-template-qa.md` and refreshed acceptance manifest. Direct diff/read confirms ignored `.evidence/lea-10/<trial>/` collection before downloads/recording, absolute paths, per-terminal restrictive umask/permission/ignore checks, private logs/media/records, preserved originals, artifact-by-artifact review and sanitized-copy handoff. Share-set hashes remain separate from package and media hashes. Blank physical acceptance and actual moving-image observation requirements remain intact. No open Medium remains for the inspected correction.

2. **Historical LOW — stale publication wording in imported LEA-7 draft.** The published draft's earlier text says the new evidence is unpushed. It accurately records the prior audit but is confusing without a prominent later status. **Resolved for the current patch:** appended dated Publication update names 9113cd1, latest five-job run and distinct artifact evidence, preserving the historical audit rather than silently changing its provenance. The draft's refreshed manifest hash matches. Earlier frozen metadata pending items are likewise explicitly superseded by current README/latest-CI records; keep frozen metadata untouched.

3. **No additional actionable acceptance/status/ownership defect identified in this bounded handoff audit.** Complete published file inventory, new claims and current patch were inspected. Templates contain no fabricated trials; source proof is host prerecorded transport only; console reconnects, USB live capture, projector observation and optional release policy remain separate. Documentation authoring still waits for observed merge and pinned main. Imported Claude review is scoped to its actual snapshot and does not substitute for the exact corrected-template re-review.

## Commit-readiness recommendation

The inspected current patch is coherent and safe for the coordinator to commit under the existing authorization, preserving the refreshed manifest and both historical/current review records together. No runtime rebuild is warranted for this patch. If any bytes change after this audit, recheck affected manifest/link/diff conditions. Publication of a later head should retain the latest run's exact SHA instead of calling it a run on that future head. Actual PR merge still requires its separately approved workflow and current readiness inspection; Linear changes and all hardware/release operations remain outside this QA verdict.
