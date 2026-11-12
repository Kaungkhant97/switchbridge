# Stock Switch 2 USB live-video feasibility — LEA-9

Research date: 2026-11-12. Scope: an unmodified Nintendo Switch 2 sending live gameplay directly over USB to macOS, without a dock, capture card, or console modification.

## Decision

**BLOCKED (feasibility UNVERIFIED):** no console USB descriptors, endpoint traffic, or attributable live gameplay frames have been collected for this investigation. LEA-8 must provide the physical USB baseline before the exposed interfaces can be classified. **NO-GO for committing to a direct USB live-capture product now**, because its required evidence is missing. This is a project gate, not a proof that such capture is impossible.

Continue the independent prerecorded/synthetic host transport in LEA-7. Its byte-identical H.264 round trip tests host transport only; chunk timestamps are not frame timing and cannot establish console capture.

## What the primary sources establish

| Path | Supported fact | Consequence for this investigation |
| --- | --- | --- |
| USB album transfer | Nintendo specifies an MTP-capable computer, a data-capable USB cable, and the console's Copy to PC setting to transfer selected saved screenshots/videos. Dock USB ports do not support that transfer. [S1] | Treat this as file transfer. MTP support does not establish a live framebuffer endpoint. Do not label the observed protocol PTP without descriptor/protocol evidence. |
| GameChat | Nintendo describes online game-screen sharing and requires Internet for GameChat. [S2] | Screen sharing is a separate network feature; it does not demonstrate a generic USB video source or a supported macOS receiver API. |
| USB camera | A compatible camera connects to the console for video chat or camera features in games. [S3] | The documented direction is camera into console. It does not establish gameplay output from console to Mac. |
| TV mode | Nintendo documents console-to-dock-to-TV/monitor output through the dock's HDMI connection. [S4] | A documented display route exists, but it falls outside the direct USB/no-dock constraint. |

None of these sources documents the required direct USB live-capture endpoint. That bounded observation about these sources does not prove that no endpoint exists.

## Physical evidence register

| Required evidence | Current state |
| --- | --- |
| Mac model/macOS; console firmware/state; cable and port | BLOCKED: no console experiment run; metadata unavailable |
| Three redacted disconnected/connected inventory pairs and repeatable device identifiers (LEA-8) | BLOCKED: physical console availability unconfirmed |
| Device/configuration/interface/alternate-setting and endpoint descriptors | UNVERIFIED: no console descriptor artifacts received |
| Transfer protocol classification for each exposed interface | UNVERIFIED; no device, class, endpoint, or protocol values inferred |
| Recorded live frames tied to console USB traffic with repeatable steps | UNVERIFIED; no live-capture experiment run |

No USB identifiers or serial numbers are invented here. An enumeration PASS would establish enumeration alone.

## Falsifiable next experiment and pass gate

1. Complete LEA-8's three disconnected/connected cycles with a data-capable cable. Record OS, Mac model, console firmware/state, cable/port, tool version, exact commands and timestamps. Preserve device identifiers and redact serials before publication. Include HOME and Nintendo's documented Copy to PC mode as separately labelled observations.
2. Obtain descriptor evidence for each enumerating state through available read-only macOS USB tooling. Record how it was obtained and preserve raw redacted output. If tooling does not expose full descriptors, mark that limitation rather than infer missing values; interface classification remains BLOCKED until adequate descriptor evidence is available. Classify each interface only to the level supported by its descriptors and observed authorized behavior. Saved album-object transfer remains file transfer even when its objects are video.
3. If an authorized live-output candidate is observed, document its repeatable activation and read procedure. Record gameplay with deliberate visible changes and a synchronized experiment log; retain endpoint traffic, received bytes, decoder output, and hashes. Repeat in at least three independent reconnect sessions. In each, decoded frames must show scripted visible gameplay changes inside logged time windows. Disconnected controls must yield no matching frames; idle controls must not reproduce the scripted changes (an idle screen may still produce frames). Retain synchronized logs and hashes for each session to make the attribution auditable. Do not substitute synthetic input, saved clips, or GameChat for USB-origin live frames.
4. **PASS** only after repeatable real gameplay frames are recorded and attributed to console USB traffic. Without that evidence, retain BLOCKED/UNVERIFIED, record the tested conditions and failure/limitation, and state the next test that could change the decision. Negative results apply only to the tested states and firmware.

## Narrow alternatives

- **Recorded-media workflow:** transfer saved captures through Nintendo's documented MTP path, then separately validate Mac compatibility and decoding. This offers offline media, not live capture. [S1]
- **Display without Mac ingestion:** use the documented dock/HDMI TV route, with a compatible projector as a separate display compatibility test for LEA-10. [S4]
- **Mac live ingestion with relaxed constraints:** evaluate a dock/HDMI capture device and macOS receiver as a new hardware path. This is an engineering proposal, not a tested or Nintendo-certified capture setup; it requires separate device/mode compatibility, output-protection behavior, latency, and playback evidence.
- **Host-only development:** continue LEA-7 transport and LEA-10 packaging/playback using clearly labelled synthetic or prerecorded input.

## Sources and review boundary

Sources were inspected on 2026-11-12. These are Nintendo's own pages; their supported facts are summarized above. No third-party capture claim is used as proof.

- **S1:** [Nintendo Support — Transfer Switch 2 screenshots and video captures to a computer via USB](https://www.nintendo.com/en-gb/Support/Troubleshooting/How-to-Transfer-Screenshots-and-Video-Captures-from-Nintendo-Switch-2-to-a-Computer-via-a-USB-Cable-2914249.html), “Before you start” and transfer steps.
- **S2:** [Play Nintendo — GameChat 101](https://play.nintendo.com/news-tips/tips-tricks/gamechat-holiday-tips-and-tricks/), online screen-sharing examples and Internet requirement footnote.
- **S3:** [Nintendo — About USB Camera](https://www.nintendo.com/sg/hardware/switch2/usb-camera/index.html), camera connection and video-chat/game uses.
- **S4:** [Nintendo Support — Connect a Switch 2 console to a TV](https://en-americas-support.nintendo.com/app/answers/detail/a_id/68459/p/1095/c/286), required equipment and dock HDMI steps.

This report is a documented blocked feasibility decision. It does not certify successful enumeration, live capture, projector playback, or an unavailable hardware test. Historical reviews are not approval of this integration; independent review of the current files remains required.
