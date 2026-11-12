# CI/CD read-only assessment

Status: recommendation only; release approach and design approval pending.
Inspected 2026-11-12 in `feat/github-cicd` at
`ce8aeac1eab18cd54c494e7b8d8445da061a7e66`. No implementation or remote
mutations performed. This assessment is the only local write.

## Existing evidence

- `.github/workflows/macos.yml` runs on main pushes, pull requests and manual
  dispatch, with global `contents: read`, pinned official action SHAs and
  credential-free checkouts of the source SHA.
- Native ARM/Intel checks precede universal packaging; both native smoke jobs
  download that same SHA-named artifact, verify archive/internal checksums,
  executable mode and architecture slices, then execute sender/receiver TCP smoke.
- Run https://github.com/Kaungkhant97/switchbridge/actions/runs/43141901934
  reports success on the exact revision above (pull_request event).
- `scripts/package-macos.sh` produces exactly two outer assets:
  `switchbridge-macos-universal.tar.gz` and its `.sha256` file. Archive members
  are executable `switchbridge` and `SHA256SUMS`. No package changes are needed
  merely to attach these assets to a release.
- Cargo package is version `0.1.0`, edition 2024, publish=false; this is a CLI,
  not a deployed service or crates.io publishing task.
- Read-only GitHub queries show public repository, default branch `main`,
  no releases and no tags; no established release naming convention was found.
- Current docs describe unsigned, unnotarized development binaries and a 14-day
  Actions artifact lifetime. Some historical evidence still says hosted CI was
  deferred; it is historical, superseded by the verified hosted run above.
- Physical reconnect, attributable live capture and projector observation are
  distinct pending gates. Packaged TCP smoke does not establish those gates.
- Fast-context was attempted but unavailable (missing Windsurf API key); direct
  local inspection was used without changing credentials.

## Approaches and tradeoffs

1. **Recommended: version tags create draft GitHub Releases.** Extend the existing
   workflow with version tag triggers and a final release job depending on BOTH
   smoke matrix completions. Reuse the package from that same run. Maintainer
   reviews the draft before publishing. Adds durable distribution while keeping
   public publication deliberate, appropriate while physical acceptance is open.
2. **Version tags publish automatically.** Same trusted pipeline and same assets,
   but successful tag runs publish immediately. Less manual work; an accidental
   valid tag distributes publicly before release-note/hardware-scope review.
3. **Artifacts only.** Retain the proven workflow and document downloads; no
   release write permission. Smallest change, but artifacts expire after 14 days
   and users have no durable versioned release page.

## Proposed minimal implementation shape (not approved)

- Keep all existing check/package/smoke jobs and main/PR behavior. Add push tags
  matching `v*`; validate an exact accepted version syntax before expensive work.
  Broad GitHub tag globs alone are not version validation.
- Require the tag value to equal `v` + the Cargo package version, read with a
  structured metadata/parser approach; reject malformed/injected/mismatched refs.
  Recommend initially stable `vMAJOR.MINOR.PATCH` tags only; prerelease policy
  remains a decision. Do not bump Cargo version or create a tag in this task.
- Require the tagged commit to be reachable from `origin/main` before release
  publication. This validates ancestry, not branch-protection or reviewer approval;
  a tag still trusts repository writers and the workflow committed at that tag.
  The first version tag should point at main AFTER the release workflow lands.
- Scope `contents: write` to the final tag-only release job; retain global read
  permission. No `pull_request_target`, privileged `workflow_run`, PAT, secrets,
  signing identity, notarization or deployment environment is required.
- Release job needs completed smoke jobs, downloads the exact SHA-named artifact
  from the same run, rechecks the outer checksum and expected asset set, then
  attaches those unchanged bytes. Never rebuild for release or select latest
  branch artifacts. This avoids cross-run promotion ambiguity and 14-day expiry
  races before release creation.
- Preserve existing asset names so the checksum's filename continues to resolve.
  The release tag/page carries the version; versioned archive names would be a
  separate optional requirement and must regenerate matching checksum content.
- Record tag, full source SHA, workflow/run URL, toolchain and archive digest in
  release notes or a run summary. This is traceability, not a cryptographic
  attestation. Optional GitHub build attestation adds OIDC/attestation permissions
  and action surface; defer unless explicitly wanted.
- Serialize release attempts per tag without canceling an active release run.
  Fail closed when a release already exists; never overwrite published assets or
  republish silently. Failed partial draft upload recovery can be documented as
  an explicit maintainer action rather than destructive automatic replacement.
- Describe releases as unsigned experimental prerecorded transport, with physical
  Switch capture/projector gates still open. Draft publication is a human choice,
  not evidence those gates passed.

## Decisions needed

1. Select draft-on-tag (recommended), automatic publication, or artifacts-only.
2. Approve stable `vMAJOR.MINOR.PATCH` matching Cargo version and main ancestry;
   otherwise specify prerelease/tag/source policy.
3. Approve exact existing two-asset names, run/SHA/checksum traceability without
   signing or attestation, and fail-closed existing-release handling.

The selected approach must be written as a short design and approved before any
implementation. No user selection is inferred by this assessment. Parent is
already collecting the primary release preference; no duplicate question sent.

## Testable acceptance criteria for the release approaches

- Existing ARM/Intel fmt/test/clippy and universal/extracted smoke behavior stays
  intact; PR/main/manual runs cannot enter the release job or receive its write
  token. No PR-derived input is interpolated into shell source.
- Valid selected version tag matching Cargo and permitted ancestry passes;
  malformed tags, version mismatch and disallowed source commit fail before any
  release mutation. Tests include shell-metacharacter-shaped refs where valid
  Git ref syntax permits them, and prerelease cases per selected policy.
- Failure of either native smoke blocks release creation. The release consumes
  the same archive verified by both native smoke jobs, with no rebuild.
- Release assets are exactly the archive and its correctly named checksum;
  downloaded release bytes satisfy the outer checksum and extracted internal
  checksum, executable mode and both architecture slices.
- Draft mode creates only a draft; automatic mode publishes only if selected.
  Existing release/tag reruns do not overwrite assets or change publication state.
- Workflow passes actionlint; any changed helper passes relevant shell/python
  checks and focused positive/negative version/promotion tests. Existing suite
  remains passing. Live tag/release end-to-end validation requires separate
  explicit approval; static/mock tests must not be reported as live release proof.
- Documentation covers trigger, version match, assets/checksums, run provenance,
  unsigned limitations, retry policy and manual publication if draft is chosen;
  physical gates remain pending and no secrets are invented.
