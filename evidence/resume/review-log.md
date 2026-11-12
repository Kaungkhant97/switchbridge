# SwitchBridge integration review and acceptance audit

Date: 2026-11-12. Branch: `feat/resume-integration`.

## Local verdict: CONVERGED

Codex and Claude independently cleared the same frozen snapshot
`2db17bddf3b76ffb57184ad9d842f219fb96ab99`, with source snapshot
`d927173a9c76a381e2141b5ab6285f663653632c`. Neither final pass required an
implementation change. No critical/high/medium finding remains. This is local
source convergence, not completion of native Intel/hosted-CI or physical gates.

| Pass | Reviewer and artifact | Findings | Action and outcome |
| --- | --- | --- | --- |
| 1 | Claude, original workstreams | Four medium; low observations | Device-heading rejection, short-value corruption, missing original USB CLI dispatch, and historical proof provenance identified. Static-only limits recorded in [preliminary report](claude-preliminary.md). |
| 2 | Codex, `b3a02d8` | Three medium; no critical/high | Reproduced cross-input leakage, identifier corruption and heading rejection; all fixed with failing regressions. [Report](codex-round1.json). |
| 3 | Codex, `948a5cc` | No actionable findings | Independent feasible checks, artifact/source hashes and mental-revert tests passed. [Report](codex-round2.json). |
| 4 | Claude, `948a5cc` | One medium, three low, one info | Synthetic qualified UUID labels and colon-bearing headings reproduced/fixed; cleanup diagnostics and optimized verifier fixed; over-redaction documented. Legacy-profiler emission premise remained unobserved. [Report](claude-round2.md). |
| 5 | Codex, `2db17bd` | No actionable findings | Independently passed 43 tests, formatting/Clippy/actionlint/shellcheck; verified current source/QA/archive/executable hashes and architecture slices; four new USB tests fail when fixes are reverted. [Final report](codex-final.json). |
| 6 | Claude, `2db17bd` | No medium or higher; two speculative low, one info | Independently traced final changes, checked regression intent, source/documentation boundaries and log consistency. Static only. [Final report](claude-final.json). |

Three bounded implementation cycles completed. Long private-value embedding,
both diff directions, short-token/VID/PID preservation, qualified labels,
colon-containing serials, malformed private fields, and cleanup error reporting
have explicit regression coverage. Initial lipo ordering failure was reproduced
with the actual packaging command and corrected in script, workflow and docs.

Claude's final low observations concern conservative over-matching and unknown
non-heading key formats; no such profiler output was observed. They are recorded
as speculative limitations, with manual review required before sharing. Broad
key rewriting was not added without evidence, to preserve meaningful labels.
The info-only stale log name in metadata prose was corrected to `release-qa.log`;
the actual log path/hash, source hashes and executable/archive hashes did not
change. Review reports, this index and the PR draft are post-review evidence,
not changes to the reviewed runtime or source documentation.

## Independent execution boundaries

Parent `t9` and developer `t10` actually ran the complete 46-test suite including
all three TCP tests. Parent [release QA](release-qa.log) independently confirms
90.98% source-line coverage, all canonical checks, fresh universal packaging,
archive/internal checksums, executable mode, both architecture slices, native ARM
packaged TCP, actual host collector/self-diff, and byte-identical synthetic H.264
with 60 decoded frames. The video proof passes normally and under `python3 -O`.

Codex reviewer `t17` independently ran 43 non-network tests and static checks and
verified actual source/log/archive/executable hashes. Its sandbox denied TCP and
Herdr-core access; those denials were not bypassed. Claude reviewer `t11` reviewed
read-only and did not independently execute tests/builds or recompute hashes.
Its denied writes were not retried; parent preserved reports received through
Herdr messages. These are different model families, not two labels for one model.

## Updated remote baseline

A read-only remote check found [LEA-9 PR #1](https://github.com/Kaungkhant97/switchbridge/pull/1)
already merged. `origin/main` was fetched at
`460e8e8f6122bd748fd6d6bba1e7073e163dec27`. Local merge `a7d42c9` preserves its
four historical review records and retains the already-reviewed feasibility
report content. The only report difference versus remote main is its review
boundary sentence; no supported research claim was removed. All 20 reviewed
source/doc/helper hashes still match the frozen manifest after the merge.
Historical LEA-9 reviews apply to their exact prior report hash, not the current
runtime integration. The primary checkout and original source worktrees were
not edited.

## Requirement-by-requirement acceptance

| Requirement | Evidence and status |
| --- | --- |
| Herdr-managed agents in dedicated worktrees | Parent `t9`, implementation `t10`, Claude `t11`, Codex review `t17`; `hc` used with interactive agents and Codex `--no-daemon`. Dedicated integration/Claude/Codex-review worktrees retained. |
| LEA-7 bounded SWB1 framing and loopback CLI | Source, wire fixtures/malformed/order/boundary tests, actual TCP tests and release QA pass. Synthetic 85,967-byte input/output SHA256 match; FFmpeg decodes 60 frames. Chunk time is explicitly not frame time. |
| LEA-8 integrated snapshot/diff tooling | 25 USB unit tests, three USB CLI tests, actual host snapshot/self-diff pass; redaction and overwrite guards verified. Controlled console enumeration remains pending. |
| LEA-9 honest feasibility decision | BLOCKED/UNVERIFIED direct USB live capture decision and falsifiable physical evidence gate retained; already-landed research history preserved. No invented descriptors, identifiers or live frames. |
| LEA-10 local universal package | Actual ARM64/x86_64 slices, archive/internal checksums, executable mode and native ARM packaged TCP verified; [metadata](verification-metadata.json) records immutable source/binary/archive hashes, time and versions. |
| Native ARM/Intel hosted CI and downloadable GitHub artifact | Workflow configured and linted; official runner labels/action SHAs checked. **PENDING remote publication/authorized CI run**; no run or artifact URL invented. |
| Three console reconnects, actual USB gameplay, projector | **PENDING physical observation**, expressly retained by the goal. One host collector smoke does not satisfy these gates. |
| Two independent reviews and actionable findings | Final reports clear medium-or-higher; substantive findings resolved with red/green evidence; residual low/speculative limits recorded. |
| Local reviewable handoff | Commits, current package, metadata, reports and [PR draft](pr-draft.md) prepared. No pushes, PR creation, ticket changes, remote jobs or credential changes performed in this resumed work. |

The full thread goal is not claimed complete while hosted/native Intel CI is
unverified. The next authorized proposal is to push this reviewed branch and
open its prepared PR, which triggers the configured macOS workflow. The supplied
AGENTS.md external-action boundary requires explicit user approval for that
GitHub mutation. Physical acceptance remains separately pending regardless of CI.
