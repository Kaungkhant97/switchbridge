# Latest CI readiness — metadata inspection

Inspected 2026-11-12T06:41:33Z using read-only GitHub CLI API queries.
[PR 2](https://github.com/Kaungkhant97/switchbridge/pull/2) head and
[run 43164099151](https://github.com/Kaungkhant97/switchbridge/actions/runs/43164099151)
head both equal `9113cd17ca892f9401a90f2f2fcc5bb4a96b4738`.
Run status is completed, conclusion success. This record inspects metadata and
annotations only; no new artifact download, hash computation or local runtime
verification was performed.

| Job | ID | Conclusion |
| --- | --- | --- |
| Check (macos-15) | 131001808501 | success |
| Check (macos-15-intel) | 131001808737 | success |
| Universal executable | 131002000086 | success |
| Packaged smoke (macos-15) | 131002172365 | success |
| Packaged smoke (macos-15-intel) | 131002172368 | success |

Step metadata also reports successful formatting/tests/clippy, extracted archive
verification and both native packaged execution steps. These are hosted step
conclusions, not locally measured numeric exits.

## Current artifact metadata

| Field | GitHub-reported value |
| --- | --- |
| Artifact | [13918893069](https://github.com/Kaungkhant97/switchbridge/actions/runs/43164099151/artifacts/13918893069) |
| Name | `switchbridge-macos-universal-9113cd17ca892f9401a90f2f2fcc5bb4a96b4738` |
| Size | 482278 bytes |
| Created | `2026-11-12T05:52:02Z` |
| Expires | `2026-11-26T05:52:00Z` |
| Expired | false at inspection |
| API artifact digest | `sha256:d43350640370ae3167677935822cb0c9e27cd73d18079af6c04f0716e3387f7d` |
| Source/run association | exact head above / 43164099151 |

Metadata source: [run artifacts API](https://api.github.com/repos/Kaungkhant97/switchbridge/actions/runs/43164099151/artifacts).
The API digest identifies GitHub's uploaded artifact archive (ZIP), not the
inner universal tar.gz or executable. Those inner digests are not exposed in
this artifact metadata and were not independently retrieved or computed here.
Do not reuse older tar/executable hashes as this artifact's identity.

## Node 20 annotation assessment

All five job annotations warn that actions targeting Node.js 20 were forced to
run on Node.js 24. Checkout is named in every job; package additionally names
upload-artifact and smoke additionally names download-artifact. Annotation
severity is warning; all affected action steps and all jobs succeeded. ARM
jobs additionally have a runner-capacity queue-time notice.

Classification: **nonblocking action-runtime maintenance**, not an observed
required CI defect in this run. Current metadata shows successful operation
under the forced runtime. It does not prove all future runner/action behavior;
updating pinned actions can be reviewed separately without replacing working CI
or expanding optional CD. No workflow change is made or required to report this
exact run as passing. GitHub links the warning to its
[Node 20 deprecation notice](https://github.blog/changelog/2025-09-19-deprecation-of-node-20-on-github-actions-runners/).
Annotation evidence: check suite 116886138350 and each job's `/annotations` API,
including [ARM check annotations](https://api.github.com/repos/Kaungkhant97/switchbridge/check-runs/131001808501/annotations).

## Evidence boundary

The older run 43141901934 / source
`ce8aeac1eab18cd54c494e7b8d8445da061a7e66` / artifact 13909373980 has separately
verified downloaded ZIP, tar and executable hashes, local ARM packaged smoke,
and real synthetic H.264 transfer/decode with 60 frames in `hosted-ci.md` and
`.json` (parent integration evidence). That proof remains tied to those older
bytes. It is not a new local 60-frame verification of artifact 13918893069.

The current run establishes hosted native checks and packaged smoke success at
the latest exact PR head, with GitHub-reported artifact identity/expiry.
Physical projector observation, console reconnect attribution and live Switch
capture remain pending. Release policy is unchanged and undecided. No remote
mutations, source/workflow edits, commits, tags or releases were performed.
