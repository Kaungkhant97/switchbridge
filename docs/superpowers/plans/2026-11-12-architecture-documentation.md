# SwitchBridge Architecture Documentation Implementation Plan

> **For agentic workers:** Execute task-by-task in this Core session with the existing paired Claude reviewer t24; the user explicitly limits the pair to one child agent. Final authoring waits for merged main and parent t9 review of the consolidated plan. Do not dispatch additional agents.

**Goal:** Document the architecture and separate data flows of the exact merged SwitchBridge implementation, with source evidence and independent semantic and visual review.

**Architecture:** Use C4 context and container views for static structure, and separate process/data-flow views for prerecorded transport, USB evidence, and CI artifact delivery. For this SwitchBridge model, represent the Rust CLI as one application container because these commands are roles of the same local application; this is a scoped modelling decision, not a universal C4 rule that shared binaries imply one container. Distinguish command roles, modules and concurrent process instances explicitly.

**Tech Stack:** Markdown, portable Mermaid, Archify typed JSON and standalone HTML/inline SVG; existing Rust source, GitHub Actions YAML and packaging scripts as evidence.

**Spec:** User architecture-documentation assignment to t23 from t9 on 2026-11-12; sibling LEA-7/8/9/10 plans listed below are dependencies, not implementation work assigned here.

## Global constraints and present status

- PLAN ONLY. No final diagrams, runtime changes, commits, pushes, PR merge, publication, comments, tags, ticket changes, or credentials changes are authorized by this assignment.
- Own this plan and future `docs/architecture/` artifacts. Researcher t24 owns only `docs/research/2026-11-12-c4-dataflow-practices.md` in its own worktree. Preserve all other worktrees and files; root t9 alone maintains primary `PLAN.md`.
- Lead worktree: `.worktrees/architecture-docs`, branch `docs/architecture-main`, source `ce8aeac1eab18cd54c494e7b8d8445da061a7e66` (PR2 head).
- Remote main observed 2026-11-12 at `460e8e8f6122bd748fd6d6bba1e7073e163dec27`: library scaffold plus merged LEA-9 report, without CLI, transport, USB implementation or workflow. Primary checkout's local main was `93e61ea`; it is not the remote-main snapshot.
- PR2 is OPEN, base `main`, `mergedAt: null`, `mergeCommit: null`. Green CI proves no merge. Revalidate these observations at execution.
- CI run `43141901934` on PR2 head succeeded across five expanded check/package/smoke jobs; that is host software evidence, not console or projector evidence.
- Live console USB video is UNVERIFIED. The feasibility report's NO-GO concerns direct-USB product commitment under present evidence, not a proof of impossibility.
- No invented backend, cloud runtime service, database, API, stream endpoint, capture source, durable Release pipeline, or signing/notarization.

## Review focus

1. A reader opens main before PR2 merges: implementation claims must be labelled PR2-only until the exact merge is observed and source is pinned.
2. A reader interprets sender success as full delivery: describe clean packet-boundary EOF and absent ACK/digest/end marker; full trailing-packet omission is not detected internally.
3. A reader interprets two CLI invocations as two services: keep one C4 application container and show sender/receiver as instances in the runtime dataflow.
4. A reader interprets inventory or local playback as console capture: keep USB evidence, synthetic media and human hardware acceptance distinct.
5. A reader downloads an artifact before smoke completes: artifact upload precedes smoke; require an entirely passing run and exact source/hash evidence.

## Proposed artifact inventory — none delivered yet

All paths below are relative to the future documentation worktree. Five diagram stems are fixed: `c4-context`, `c4-container`, `dataflow-prerecorded-loopback`, `dataflow-usb-evidence`, and `workflow-ci-artifacts`.

