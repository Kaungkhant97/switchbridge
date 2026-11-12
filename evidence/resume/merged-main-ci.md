# Merged-main CI provenance

Read-only GitHub metadata inspection at **2026-11-12T07:51:15Z**.
[PR 2](https://github.com/Kaungkhant97/switchbridge/pull/2) is MERGED at
`2026-11-12T07:49:03Z`, merge commit
`e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab`.

[Post-merge run 43178062662](https://github.com/Kaungkhant97/switchbridge/actions/runs/43178062662)
is completed with conclusion **success**, event `push`, exact head SHA equal to
that merge commit. Artifact metadata associates the run with branch `main`.

| Job | Job ID | Conclusion |
| --- | --- | --- |
| Check (macos-15) | 131047024316 | success |
| Check (macos-15-intel) | 131047024587 | success |
| Universal executable | 131047313159 | success |
| Packaged smoke (macos-15-intel) | 131047498771 | success |
| Packaged smoke (macos-15) | 131047498871 | success |

This confirms the existing native ARM/Intel checks, universal package and both
packaged native smoke jobs passed on merged main. Conclusions are GitHub's
hosted results, not numeric exits or a new local runtime test.

## Same-run artifact identity

| Metadata field | GitHub-reported value |
| --- | --- |
| Artifact | [13924514656](https://github.com/Kaungkhant97/switchbridge/actions/runs/43178062662/artifacts/13924514656) |
| Name | `switchbridge-macos-universal-e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab` |
| Source / branch | `e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab` / `main` |
| Run | `43178062662` |
| Size | `482294` bytes |
| Created | `2026-11-12T07:50:15Z` |
| Expires | `2026-11-26T07:50:13Z` |
| Expired | `false` at inspection |
| Artifact ZIP digest | `sha256:d587e05ee1ee660e3ef17fde7157cade8f128d65584d28d056ca7d913b0f4a98` |

Source: [run artifacts metadata API](https://api.github.com/repos/Kaungkhant97/switchbridge/actions/runs/43178062662/artifacts).
The digest identifies the uploaded GitHub artifact ZIP; it is not the inner
tar.gz or executable digest. No new download, extraction, independent checksum
computation, local execution, rebuild or workflow dispatch was performed for
this report. Inner archive/executable hashes are not exposed by this metadata.

Older `hosted-ci.md` / `.json` evidence establishes independently downloaded
and locally verified bytes, including a synthetic 60-frame transport/decode,
for source `ce8aeac1eab18cd54c494e7b8d8445da061a7e66`, run `43141901934` and
artifact `13909373980`. That proof remains specific to those older bytes and is
not a local reexecution of the merged-main artifact. The intermediate PR-head
metadata report `latest-ci-status.md` likewise concerns a separate run/artifact.

Physical projector observation, controlled console reconnects and attributable
live Switch capture remain pending. Release behavior remains unselected; this
passing CI result does not select or create a release. Only this report was
written for this assignment; no source/workflow edits, remote mutations,
commits, tags, releases or subagents were used.
