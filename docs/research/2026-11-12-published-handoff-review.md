# Independent review: published planning/evidence handoff `9113cd1`

**Reviewer:** t24 (Claude, paired architecture researcher).
**Date:** 2026-11-12. This was a read-only review; nothing in the reviewed commit was edited.

**What was reviewed:**

- Commit `9113cd17ca892f9401a90f2f2fcc5bb4a96b4738`, read from local git objects. Its parent is `ce8aeac`.
- Expected preserved artifacts:
  - architecture plan, sha256 `ab93a8fcd161437aeb18db56ed26157700052266cb3e003ab71dbae8a6bed63b`;
  - research note, sha256 `e03e08cefdd60ad5de3522f0f18614d8c884741f3ffbfa2a8f7e0a79d43242b5`.
- Remote state was checked with read-only `gh` queries. Nothing was mutated.

## Verdict

**READY as a planning/evidence-only handoff. No material (medium or higher) findings.**

Two LOW and three INFO observations follow. None blocks the handoff. The actual-merge gate is unchanged and still unmet.

## Checks performed

| Check | Result | Evidence |
| --- | --- | --- |
| Commit adds no runtime changes | **PASS** | `git diff --name-only ce8aeac 9113cd1 -- src tests scripts Cargo.toml Cargo.lock .github README.md` lists 0 files. The 19 changed files are docs, plans and `evidence/resume/*` only. |
| Preserved architecture plan and research bytes | **PASS** | `git show 9113cd1:<path> \| shasum -a 256` gives `ab93a8fc…bed63b` and `e03e08ce…3242b5`, the exact expected values. |
| Seven-document manifest (`evidence/resume/parallel-planning.json`) | **PASS, 7/7** | Each of these hashes matches its blob at `9113cd1`: the LEA-7/8/9/10 plans, the architecture plan, the research note, and `evidence/resume/cicd-assessment.md`. |
| Acceptance handoff manifest (`evidence/resume/acceptance-handoff.json`) | **PASS, 4/4** | The two closeout drafts and two trial templates match. |
| The "20 frozen source/doc/helper hashes" (roadmap line 168) | **PASS, 20/20** | `evidence/resume/verification-metadata.json` `sources_sha256` (snapshot `d927173`). All 20 match at `9113cd1`, and the metadata and provenance files are untouched by this commit. |
| Relative Markdown links in changed `.md` files | **PASS** | No broken relative links against the `9113cd1` tree. That includes the roadmap's `../../research/2026-11-12-c4-dataflow-practices.md` and the README's links to the roadmap and manifest. |
| Actual-merge gate | **UNCHANGED / NOT MET** | `gh pr view 2`: `state=OPEN`, `mergedAt=null`, base `main`, head `9113cd1`. Remote `main` = `460e8e8f6122bd748fd6d6bba1e7073e163dec27`, which matches the plan's observation. Roadmap lines 51, 143 and 156 and plan Task 2 still require an observed merge and a pinned actual main SHA. |
| Release/tag claims | **PASS** | `gh api …/releases` returns 0 and `…/tags` returns 0, which agrees with "No tag, GitHub Release…" in `review-log.md`, `pr-draft.md` and roadmap line 45. |
| Hardware/capture claims | **PASS** | No document claims a console reconnect, live USB capture, or projector observation. Hardware gates stay pending in the roadmap (lines 36, 41–43, 63–65, 157), `hosted-ci.md`, `pr-draft.md` and `review-log.md`. CI is described as host-software evidence only. |
| CI evidence identity | **PASS (scoped)** | `hosted-ci.md`/`hosted-ci.json` bind run `43141901934`, its five job IDs, artifact `13909373980` (expiry 2026-11-26) and the downloaded hashes to `source_sha = ce8aeac`. `pr-draft.md` says "Subsequent source changes need their own CI." |
| Source vs plan distinction | **PASS** | The roadmap's "Architecture" paragraph and Task 3 say the views come *after* the observed merge. Line 152: "No final C4/data-flow artifacts are claimed delivered". Every manifest entry has status "reviewed planning/research draft; not implementation". |

## Observations

**L1 (LOW): "current CI" is ambiguous now that the head has moved.**

- Roadmap line 156 says "current CI is green", and line 34 links only run `43141901934`, which ran at `ce8aeac`.
- A read-only query shows PR2's head is now `9113cd1`, with a separate successful run `43164099151` (`event=pull_request`, `headSha=9113cd1…`).
- The cited evidence still describes the runtime correctly, because all 20 frozen hashes are unchanged. But "current" now refers to a different run, and that run's artifact has not been independently QA'd. A commit cannot cite its own CI run, so the ambiguity is unavoidable at authoring time. Resolve it when the source is pinned.
- Recommendation: in the post-merge evidence matrix (plan Task 2/3), record:
  - (a) the QA'd run `43141901934` at `ce8aeac`;
  - (b) run `43164099151` at `9113cd1`, as status-only and not artifact-verified;
  - (c) any post-merge run at the actual merged SHA.

  Do not treat a later run's artifact as the verified artifact.

**L2 (LOW): the frozen metadata still lists hosted CI as pending.**

- `evidence/resume/verification-metadata.json` `pending_acceptance` still lists "Native Intel test and packaged smoke execution" and "Passing hosted Actions run and downloadable artifact link".
- This file is a frozen snapshot from `d927173`, and `evidence/resume/README.md` (Hosted CI verification) explicitly says the later evidence "supersedes earlier pending hosted/native Intel notes". That is acceptable, but a reader of the JSON alone could be misled.
- Recommendation: optionally note in the evidence matrix that this field is historical. Leave the frozen file unchanged.

**I1 (INFO): raw `.evidence/` is a manual practice, not code behaviour.**

The roadmap's `.evidence/` raw-collection wording (lines 41–43, 101–103) describes operator practice for physical trials. That agrees with the research note §3: `.evidence/` is a README/gitignore convention, the code writes no raw store automatically, snapshot paths are caller-chosen, and the diff goes to stdout. No conflict.

**I2 (INFO): the roadmap's dependency chart lacks accessibility metadata.**

The roadmap's Mermaid chart (lines 76–85) is a planning dependency chart, not one of the five architecture views. It has no `accTitle`/`accDescr`. The research checklist applies to architecture views, so this is non-blocking.

**I3 (INFO): this review covered consistency, not substance.**

- I did not re-verify the substance of the LEA-7/8/9/10 plans, the hosted CI logs, or the downloaded artifact bytes beyond their recorded hashes and claims.
- The test counts and coverage figures (46 tests, 90.98%) are recorded claims; this review did not re-run anything.
- The Linear ticket state ("In Progress", roadmap line 163) was not queried.

## Gate status after this review

- **Planning/evidence handoff:** ready, with no material findings.
- **Actual merge:** NOT observed. PR2 is open, and merge approval belongs to the user.
- **Architecture authoring:** blocked until the merge is observed and main is pinned. After that: the lead authors, then QA, then t24 does an exact semantic review and an actual screenshot review on recorded hashes.
- **Hardware and release decisions:** pending and unchanged.
