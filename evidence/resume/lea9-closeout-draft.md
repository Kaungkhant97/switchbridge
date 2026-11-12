# LEA-9 ticket update draft — review before posting

Proposed update for [LEA-9](https://linear.app/trip123/issue/LEA-9/m2-investigate-stock-switch-2-live-stream-feasibility-stopgo-decision):

The documentary feasibility deliverable is complete in `docs/research/stock-switch2-usb-feasibility.md`, landed through [merged PR1](https://github.com/Kaungkhant97/switchbridge/pull/1) (merge `460e8e8`, report commit `efa25fb`). Recommend accepting this documented gate under the ticket's explicit fallback:

> Otherwise write a documented **NO-GO / BLOCKED** decision and narrow feasible alternatives.

Decision: **BLOCKED — direct USB live-gameplay feasibility UNVERIFIED; NO-GO for committing to that product path on current evidence.** This supports documentary ticket closeout independently of future hardware research. It does not establish capture success or impossibility.

The report's inspected Nintendo primary sources distinguish:

- [Saved-media USB transfer](https://www.nintendo.com/en-gb/Support/Troubleshooting/How-to-Transfer-Screenshots-and-Video-Captures-from-Nintendo-Switch-2-to-a-Computer-via-a-USB-Cable-2914249.html): an MTP-capable computer and Copy to PC transfer saved screenshots/videos; this is not evidence of live framebuffer access. Do not relabel an observed protocol PTP without evidence.
- [GameChat](https://play.nintendo.com/news-tips/tips-tricks/gamechat-holiday-tips-and-tricks/): Internet-based screen sharing is a distinct network feature, not evidence of a generic USB video source.
- [USB camera](https://www.nintendo.com/sg/hardware/switch2/usb-camera/index.html): documented camera input to the console does not establish gameplay output to a Mac.
- [Dock/HDMI display](https://en-americas-support.nintendo.com/app/answers/detail/a_id/68459/p/1095/c/286): a documented display alternative outside the direct USB/no-dock constraint.

No physical console experiment has been observed for this investigation: no three controlled reconnect pairs, sufficient console descriptors, endpoint traffic, or attributable decoded live gameplay frames. Interface classification remains blocked on actual evidence. Host transport tests and passing ARM/Intel CI prove software behavior only; PR2 remains open and is separate from merged PR1.

Future research handoff from LEA-8 must contain only explicitly privacy-reviewed files in `evidence/lea-8/<session>/`: `session.md`, `commands.log`, `trial-{1,2,3}-{before,after,diff}.txt`, `summary.md`, sanitized `transformations.md`, and verified `SHA256SUMS`. Include actual Mac/macOS, console firmware/state, cable/ports/topology, source/executable provenance, timestamps/exits, controlled attribution, privacy sign-off and descriptor limitations. Copy to PC evidence, if collected and reviewed, belongs in separately labelled `copy-to-pc/` records. Private originals, unreviewed collector output, failed attempts and full transformation/source-hash mappings remain in ignored `.evidence/lea-8/<session>/`; they are not the default LEA-9 input. Additional sanitization requires regenerated, reviewed share diffs. Enumeration alone cannot supply missing interface/endpoint descriptors or prove video.

A future PASS still requires the contract's repeatable real live frames attributable to console USB traffic: sufficient actual descriptors, an authorized live-output candidate/read method, and three independent reconnect sessions retaining synchronized USB traffic → received payload → decoder → contemporaneous scripted gameplay evidence and hashes. Disconnected controls must yield no matching frames; idle controls must not reproduce the scripted changes. No synthetic input, saved media or GameChat substitution. Hardware availability, scoped human permission and suitable read-only tooling remain gates; no console modification or writes are proposed.

Narrow alternatives remain offline MTP media with separate Mac validation; dock/HDMI display; a separately scoped, untested HDMI capture-to-Mac proposal if constraints are relaxed; and continued clearly labelled synthetic/prerecorded host development. Future hardware research can be tracked separately without withholding acceptance of this documented BLOCKED/NO-GO deliverable.

Draft only: no ticket state change or external posting performed. Execution detail: `docs/superpowers/plans/2026-11-12-lea-9-feasibility-gate.md`.
