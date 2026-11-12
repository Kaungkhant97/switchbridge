# Independent final review: SwitchBridge architecture views at `e1b81f7`

**Reviewer:** t24 (Claude, paired architecture researcher). **Date:** 2026-11-12.

This was a read-only review. I did not edit any reviewed artifact.

## What was reviewed

**Source.** Merged main `e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab`.

- I confirmed the pin myself with read-only `gh` queries:
  - PR2 is `MERGED` at 2026-11-12T07:49:03Z.
  - The merge commit equals remote `main`.
  - The parents are `460e8e8` and `98afc45`.
- `git diff` shows 0 runtime files changed against `ce8aeac` and 0 files changed against `98afc45`.
- The preserved plan (`ab93a8f…`) and research note (`e03e08c…`) are byte-exact at `e1b81f7`.

**Artifact access.** I reviewed approved review copies under my own worktree's ignored `.evidence/architecture-review-e1b81f7/`, as the lead authorised.

- Outside-worktree reads were refused by the harness. I did not bypass that, and no broader read was approved.
- `copy-manifest.json` (sha256 `34ae6614…cb4d60`) lists 73 items. Every copy's hash equals both its recorded source hash and its copy hash.
- Before the copies existed, I had hashed the 40 first-handoff render files directly from the authoring worktree. All 40 matched `render-handoff.json` at that time, so the copies trace back to originals I hashed myself.

**Method.**

- **Semantic review.** I checked the JSON specifications, Markdown, Mermaid fences and evidence matrix against `git show e1b81f7:<path>` bytes, the research note's §5 checklist, and read-only GitHub metadata.
- **Visual review.** I opened and inspected all 20 current PNG captures myself: five views, each in light and dark themes at 1440×900 and 2048×1320.
- I treated the automated validation, delivery and visual-check receipts as separate evidence. They do not substitute for the visual inspection.

## Verdict

**Semantic review: PASS. No material (medium or higher) findings.**

**Actual visual review of the 20 PNGs: PASS with LOW observations.**

**Overall: READY from this reviewer's side once the pending items listed under "Pending, outside this review" are closed.** The pending items do not change content.

## Semantic checks

| Check | Result | Evidence |
| --- | --- | --- |
| One application container; `send`/`receive` only as process instances | PASS | c4-container shows one `switchbridge` container. The dataflow labels sender and receiver as "Rust CLI" process instances. The card and prose say "not separate services". |
| OS tools, ffmpeg/ffplay and CI are external | PASS | `system_profiler`, `sysctl` and `sw_vers` match `src/usb.rs:52-56`. ffplay is an operator-run external process, matching `docs/macos-playback.md:84`. CI nodes are external and the lanes are workflow activities. |
| Console appears only as an UNVERIFIED callout with no flow | PASS | The context card and prose carry it, and no arrow touches it. |
| No store→store flow; processes may originate flows | PASS | All four flows in both dataflows run between a process and a store or external element, or process to process. |
| Validation, privacy and OS boundaries are not presented as trust-zone changes | PASS | Both DFD boundary tables say "not automatically a trust zone". Dashed columns are described as stage grouping only. |
| No automatic `.evidence/` store; snapshot path is caller-chosen; diff goes to stdout | PASS | Matches `src/usb.rs:15-22` and `:251-256` (`create_new`). |
| Timeout scope | PASS | `TIMEOUT` is set at `main.rs:9`. `connect_timeout`/`set_write_timeout` are at `:34-35` and `set_read_timeout` at `:56`. `accept()` at `:49` has no timeout. |
| Transport facts | PASS | `CHUNK_SIZE` is 64 KiB (`transport.rs:7`). `MAX_PAYLOAD` is 1 MiB (`framing.rs:4`). The first payload must be Annex B (receive loop). Sequence order is enforced. Loopback and nonzero port are checked (`transport.rs:13`). Cleanup failure is logged and the original error kept (`main.rs:61-66`). No ACK, digest or end marker exists. |
| USB facts | PASS | SPUSBHost→SPUSB fallback is in `select_usb_type`. Private values are shared across both inputs in `inventory_diff:188-196`. Malformed fields fail at `:92-93`. The 16 MiB stdout check happens after collection (`:38`), and the file read is bounded with `take` (`:240-246`). |
| CI workflow facts | PASS | `SOURCE_SHA` is at line 19. Matrix check runs on `macos-15`/`macos-15-intel` with fmt, tests and clippy. Package does lipo, checksums and upload with `retention-days: 14` before smoke. Smoke downloads the artifact and runs natively. Permissions are `contents: read`. |
| Run identities kept distinct (resolves handoff-review L1/L2) | PASS | Four runs are listed separately. Only `43141901934`@`ce8aeac` claims downloaded-byte QA. `43178062662` is confirmed read-only: `push`, `headSha=e1b81f7`, 5/5 success. Artifact `13924514656` has expiry and digest equal to `evidence/resume/merged-main-ci.md`. The frozen metadata is noted as historical. |
| Evidence-matrix anchors | PASS | I checked every cited line at `e1b81f7`. The final copy fixes `usb.rs:44` (a closing brace) → `:48` `collect_inventory`, `macos.yml:20` (blank) → `:19`, and `macos-playback.md:65` → `:84` (the ffplay command). All 13 named tests exist in the stated files. |
| Mermaid | PASS | All five fences have `accTitle`/`accDescr`. For each view, `mermaid-validation.json` (CLI 12.2.0 / Mermaid 12.3.0) gives exit 0, its `mermaid_block_sha256` equals the current fence, and its `svg_sha256` equals the delivered `*.portable.svg`. |
| Titles, legends, labelled unidirectional relationships | PASS | All five views have a title, an on-canvas legend, and a label on every arrow. Numbered DFD flows are 1–4. |
| Semantic labels retained after composition compaction | PASS | Element labels, sublabels, flow labels, cards and boundary labels match the JSON in every capture. |