| Proposed path | Responsibility |
| --- | --- |
| `docs/architecture/README.md` | Reader entry point, audience/view selection, pinned source, status, legend conventions and maintenance ownership |
| `docs/architecture/evidence.md` | Node/relationship/scenario-to-source matrix with immutable commit URLs and test anchors; evidence limitations |
| `docs/architecture/c4-context.md` | Context purpose, portable Mermaid source, interpretation and unsupported capture callout |
| `docs/architecture/c4-container.md` | Single CLI application container, external dependencies, module explanation without a separate component diagram |
| `docs/architecture/dataflow-prerecorded-loopback.md` | Sender/receiver instances, input/output file stores, SWB1 flow, validation and failure behavior |
| `docs/architecture/dataflow-usb-evidence.md` | Snapshot and diff scenarios, OS subprocesses, privacy boundaries, evidence limitations |
| `docs/architecture/workflow-ci-artifacts.md` | Source-to-archive-to-smoke artifact lineage, separated from product runtime |
| `docs/architecture/diagrams/<stem>.json` | Exact Archify candidate specification; type architecture for C4, dataflow for runtime/USB, workflow v2 for CI |
| `docs/architecture/diagrams/<stem>.html` | Standalone HTML with inline SVG built from the accepted JSON |
| `docs/architecture/review/manifest.json` | Pinned source, five stem/type mappings, source/artifact SHA-256, receipt and screenshot paths, review state |
| `docs/architecture/review/<stem>.validation.json` | Final showcase validation receipt |
| `docs/architecture/review/<stem>.delivery.json` | Deliver acceptance receipt; renderer snapshot path/hash recorded |
| `docs/architecture/review/<stem>.visual-check.json` | Captured visual-check receipt; generated sidecars/screenshots stay beside HTML and are indexed by actual returned paths |
| `docs/architecture/review/paired-review.md` | Imported reviewer findings, dispositions, exact source/spec/artifact hashes and actual visual inspection outcome |

Mermaid lives in fenced blocks in the five Markdown views, avoiding a second independently maintained `.mmd` copy. A separately saved SVG export is optional only if requested; inline SVG is already in HTML. Renderer-generated files are recorded by actual receipt paths rather than guessed filenames. The research note is preserved from t24's worktree only after reading it and checking its citations; its existence alone is not research acceptance.

## View contract

Every view has a title, named audience and question, exact source SHA, scope, status (implemented at that SHA / proposed / unverified), legend, element descriptions and labelled directional relationships. Names stay consistent between Markdown, Mermaid and Archify. Labels identify actual data/action and mechanism; color is supplemental, with shape/text/status making meaning readable in both themes. C4 system/container boundaries, host/process boundaries and DFD trust boundaries have separate labelled meanings. Draw a trust boundary only with a stated change in trust; a file, subprocess or redaction step alone does not prove a new security zone. Files are file stores, not databases. External OS commands and playback tools remain outside the SwitchBridge executable. Each DFD's adjacent context table records process language/permissions/isolation/input validation, store type/permissions/controls, external ownership and flow payload/mechanism; unknown OS permissions remain explicit rather than inferred. Number scenario data movements and describe responses only where the implementation has them, including no application ACK for transport.

The README legend states that ordinary caller-selected media/evidence files are shown as external file artifacts in this C4 scope and as data stores in the DFD; this is a modelling choice, not a claim that C4 excludes owned storage containers. Snapshot paths are supplied by the caller; diffs go to stdout unless redirected. No automatic raw-evidence persistence is implemented.

Boundary assessment entries must cover: caller paths/content entering validation; OS-command stdout entering supported redaction; any local process connecting to the loopback listener (in scope, no authenticated peer identity); raw-to-redacted data as a privacy transformation with residual identifiers possible; and Actions-produced artifacts entering a contributor's local filesystem as externally obtained executable bytes. Record which entries are validation/privacy/provenance boundaries and which demonstrably change trust zones; do not elevate the former into security isolation claims. Checksums establish integrity relative to a trusted expected hash, not signer authenticity.

| View | Audience and purpose | Required content and boundaries |
| --- | --- | --- |
| C4 context | New contributors and reviewers: what does SwitchBridge currently do? | Operator uses SwitchBridge for prerecorded loopback transport and USB evidence. macOS inventory facilities are external dependencies. Files/tooling are summarized in prose at this level; no implementation modules or process-instance detail. Console capture appears only in a visibly isolated UNVERIFIED scope callout, with no implemented video arrow. |
| C4 container | Maintainers: what executable units implement the system? | One std-only Rust CLI application container inside SwitchBridge; local input/output files and macOS facilities outside the executable. A module table names `main`, `transport`, `framing`, `usb` as Rust code-organization modules, with no C4 component claim. `send` and `receive` are invocation roles of that same application. No separate component diagram unless later code complexity creates a concrete reader question and t9 approves the scope change. |
| Prerecorded dataflow | Developers and testers: which bytes move and where can they fail? | Local Annex B file -> sender instance -> SWB1/TCP numeric loopback -> receiver instance -> create-new output file. Mark each process and file boundary; local input/socket bytes are validation inputs, not proof that local peers are trusted. 64 KiB sender chunks, 24-byte network-order header, 1..1 MiB accepted payload, zero-based consecutive sequence, initial Annex B prefix, chunk elapsed timestamps. Playback is an explicit post-completion external `ffplay` step from the saved file, not live rendering or a stream endpoint. |
| USB evidence dataflow | Hardware investigators/privacy reviewers: what is collected and shared? | Operator invokes snapshot -> `system_profiler` type detection/inventory plus `sysctl` model and `sw_vers` version -> supported private-value redaction -> create-new snapshot. Separate before/after snapshots -> bounded UTF-8 reads -> validate/share known private values across both -> redacted diff -> stdout. Raw OS output stays a process input; saved file is redacted, not certified anonymous. Mark human privacy review before any sharing as manual practice, not an implemented upload gate. |
| CI artifact workflow | Contributors downloading binaries: where do artifacts come from? | Pinned source selected by workflow -> parallel ARM/Intel check -> dual target builds/lipo/archive/checksums/upload -> parallel downloaded packaged smoke. Upload occurs before smoke; show whole-run success as download-selection requirement. Actions retention 14 days; no Release publication. CI infrastructure boundary is distinct from runtime and hardware. |

