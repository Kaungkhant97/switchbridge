# Source evidence matrix

Pinned source: `e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab`. PR2 merged at 2026-11-12T07:49:03Z, with parents `460e8e8f6122bd748fd6d6bba1e7073e163dec27` and reviewed head `98afc45178d7dfefaf4196e296766828e27e5b18`. Merge is an observed GitHub state, not inferred from CI. The merge tree equals reviewed head; runtime source/tests/helpers/workflow remain unchanged from `ce8aeac`.

[Source-baseline receipt](review/source-baseline.json) verifies all 20 frozen source/doc/helper hashes against the pinned tree, including unchanged root README. Neither frozen verification metadata nor earlier plan/research bytes are rewritten. Plan-review and observed-merge gates were both satisfied before final authoring. The merge observer exited 0 on the observed merge; no continuing merge watcher is required for this source snapshot.

## Nodes and relationships

Stable identifiers match JSON and Mermaid. Every arrow in the five views appears below; sources include the endpoint responsibilities. Citations refer to exact pinned bytes, with symbols/descriptions helping when the code later moves.

| View | Node / relationship IDs | Evidence | Meaning |
| --- | --- | --- | --- |
| c4-context | `operator`, `switchbridge`, `operator-use` | [src/main.rs:10](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/src/main.rs#L10) | CLI commands and usage |
| c4-context | `macos`, `inventory-dependency` | [src/usb.rs:48](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/src/usb.rs#L48) | Inventory collection |
| c4-container | `cli`, `cli-files`, `files` | [src/main.rs:19](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/src/main.rs#L19) | Dispatch and file I/O |
| c4-container | `os`, `cli-os` | [src/usb.rs:48](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/src/usb.rs#L48) | OS command dependencies |
| dataflow-prerecorded-loopback | `clip`, `sender`, `read-clip` | [src/transport.rs:24](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/src/transport.rs#L24) | Annex B input and chunking |
| dataflow-prerecorded-loopback | `receiver`, `tcp-chunks` | [src/main.rs:30](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/src/main.rs#L30) | Socket invocations |
| dataflow-prerecorded-loopback | `write-output`, `output` | [src/main.rs:45](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/src/main.rs#L45) | Create-new, receive and cleanup |
| dataflow-prerecorded-loopback | `read-playback`, `player` | [docs/macos-playback.md:84](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/docs/macos-playback.md#L84) | Manual post-completion playback |
| dataflow-usb-evidence | `os-tools`, `collect-inventory`, `snapshot` | [src/usb.rs:48](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/src/usb.rs#L48) | OS collection |
| dataflow-usb-evidence | `save-snapshot`, `snapshots` | [src/usb.rs:251](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/src/usb.rs#L251) | Redaction and exclusive write |
| dataflow-usb-evidence | `load-inventories`, `diff` | [src/usb.rs:188](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/src/usb.rs#L188) | Two-input validation/redaction |
| dataflow-usb-evidence | `emit-diff`, `stdout` | [src/usb.rs:22](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/src/usb.rs#L22) | stdout command result |
| workflow-ci-artifacts | `source`, `source-checks` | [.github/workflows/macos.yml:19](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/.github/workflows/macos.yml#L19) | Source SHA selection |
| workflow-ci-artifacts | `checks`, `checks-package` | [.github/workflows/macos.yml:23](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/.github/workflows/macos.yml#L23) | Parallel checks then package |
| workflow-ci-artifacts | `package`, `package-smoke` | [.github/workflows/macos.yml:46](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/.github/workflows/macos.yml#L46) | Universal upload before smoke |
| workflow-ci-artifacts | `smoke`, `smoke-select` | [.github/workflows/macos.yml:84](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/.github/workflows/macos.yml#L84) | Same-run download and native smoke |
| workflow-ci-artifacts | `select` | [docs/macos-playback.md:8](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/docs/macos-playback.md#L8) | Manual entire-run selection |

## Scenarios, controls and failure evidence

| Scenario / claim | Exact source | Named software evidence / limit |
| --- | --- | --- |
| Numeric loopback/nonzero port | [src/transport.rs:9](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/src/transport.rs#L9) | `only_numeric_loopback_endpoints_are_allowed` in tests/transport.rs |
| SWB1 header and payload bounds | [src/framing.rs:1](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/src/framing.rs#L1) | tests/framing.rs: `writes_independently_specified_wire_bytes`, `every_truncated_prefix_is_rejected`, `malformed_magic_version_kind_and_flags_are_rejected`, `invalid_payload_sizes_are_rejected_before_reading_payload` |
| Chunking / Annex B / sequence | [src/transport.rs:24](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/src/transport.rs#L24) | tests/transport.rs: `transport_round_trip_preserves_annex_b_across_chunk_boundaries`, `sender_rejects_empty_short_or_non_annex_b_input`, `receiver_rejects_missing_or_out_of_order_packets` |
| One connection, exclusive output, handled cleanup | [src/main.rs:45](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/src/main.rs#L45) | tests/network.rs: `cli_tcp_round_trip_preserves_h264_and_existing_output_is_protected`, `malformed_tcp_stream_fails_and_removes_partial_output`, `original_protocol_error_survives_failed_partial_output_cleanup` |
| 10s connected I/O, accept unbounded | [src/main.rs:9](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/src/main.rs#L9) | connect_timeout/set_write_timeout/set_read_timeout; accept has no timeout |
| No ACK/end marker/digest; boundary EOF | [src/framing.rs:16](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/src/framing.rs#L16) | read_packet clean EOF returns None; receive exits at EOF. Whole trailing packet omission is an explicitly documented limit, not a passing guarantee test. |
| Supported private-value handling | [src/usb.rs:65](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/src/usb.rs#L65) | module tests cover known labels, continuations, embedded/short/colon-bearing values and cross-input redaction; tests/usb_cli.rs validates CLI diff paths |
| Diff input and snapshot output limits | [src/usb.rs:240](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/src/usb.rs#L240) | `rejects_empty_invalid_utf8_and_oversized_inventory_inputs`; `writes_redacted_inventory_and_refuses_to_replace_existing_evidence` |
| Universal build and checksums | [scripts/package-macos.sh:16](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/scripts/package-macos.sh#L16) | Both targets, lipo and archive/internal hashes; checksums do not authenticate a signer |
| Native packaged transport smoke | [scripts/smoke-packaged-macos.py:13](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/scripts/smoke-packaged-macos.py#L13) | Executed by both hosted smoke jobs; not a hardware test |
| Hardware/privacy limitations | [docs/usb-baseline.md:1](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/docs/usb-baseline.md#L1) | Three controlled human sessions pending; unknown private data needs manual review |
| Playback / projector acceptance | [docs/macos-playback.md:57](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/docs/macos-playback.md#L57) | Completed prerecorded file and human observed mirrored projector are separate |
| Console video unverified | [docs/research/stock-switch2-usb-feasibility.md:1](https://github.com/Kaungkhant97/switchbridge/blob/e1b81f7490ea4f5893ec8fe8b276d76ff92ed2ab/docs/research/stock-switch2-usb-feasibility.md#L1) | Present feasibility gate; not a proof of universal impossibility |

## CI and observation provenance

- [Merged-main CI report](../../evidence/resume/merged-main-ci.md): `43178062662` on exact `e1b81f7`, all five jobs pass. Artifact `13924514656`, expires 2026-11-26T07:50:13Z; ZIP digest is GitHub metadata, with no independent local download/runtime claim for those bytes.
- [Historical downloaded-byte QA](../../evidence/resume/hosted-ci.md): `43141901934` on `ce8aeac`; artifact `13909373980` was separately downloaded/hashed and exercised with 60 synthetic frames. That proof remains attached to those older bytes.
- Intermediate `43164099151` on `9113cd1` and reviewed-head `43170855840` on `98afc45` are separate successful runs; their artifacts are not substituted for the historically verified object.
- `verification-metadata.json` pending-hosted-CI values are a frozen historical snapshot, explicitly superseded by later hosted evidence. Preserve its bytes.

No new runtime tests were run for this documentation change. Named tests describe existing coverage; historical counts/coverage belong to their recorded source. No console reconnect, live USB video or physical projector test was performed during documentation authoring.

## Primary diagram practices

The [reviewed research note](../research/2026-11-12-c4-dataflow-practices.md) cites official C4 abstractions/notation/checklist, Microsoft DFD process/store/flow/trust-boundary guidance and Mermaid syntax/accessibility, accessed 2026-11-12. C4 and DFD mappings here are scoped modelling choices. Processes can originate flows, including process-to-store/process-to-process; stores never communicate directly. Render validation is separate from semantic review and actual visual inspection.
