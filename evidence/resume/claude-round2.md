# Claude review of snapshot 948a5cc

Parent transcription of the structured report received from independent Claude
terminal `t11`. Reviewed commit: `948a5ccff6a08e7f41eacf373ccd0e0d544c3ae1`;
runtime/docs source: `ab2035e965197b2881d5359ab93d1ea6c5b7b08a`.
This is a static read-only review. The reviewer read the source, CLI integration
tests, documentation, workflow, packaging scripts, verification helper, metadata
and QA log excerpts. Its hash/diff shell check was denied and not retried; it
did not execute tests/builds or independently recompute hashes.

Verdict: **clears_bar=true** at the no-critical/high threshold, with one
actionable medium finding. This is not a zero-findings approval. Parent chose
to address the medium and selected low/info issues before final convergence.

| ID | Severity | Finding and evidence | Disposition |
| --- | --- | --- | --- |
| N1 | Medium | Exact private-label allowlist leaves a `Volume UUID: 1234-ABCD` fixture unchanged; other qualified UUID/UDID/serial-number labels have the same path. The assertion that legacy profiler output emits this exact label came from reviewer recollection and was **not observed** in this session. The local fixture behavior is separately testable. | Developer will add failing fixtures and generalize private-field recognition without rejecting ordinary `USB Serial:` headings; no hardware-output claim inferred. |
| N2 | Low | First-colon splitting can leave a known colon-containing serial visible in a device heading such as `Dev 00:11:22:33:44:55:`. Static trace; actual profiler use of this serial format was not observed. | Developer will reproduce with fixtures and preserve full-heading redaction across both diff inputs. |
| N3 | Low | Short-token policy can over-redact unrelated version/location fragments, e.g. serial `1` within `Version: 1.00`. No leak; policy already documented. | Record as evidence-quality limitation; no broad preservation of unrelated fields that would silently weaken privacy. |
| N4 | Low | Failed output cleanup masks the original receive/protocol error. | Developer will add a network regression and retain the original error while reporting cleanup failure. |
| N5 | Info | Review log is intentionally pending; verifier assertions disappear under Python optimization. | Parent will create final review log; developer will use explicit verification exceptions. |

Prior F1, F2, F4, F8 and naming drift were resolved in the reviewed source or
documentation. Prior speculative volume-field concern became N1; its actual
host-output premise remains unverified. Low-priority single-connection/timeout,
sender-acknowledgment, archive reproducibility/signing, PR-head testing and
ephemeral-port race limitations remain as documented. No defects were reported
in framing, transport ordering, endpoint restrictions or diff bounds. The
reviewer confirmed lipo argument order, SHA-pinned Actions, read-only workflow
permissions and claim boundaries by inspection.

Parent QA evidence was explicitly distinguished from independent Claude
execution. No console reconnects, real USB gameplay, native Intel/hosted CI or
projector results were claimed. A new frozen snapshot and two clean reviews
are required after implementation cycle 3; this report is not approval of those
later edits.