Transport error branches must include invalid endpoint, missing/short/non-Annex-B input, connect/write/read failures, malformed or truncated framing, invalid lengths/sequence, existing output refusal, and handled-error partial-output removal attempt. The original failure survives cleanup failure, which is separately logged. The 10-second timeouts apply to connect and connected socket operations; listener accept can wait until connection/interruption. Do not depict successful cleanup under abrupt process death or guaranteed detection of whole trailing-packet loss.

USB branches include unsupported OS, unavailable profiler type, command failure/empty/oversize output, malformed/continued private fields, empty/oversize/non-UTF-8 diff inputs, and overwrite refusal. Collection's 16 MiB stdout check happens after subprocess output collection; do not claim a bounded subprocess-memory guarantee. Human three-session reconnect evidence and console classification are pending outside the implemented data path. CI failure stops dependent jobs; neither a checksum nor matching local transport hashes proves hardware capture, signer identity, or projector mirroring.

## Evidence inputs and coordination

Initial source anchors are PR2 `ce8aeac`; replace them with verified merged-main anchors before diagram delivery:

| Source | Claims/scenarios to anchor |
| --- | --- |
| `src/main.rs:9`, `:19`, `:45` | Command dispatch, 10s connected I/O behavior, one accepted connection, create-new output and cleanup |
| `src/transport.rs` (`validate_endpoint`, `send`, `receive`) | Numeric loopback, chunking, prefix validation, sequence, EOF and timestamps |
| `src/framing.rs` (`read_packet`, `write_packet`) | SWB1 header, limits, truncation and malformed-input behavior |
| `src/usb.rs` (`run`, `collect_inventory`, `inventory_diff`, `write_inventory`) | OS collection, supported redaction, two-input privacy handling, file limits and create-new policy |
| `tests/framing.rs`, `tests/transport.rs`, `tests/network.rs`, `tests/cli.rs`, `tests/usb_cli.rs` | Test names for the above claims; tests are software evidence, not hardware observations |
| `.github/workflows/macos.yml`, `scripts/package-macos.sh`, `scripts/smoke-packaged-macos.py` | Exact workflow dependency graph, source selection, build/archive/upload/download/smoke lineage |
| `docs/macos-playback.md`, `docs/usb-baseline.md`, `docs/research/stock-switch2-usb-feasibility.md` | Manual usage, physical evidence gates, privacy limitations and feasibility conclusion |

At execution record immutable `https://github.com/Kaungkhant97/switchbridge/blob/<SHA>/<path>#L<line>` anchors plus symbols, test names and explicit observed/proposed status for every meaningful node and edge. Recompute line numbers from pinned source; line references above are starting pointers. Capture CI URL `https://github.com/Kaungkhant97/switchbridge/actions/runs/43141901934` with its tested head SHA and distinguish it from any future post-merge run.

Sibling plans (read-only coordination, no duplicated implementation):

- LEA-7: `.worktrees/plan-lea7/docs/superpowers/plans/2026-11-12-lea-7-transport-closeout.md`; sender success is not receiver ACK, missing whole terminal packets can escape protocol detection.
- LEA-8: `.worktrees/plan-lea8/docs/superpowers/plans/2026-11-12-lea-8-usb-baseline.md`; three human-controlled HOME disconnect/connect sessions and privacy review remain pending.
- LEA-9: `.worktrees/plan-lea9/docs/superpowers/plans/2026-11-12-lea-9-feasibility-gate.md`; descriptors/traffic/live frames absent; no fabricated USB endpoint. MTP saved media, network GameChat and camera input are separate capabilities.
- LEA-10: `.worktrees/resume-cicd/docs/superpowers/plans/2026-11-12-lea-10-macos-cicd.md`; optional Release extension is undecided/unapproved, and moving synthetic image on a physical mirrored projector needs human observation.

