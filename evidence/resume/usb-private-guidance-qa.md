# F1/F2 USB private-guidance QA

**Verdict: PASS. No remaining actionable findings in the bounded patch.** Independent reviewer t20, 2026-11-12. Reviewed sibling `.worktrees/usb-doc-privacy`, branch `fix/usb-private-guidance`, base `c23c2cf426c30923831b91665d0e746660c9b763`. Three modified manuals and one exact imported original audit only. This is approval of the documentation correction's observed quality, not a new hardware acceptance result.

## Exact reviewed bytes

| File | SHA256 |
| --- | --- |
| `README.md` | `f75dee49b8903ec0f377744a4fc03ac97d846994b636163fc6df92848fa3b706` |
| `docs/usb-baseline.md` | `f7941867ed2cab4423baaf5c7227c4d04aad577333d1567d197f563763b3cc6f` |
| `evidence/resume/README.md` | `f22527cbe3a55679a668eb1a5d12b504b1a7fdba2f9c57882144f52e5dc1331f` |
| `docs/research/2026-11-12-postcompletion-claude-audit.md` | `91d4964d6811435794994de98dcd1f60df384f6d74e798675105802d4d64715a` |

Imported audit bytes independently compare equal to the original file in `.worktrees/claude-final-audit`; its findings/recommendations were not edited to claim resolution retroactively.

## Checks and dispositions

| Area | Independent result |
| --- | --- |
| F1 collection destination | README and primary baseline now use ignored `.evidence/lea-8/ACTUAL-UNIQUE-SESSION`, with fresh unique label, dedicated shell, restrictive umask, exclusive session mkdir, chmod and ignore guard before collection. README setup prefix exactly equals baseline setup. No direct `evidence/trial-*` collection remains in the modified procedures. |
| Stop guards and permissions | Executed the exact setup block in temporary synthetic Git directories using `/bin/sh`, `/bin/bash`, `/bin/zsh`. Fresh setup exits 0; directory is 0700 and a synthetic generated file is 0600. Existing session mkdir exits 1 before a continuation marker; missing ignore rule exits 1 before a marker; obstructed `.evidence` parent exits 1 before a marker. No profiler, switchbridge, USB or device command invoked. |
| Noclobber | In each shell, `set -o noclobber` makes a synthetic attempted overwrite fail with exit 1; original sentinel bytes remain unchanged. This verifies shell redirection behavior without introducing repository tests or executing collection. |
| Trial failure handling | Instructions require commands individually, immediate actual exit records, stop on failure, successful/nonempty snapshots, preserved failed attempts and a new name for retries. They do not present a successful diff command as enumeration PASS. |
| Hardware scope | Three controlled HOME disconnect/connect pairs, fixed cable/ports/firmware/peripherals, actual metadata/wait interval and attribution remain mandatory. Copy to PC is separately labelled; no inferred endpoint/classification, fixture identifiers or observed hardware result appears. |
| Privacy-reviewed handoff | Private originals/logs/metadata/failed attempts stay ignored and unchanged. Only explicitly reviewed sanitized copies enter `evidence/lea-8/<session>/`; reviewer/date/scope, sanitized transformation mapping and verified share hashes required. Further snapshot sanitization requires regenerated/reviewed share diffs. Sharing/Linear attachment remains authorization-gated. Links lead to the stricter blank USB template. |
| F2 navigation/status | Root README links architecture. Evidence index identifies PR2/PR3 merged, links actual architecture/paired review and post-completion audit, marks older deferred/current/open prose historical and retains hardware/release gates. |
| CI/source identity | Unchanged `merged-main-ci.md` remains `e1b81f7` / run `43178062662` / artifact `13924514656`, metadata only. Imported audit separately records `c23c2cf` / `43181069547` / artifact `13925903782`, metadata only. Older `ce8aeac` / `43141901934` / downloaded artifact `13909373980` retains its independent-byte/60-frame proof. New index explicitly distinguishes these; no later artifact inherits older inner hashes or local execution. These checks compare the supplied exact records, not a fresh remote availability query. |
| Architecture package | All 75 architecture manifest entries independently SHA256-match. No `docs/architecture/` delta. Architecture manifest and source baseline are byte-identical to base HEAD. |
| Frozen source manifest | Exactly 18/20 match; only `README.md` and `docs/usb-baseline.md` intentionally differ as later manuals. Evidence index explicitly explains historical pinning instead of rewriting old provenance. `verification-metadata.json`, architecture source baseline and prior CI report remain unchanged. |
| Links / whitespace | All 54 repository-relative Markdown link occurrences in the four scoped files resolve. `git diff --check` exits 0. External run links/IDs were cross-compared with exact locally supplied reports; no remote request needed. |
| Runtime scope | `git diff --name-only -- src tests scripts .github Cargo.toml Cargo.lock` is empty; no runtime/code/test/workflow/package change. |

## Limits and handoff

Safe synthetic shell probes ran in automatically removed temporary directories; they test only the actual documented guards and redirection, not device collection. Dedicated-shell `exit 1` behavior is deliberate and documented. Snapshot/diff commands were not executed, and no physical trial or redaction completeness is claimed. This QA adds no new feature, test suite, dependency, rebuild, credential or approval requirement.

Historical architecture and source manifests remain accurate statements about their pinned snapshots; changing two later manual files does not require falsifying that old proof. Future changes to architecture content itself require its own review, which this patch does not make.

Only `evidence/resume/usb-private-guidance-qa.md` in plan-lea7 was written. No sibling edits, builds, source changes, commits, remote operations or subagents. Coordinator may continue the reviewed documentation publication workflow under existing authorization; physical acceptance remains pending.