## Visual observations (actual screenshot inspection)

Severity: none is material. The parent's notes were observations; these are my own independent ones.

- **V1 LOW: 1440 readability in the two dataflow views.**
  - Node sublabels, flow labels and stage headers render at about 8–9 px.
  - Each node occupies about 65 px of a roughly 480 px dashed stage column, leaving most of the canvas empty.
  - In dark mode the stage headers have low contrast.
  - The text is legible at 2048, and the cards repeat the key facts.
  - The C4 views are comfortably readable at 1440 after compaction.
  - Suggestion: shrink the dataflow viewBox height (560 → about 300) so nodes scale up.
- **V2 LOW: legend spacing.**
  - In the container legend, the gap between the first badge "1" and the next swatch is about 30 px, which is adequate.
  - The context legend is tight: badge "1" sits about 8 px from the "Person / external system" swatch at both sizes and in both themes.
  - No overlap.
- **V3 LOW: `tag` text is not drawn on canvas.**
  - Archify puts `tag` only in the SVG `<title>` hover text. This affects the container commands `send · receive · usb-snapshot · usb-diff`, the CI `fmt · tests · clippy` and `Archive + hashes + upload`, and the USB `BEFORE AFTER` arguments.
  - Static screenshots therefore omit them. The cards, prose and arrows convey the same facts, so the semantics are not lost.
- **V4 LOW: workflow view.**
  - At 1440 the legend text "External system 5" is about 9 px.
  - Icons touch the first letter of "Universal package", "Packaged smoke ×2" and "Select passing run".
  - "Entire run succeeds" sits on the lane divider.
  - The lower canvas and page are largely empty.
  - The legend wording "External system" is coarser than the prose ("workflow activities or manual contributor actions").
- **V5 INFO.**
  - Dataflow legends key only the arrow styles. Element type is carried by colour plus sublabel text ("File store", "Rust CLI", "External"), not by DFD shapes. This is acceptable as a declared modelling choice, which README line 17 states.
  - In the context view, the "Runs CLI experiments" arrowhead lands beside the "SwitchBridge scope" label without overlapping it.
- **USB polish verified.** The shortened `usb-diff` node removes the earlier icon/text crowding seen in the pre-refresh capture.

## Pending, outside this review

1. **`review/manifest.json` has a stale entry (LOW).**
   - Its `file_hashes` entry for `review/render-handoff.json` records `f90176ee…596f1f`, but the current copy is `5b3c531d…c0707`.
   - The other 99 path/hash pairs match.
   - Refresh it in the final manifest; `review_status` is already marked pending.