## Task 1: Verified research and plan review (before authoring)

**Inputs:** User requirements, source inspection, sibling plans, t24's primary-source research.
**Produces:** Reviewed plan with source-backed C4/DFD conventions; no diagrams.

Lead independently verified these primary pages on 2026-11-12 while t24 researches the fuller note:

- [C4 container abstraction](https://c4model.com/abstractions/container): application and storage abstractions are distinct from deployment technology. The one-application decision here reflects SwitchBridge's local CLI responsibilities and invocation roles; neither binary reuse nor packaging alone universally determines C4 container boundaries. C4 also permits filesystem containers.
- [C4 diagram review checklist](https://c4model.com/diagrams/checklist): require readable scope/type, element responsibilities and relationship intent/direction, plus an understandable legend.
- [Microsoft DFD process guidance](https://learn.microsoft.com/en-us/training/modules/tm-create-a-threat-model-using-foundational-data-flow-diagram-elements/2-process-the-task-element) and [data-flow guidance](https://learn.microsoft.com/en-us/training/modules/tm-create-a-threat-model-using-foundational-data-flow-diagram-elements/5-data-flow-the-data-in-transit-element): processes can originate flows, including process-to-process and process-to-store interactions. Stores communicate through a process, never through a direct store-to-store flow. Process context includes input handling and permissions; C4 container identity and DFD task identity answer different questions.
- [Mermaid C4 syntax](https://mermaid.js.org/syntax/c4.html): official syntax is documented; no current experimental warning was present in the page checked. Portable flowcharts remain this plan's chosen representation, subject to target-viewer validation.
- [Microsoft trust-boundary guidance](https://learn.microsoft.com/en-us/training/modules/tm-create-a-threat-model-using-foundational-data-flow-diagram-elements/6-trust-boundary-the-trust-zone-change-element): boundaries represent changes between trust zones; justify each separately from ordinary process/host groupings.
- [Mermaid accessibility](https://mermaid.js.org/config/accessibility.html) and [flowchart syntax](https://mermaid.js.org/syntax/flowchart.html): add `accTitle`/`accDescr`, labelled edges and explicit legends; record the exact Mermaid version actually used for validation rather than assuming the website version matches the target viewer.

Research/source verification and the observer run independently now. After both authoring gates, the lead authors views while t24 reviews completed semantic units; the same reviewer later checks exact rendered outputs. This is the complete allowed pair, not five additional authoring agents.

- [ ] Read t24's research note in `.worktrees/architecture-research`; verify official C4 abstractions/notation/review checklist, official DFD process/store/flow/trust-boundary guidance and any Mermaid syntax recommendation against cited URLs/access dates. Record unavailable sources rather than filling gaps from memory.
- [ ] Incorporate research findings into this plan and request t24's independent scope/semantic review of this exact plan; resolve material findings without broadening the single-child limit.
- [ ] Send t9 PLAN READY with plan/research paths and observer handle; wait for t9 consolidated-plan review before final authoring. Approval is not inferred from elapsed time or CI.

## Task 2: Observe merge and pin source (independent of research)

**Inputs:** PR2 state and exact main SHA. **Produces:** Immutable source baseline and preserved planning files.

- [ ] Keep observer `t25` / `architecture-merge-watch` alive; use `hc wait t25` to receive lifecycle updates. Script `/tmp/switchbridge-pr2-observer-t23.py` polls every 30s for up to 24h, bounds each network call to 20s, logs `/tmp/switchbridge-pr2-observer-t23.jsonl`, and notifies t23/t9. Re-arm on bounded expiry if still requested; this is an observer program, not a second agent.
- [ ] Require PR `state=MERGED`, non-null `mergedAt`, base `main`, merge commit SHA and remote main SHA. Closed-unmerged stops the observer and is reported as such. Network errors preserve uncertainty and retry within the bound.
- [ ] Fetch only into this repository's remote refs after observed merge, inspect exact commits and verify merge commit ancestry of the observed main SHA. Pin that observed SHA as the documentation baseline even if main subsequently moves; recheck source differences from PR2.
- [ ] Preserve this worktree and research note. If squash/rebase history prevents fast-forward, create a fresh `docs/architecture-merged-<shortSHA>` branch/worktree from the pinned main SHA and copy only this plan plus the reviewed research note. Do not reset, discard, switch the primary checkout, or rewrite siblings. Report the new path to t9.
- [ ] Record actual source/merge/head SHAs, timestamps and scope differences in `evidence.md` only once both authoring gates are satisfied.

## Task 3: Author evidence and five reviewable views (after BOTH gates)

**Inputs:** Merged pinned source, approved plan, verified research. **Produces:** Inventory's Markdown/Mermaid and five JSON candidates.

- [ ] Build evidence matrix first; check all five Review Focus conditions against exact source and test names. Confirm every arrow's payload/action, direction, protocol and implementation status.
- [ ] Write the README and five Markdown views using the contracts above; compare all Mermaid relationships to the same evidence matrix. Use ordinary Mermaid flowcharts with explicit C4 labels for portability across target Markdown viewers; verify the selected syntax and target viewer support using official docs. Do not assume Mermaid C4 syntax is experimental without current evidence.
- [ ] For each Archify type, read only its schema, `schemas/common.schema.json`, and one matching example first. The next authoring action writes that candidate before inspecting renderer internals. Use new domain IDs, at most 12 primary nodes per view, automatic routing and `meta.quality_profile: "showcase"`; omit animation and optional visual presets.
- [ ] After the first candidate exists, run Archify's packaged update checker once as the skill directs. Validate after every candidate edit. Add routing controls only for a diagnosed geometry issue; retain meaningful relationship labels.
- [ ] Ask existing t24 for independent semantic review of the exact pinned source plus candidate/Markdown hashes. Findings must identify evidence, affected view and correction; author fixes before final acceptance. Do not treat render validity as semantic correctness.

## Task 4: Deliver and visually review exact artifacts

**Inputs:** Semantically reviewed candidates. **Produces:** Frozen local artifacts, receipts and paired review record; no publication.

- [ ] Run `node /Users/kaungkhantthu/.agents/skills/archify/bin/archify.mjs validate <type> <candidate.json> --quality showcase --json`; save final receipt. Acceptance requires all 9 artifact checks, zero composition errors and zero warnings; four basic checks do not qualify.
- [ ] Run `deliver <type> <candidate.json> <output.html> --quality showcase --json` with that same executable and save receipt. Nonzero exit is failure; do not inspect stale last-good HTML as if it were the failed candidate. Freeze accepted JSON bytes and record source/spec/artifact hashes in manifest.
- [ ] Run `visual-check <output.html> --json` on each successfully delivered HTML. Save receipt and actual screenshot/sidecar paths. Check 1440x900, 1600x1000, 1920x1080 and 2048x1320; require page scroll width/height within viewport at each desktop size without clipping or hidden overflow.
- [ ] Lead and paired t24 actually open/inspect the captured light/dark images at smallest/largest sizes. Check readable labels, hierarchy, balanced space, complete boundaries, arrow direction, collisions, unrelated-node crossings and legend/status clarity. If a reviewer cannot inspect images, record visual review pending; never substitute an automated pass.
- [ ] Repair diagnosed defects, regenerate/validate/deliver/recheck, and review updated exact hashes. Stop and report unresolved diagnostics if two consecutive repairs fail to improve the best diagnostic count, per Archify. Final pending/failed reviews prevent a delivered-success claim.
- [ ] Validate Markdown links and Mermaid rendering, compare all portable relationships with accepted JSON, and inspect `git diff --check` plus the complete scoped diff. Runtime tests need not be rerun for documentation alone; cite existing test evidence with SHA/date and do not report it as newly executed.
- [ ] Deliver local paths, pinned source, receipts/hash manifest and paired findings to t9. Keep research pair available. Commit/push/publication remain separate actions requiring authorization.

## Maintenance and peer-review acceptance

Changes to CLI dispatch, transport/framing, USB/redaction, packaging/workflow or hardware-evidence status trigger reinspection of the matrix and affected views. The change author updates evidence first, regenerates affected portable/Archify artifacts, repeats semantic plus actual visual review and records new hashes/source SHA. Scope/status labels retain historical uncertainty until new evidence supports a change. Source-code comments and existing usage/feasibility docs remain authoritative for their respective details rather than being copied wholesale.

Acceptance uses the official C4 diagram review checklist and the paired research note's DFD checklist as the semantic rubric, separately from Archify's layout checks. Require explicit element abstraction/responsibility/technology, direction-matching relationship labels/protocols, and legend meanings for shapes, colors, borders, line styles and arrowheads. Acceptance also requires: both authoring gates recorded; all five views clearly answer their named question; all nodes/edges/scenarios trace to source or explicitly proposed/manual/unverified status; one CLI container and correctly labelled process instances; separate media/USB/CI boundaries; supported and failure paths covered; no unsupported capture/service/release claims; Mermaid/JSON semantic parity; five successful showcase/delivery/visual-check receipts; actual paired semantic and visual inspection on recorded hashes; all material findings resolved or candidly blocked. The plan is the only deliverable at this stage.
