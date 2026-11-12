# Final architecture review record

Source: merged main `e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab`. Reviewed on 2026-11-12. The five view specifications and artifacts are bound by [manifest.json](manifest.json); exact hashes live there rather than being manually repeated in this summary.

## Independent outcomes

- **Developer/lead t23:** authored the views and source matrix; actually inspected all 20 final Archify PNGs at 1440×900 and 2048×1320 in light/dark. [Lead visual record](lead-visual-review.json): passed.
- **QA t20:** independently checked source pin and frozen hashes, all five specifications/HTML/receipts, Mermaid/SVG block hashes and edge parity, links/anchors and all 20 actual screenshots. [Byte-preserved QA report](qa-review.md), SHA256 `9084e50def80cb959c703cbe5756f865d188496788933c7e57de40a4b896f2cf`: content QA PASS, with final metadata assembly explicitly pending at the time of that report.
- **Paired Claude t24:** source-backed semantic PASS, no material findings; actual visual review of all 20 current Archify PNGs PASS with low observations. [Byte-preserved peer report](peer-review.md), SHA256 `9e3698882311783d0aee300b8a5ca12dc3da5adcd751da431bc1e9dc099031e9`.

The reviewer initially encountered a sibling-worktree image-read permission prompt. It was denied/cancelled; no global read permission was granted. The coordinator authorized exact copies under the reviewer's own ignored `.evidence/architecture-review-e1b81f7/`. A source-relative copy manifest verified SHA256 equality; t24 checked 73/73 copies and recorded actual reviewed source/copy identities. This copy manifest is private review-input evidence, not an alternate documentation source.

## Findings and dispositions

| Finding | Disposition |
| --- | --- |
| QA V1 USB long command label touched its icon | Resolved: static node says `usb-diff`; BEFORE/AFTER meaning remains in file/flow/prose and metadata. All affected validation/delivery/capture identities rechecked. |
| QA A1 imprecise USB/source-SHA/playback line anchors | Resolved: collector 48, workflow source assignment 19 and ffplay invocation 84; C4 source-link changes regenerated and independently rechecked. |
| Metadata assembly: stale render-handoff hash / absent paired-review link | Resolved in final assembly: this record closes the link, and the manifest is regenerated last over exact final files. Historical review reports retain their truthful earlier pending state. |
| Peer V1 small 1440 data-flow labels in sparse/tall stage columns | Low, nonblocking; retained as a known visual limitation. Actual reviewers could read the labels and found no clipping or ambiguity. A shorter canvas is a possible future refinement; no additional geometry churn is claimed complete. |
| Peer V2 context legend badge close to following swatch | Low, nonblocking; no overlap. Container spacing is clean. Retained in exact peer report. |
| Peer V3 some Archify tags appear only on hover/fine detail | Low, nonblocking; command arguments, CI details and timing/limits remain explicit in static cards and portable Markdown. No static screenshot is claimed to show hover-only content. |
| Peer V4 compact CI legend/icon/title and lane-divider label placement | Low, nonblocking; direction and dependency order remain clear. Retained in exact peer report. |

No critical/high/medium issue remains. Renderer geometry acceptance and perceptual review are separate: automated `visualReview: pending` fields are preserved in original receipts, while the independent reports above establish actual inspection. This summary does not rewrite automated receipts as human approval.

## Verification boundary

Five final specifications: showcase validation 9/9 checks each, zero composition errors and warnings; atomic delivery successful. Containment measured at 1440×900, 1600×1000, 1920×1080 and 2048×1320; light/dark captures at smallest/largest. Actual image inspection covered those 20 captured images. Do not infer dark-theme middle-size measurements.

Five portable Mermaid blocks rendered successfully using CLI 12.2.0 / Mermaid 12.3.0, with exact source-block and SVG hashes, XML parsing and node/edge parity. Portable SVGs were not separately given the Archify desktop screenshot review; that visual outcome applies to the HTML/inline-SVG captures.

All 20 frozen source/doc/helper hashes remain unchanged, including root README. Only `docs/architecture/` and the coordinator-authorized byte copy `evidence/resume/merged-main-ci.md` were added. No runtime tests were rerun for documentation. No console, USB video, physical projector or new local merged-main-artifact execution was observed. Publication/merge remains with coordinator t9 under the user's authorization.