2. **The README links to `review/paired-review.md`, which is not in the copies yet.**
   - This report is the semantic and visual input for it.
   - Do not record a PASS there that is stronger than this report's.
3. **The five `*.portable.svg` files were verified by receipt and hash only.** I did not visually inspect them; only the PNG captures were inspected.
4. **QA (t20) record and final manifest status.**

## Not verified here

- I did not run any runtime tests. Named tests were verified to exist only.
- The Archify validation, delivery and containment receipts were not re-run.
- Hosted CI logs and artifact bytes for `43178062662` were not downloaded; I checked metadata only.

## Exact hashes reviewed (sha256)

Paths are relative to `docs/architecture/` unless they start with `evidence/` or name `copy-manifest.json`. The 20 PNG hashes are the bytes I visually inspected.

| File | sha256 |
| --- | --- |
| `README.md` | `8c74e521e99816427e9cd74716d75ab7012d4f9959f432982cca6ff77127e30d` |
| `evidence.md` | `9db158be8909704e665dd0ecb7ca99f06f404cbd086cc90fb64fd084efd7604e` |
| `c4-context.md` | `6c1ccb4916e00d79ce69c4fea278d9536fedd98b81d20b46a1666510bf2d5485` |
| `c4-container.md` | `c3fc410191528642eb788c0dc140dbe28e31911d00e486bb3535971c8d813303` |
| `dataflow-prerecorded-loopback.md` | `1713baada38d146ce41c97981513910c6af3723b50ece95f7970465cd3beb902` |
| `dataflow-usb-evidence.md` | `2fe988a3b3fb904848fcc1a5d398644e06e16db32565221a81fe8530cc599d90` |
| `workflow-ci-artifacts.md` | `34d8d92db0dc92f1484035515a23cd08e7f271e96b5e3974c161ade5ad790b91` |
| `diagrams/c4-context.json` | `b46649cdcff1aa7d017360178d37b31c2c5bd59e98f72b937d10685be14ff8c2` |
| `diagrams/c4-container.json` | `5c64cd5e600792b7d2923e9b2f5aa3333ec104ee3285fe7d99dd1bcc89e33296` |
| `diagrams/dataflow-prerecorded-loopback.json` | `8f0984ef793d6107c6741595f3ddd3220aeeb1f2bacf5650e237ce5825442fda` |
| `diagrams/dataflow-usb-evidence.json` | `c4e1ebc26a0f00954bf88c5e9fdb84fe6db1e503e2d987579aada686abd71482` |
| `diagrams/workflow-ci-artifacts.json` | `a5f6c0b69631f9bcc266c02a2e89d388d7b331e9e75a3b851a120a326d278ca7` |
| `diagrams/c4-context.portable.svg` | `f8b06d55c54d93da87d31f9f24e21d879004b0c0cc688012e2d1b2f72a85fee8` |
| `diagrams/c4-container.portable.svg` | `d1212745334e60accae27f7123f53dea74b5d975b27004e6202b7054c8a8ffdf` |
| `diagrams/dataflow-prerecorded-loopback.portable.svg` | `16a50ba95c08f6a780f837f9367b80b7915d00f59d9657d6c12fce571e623dfc` |
| `diagrams/dataflow-usb-evidence.portable.svg` | `21aa1961ad6a7da77eabb06980730a38ba44859b7345a454f4cbe00c9e6755fd` |
| `diagrams/workflow-ci-artifacts.portable.svg` | `9d77b70a0fb1c69a30b56f2b5bb867436171dec82dbc4c147cf14a9e9b063d2e` |
| `diagrams/c4-context.visual-check.1440x900.light.png` | `9d30d9416be2a6a5a1b46cc2c2ce21cd1b4862e51c9498c0b377291c15e66854` |
| `diagrams/c4-context.visual-check.1440x900.dark.png` | `bb45919e7555f37a91399bcb9bf17ace9308aab15694191f0b287571c811152b` |
| `diagrams/c4-context.visual-check.2048x1320.light.png` | `6471de6c80b2931387a7ea4a1aea3411a3c385271f1d66f27711f43b8c5931b9` |
| `diagrams/c4-context.visual-check.2048x1320.dark.png` | `14ebc959b384765f5b8bbb68c34a69240486f970f11fa9dc4974360e3f63f1b1` |
| `diagrams/c4-container.visual-check.1440x900.light.png` | `01f2239c51b56eda6f4d2e4f17b0b805f02f2a7c8b71759f2700017836a7ef08` |
| `diagrams/c4-container.visual-check.1440x900.dark.png` | `bd8f599a634c97eaf97fe80a3c55478e4773017867b82a17ab16ff960af6ef34` |
| `diagrams/c4-container.visual-check.2048x1320.light.png` | `7e42e56b2ce81eb71e94481104348733a43eb5a05e8e6758bf5ed294d2066061` |
| `diagrams/c4-container.visual-check.2048x1320.dark.png` | `81970580b3c0e016a646bebd73a013b3783b37760bc0f4125d514e93a99c57fc` |
| `diagrams/dataflow-prerecorded-loopback.visual-check.1440x900.light.png` | `1800e80077f5ea9312a352fc407ffad1e4ae59034177cf54b23c43abff312f66` |
| `diagrams/dataflow-prerecorded-loopback.visual-check.1440x900.dark.png` | `ce9a1da56f3d2464f5d980389950979dec39c416a06366f7ce993931be80f7ad` |
| `diagrams/dataflow-prerecorded-loopback.visual-check.2048x1320.light.png` | `56f5e0d6ba45a82a20f337dbff5f9c6b5c183e880918ca676fd1fecd6bead4df` |
| `diagrams/dataflow-prerecorded-loopback.visual-check.2048x1320.dark.png` | `52045769a09a8c9a839c67a38d5027df6328bec42a4eab6fd22f44f514feb91a` |
| `diagrams/dataflow-usb-evidence.visual-check.1440x900.light.png` | `1e8d5716c05ac4dbdefd7b60fd7bb6735d6b18d8c61ee4769117a2a13f676530` |
| `diagrams/dataflow-usb-evidence.visual-check.1440x900.dark.png` | `58a6e6b84a3135f7305bb460a6f6b42cb0cb99c04bd1739e08c1671fd76e694b` |
| `diagrams/dataflow-usb-evidence.visual-check.2048x1320.light.png` | `fbc0d9ca2b8bc96bb4952a79c1e4656e47232a36905d9ece7090b12c356e38f6` |
| `diagrams/dataflow-usb-evidence.visual-check.2048x1320.dark.png` | `e4a053d07f1a9a322e51ae1803520b333222f350f95262cddc5af72e5a789ca3` |
| `diagrams/workflow-ci-artifacts.visual-check.1440x900.light.png` | `33e83a51924e11fc00ebd85eea5b7893f1c35273e2422a8cb19850fde6646f82` |
| `diagrams/workflow-ci-artifacts.visual-check.1440x900.dark.png` | `64be578e0821a07f6deb8f4c7a52c1d76fc39f9ba10a8870fa41777a57d24405` |
| `diagrams/workflow-ci-artifacts.visual-check.2048x1320.light.png` | `0234fc96c304035bfcce4643b3f75fda20cd936d10bedb7b3e705e9b764b1ea5` |
| `diagrams/workflow-ci-artifacts.visual-check.2048x1320.dark.png` | `bceffccdadfc7d8a84995f70ed82d255e50a8ce112553f0ce20e7846a53afbc2` |
| `review/manifest.json` | `87c87e3c03356c7d92dc1f251b032806b7ed5b372a26fe2192f0759b3b765492` |
| `review/mermaid-validation.json` | `0e9e469220b1868b91b365497a4344a10edc3ebd8cc6d026ac94d10147737549` |
| `review/render-handoff.json` | `5b3c531d22c040e886f51b87817a33922106c5eaa3fa441e6c2f2bae9a3c0707` |
| `evidence/resume/merged-main-ci.md` | `17ca4af625ebaa7764ca3781f0b8365c39692476b84ccf014440e3e3a400e762` |
| `copy-manifest.json` (review-copy index) | `34ae6614370f2081c224c240e59ed7970b4c022980e9ad352196476987cb4d60` |

Any change to these bytes requires a re-review of the changed files.
