# SwitchBridge architecture

These views document merged main `e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab` (PR2 merged 2026-11-12T07:49:03Z). They describe implemented host software, with manual and unverified work labelled separately. Start with context, then choose the flow relevant to your question.

| View | Reader question | Standalone artifact |
| --- | --- | --- |
| [C4 context](c4-context.md) | What does the system do? | [HTML](diagrams/c4-context.html) |
| [C4 container](c4-container.md) | Which application implements it? | [HTML](diagrams/c4-container.html) |
| [Prerecorded flow](dataflow-prerecorded-loopback.md) | Which bytes move and how can they fail? | [HTML](diagrams/dataflow-prerecorded-loopback.html) |
| [USB evidence flow](dataflow-usb-evidence.md) | What is collected/redacted/compared? | [HTML](diagrams/dataflow-usb-evidence.html) |
| [CI artifact workflow](workflow-ci-artifacts.md) | Where do checked binaries come from? | [HTML](diagrams/workflow-ci-artifacts.html) |

## Reading conventions

C4 views show structure and dependency intent; data-flow views show process instances, file stores and directional payload movement; CI shows build/artifact dependencies. These arrow meanings are explained in each view. CLI subcommands are roles of one application in this chosen SwitchBridge model, not universally constrained by sharing a binary. Rust modules are code organization, without an automatic C4 component claim.

Rectangles labelled files are ordinary caller-selected artifacts outside the executable in the C4 scope and data stores in the DFD. They are not databases. Dashed C4 boxes mark system scope; dashed data-flow columns group stages. Neither automatically means a trust boundary. The adjacent context tables assess actual validation, privacy and external provenance boundaries. Node type, words and arrows carry meaning in both themes; colors are supplemental. SWB1 names the custom packet format; H.264 Annex B is the prerecorded input format; USB is Universal Serial Bus; DFD means data-flow diagram; CI means continuous integration.

**Outside implemented flow:** console USB video, attributable console enumeration trials and physical mirrored projector observation remain UNVERIFIED/pending. Inventory, matching saved media and successful host CI do not establish those claims. There is no new backend, database, release pipeline or live endpoint.

## Evidence and review

[Evidence matrix](evidence.md) anchors every view to immutable source links and named tests. [Source baseline](review/source-baseline.json) records merge parents and all 20 frozen hashes. Root README was left unchanged, so its frozen hash still matches. [Manifest](review/manifest.json) binds specifications, HTML, screenshots and receipts. [Paired review](review/paired-review.md) records independent semantic and actual visual review, separately from automated validation.

Portable Mermaid is included in each Markdown view with accessibility descriptions. Renderer validation uses the actual installed Mermaid version recorded in review tooling evidence; hosting viewers may use another version. Standalone HTML includes inline SVG, theme controls, zoom and export. Read the prose for failure paths and contextual details that would crowd a diagram.

## Maintenance

When CLI, framing/transport, USB redaction, packaging/workflow or hardware evidence changes, update the evidence matrix and affected views from the new exact source SHA. Revalidate Mermaid and Archify, atomically deliver, capture all desktop sizes, and perform semantic plus actual screenshot review on the new hashes. Preserve old evidence attribution; a new CI run's artifact is a different object. Existing usage docs and feasibility report own their detailed instructions/conclusions.

The local package uses installed Archify 2.16.0. Its palette's `frontend` kind denotes the user-facing CLI here; it does not imply a browser UI. File nodes use neutral external-artifact styling. No database node type is used. Review commands and limitations live in the manifest and review record.
