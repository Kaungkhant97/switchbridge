# SwitchBridge

Rust experiments for transporting prerecorded H.264 on a Mac and investigating
stock Nintendo Switch 2 USB enumeration. Live Switch 2 video capture is
**UNVERIFIED**. Local synthetic transport does not demonstrate console capture.

## Development

Use Rust 1.96.0 (the verified development toolchain), then run:

```sh
cargo test --locked --all-targets
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked
```

There are no third-party runtime dependencies. Tests cover wire fixtures,
malformed/truncated input, size boundaries, transport ordering, endpoint
validation, actual CLI TCP transfer, and failed-receive cleanup.

## Prerecorded H.264 transport

Generate a synthetic H.264 Annex B file with an installed `ffmpeg`:

```sh
ffmpeg -f lavfi -i testsrc2=size=160x90:rate=30 -t 2 -an \
  -c:v libx264 -preset ultrafast -f h264 synthetic.h264
cargo run -- receive 127.0.0.1:9000 received.h264
# In another terminal after the receiver prints "Listening on":
cargo run -- send 127.0.0.1:9000 synthetic.h264
shasum -a 256 synthetic.h264 received.h264
```

The receiver accepts one connection and refuses an existing output path. Sender
and receiver accept only numeric loopback addresses and use ten-second socket
I/O timeouts. The receiver waits for an incoming connection until interrupted.
Handled receive errors remove the incomplete output. Process termination (for
example, Ctrl-C) can leave a partial file; remove it before retrying.
Input must start with a three- or four-byte Annex B start code; this is prefix
validation, not full H.264 decoding or proof of a playable stream.

## SWB1 v1 wire format

Each packet has a 24-byte big-endian header followed by 1–1,048,576 payload bytes:

| Offset | Bytes | Value |
| --- | --- | --- |
| 0 | 4 | ASCII `SWB1` |
| 4 | 1 | Version `1` |
| 5 | 1 | H.264 chunk kind `1` |
| 6 | 2 | Reserved flags, both zero |
| 8 | 4 | Payload length |
| 12 | 4 | Zero-based sequential chunk number |
| 16 | 8 | Microseconds elapsed in sender transport |

The sender chunks bytes in blocks of up to 64 KiB. Chunk boundaries are not
frames, and chunk timestamps are **not frame timing**. EOF is clean only between
packets. The format has no end marker or whole-stream digest, so it cannot detect
loss of complete trailing packets; compare file hashes for the local experiment.

## Evidence and scope

Implementation is tracked in [LEA-7](https://linear.app/trip123/issue/LEA-7).
[LEA-8](https://linear.app/trip123/issue/LEA-8) covers physical USB enumeration;
[LEA-10](https://linear.app/trip123/issue/LEA-10) covers packaging and playback.
Redacted verification records belong in `evidence/` and ticket comments. Local
raw inventories and generated videos belong in ignored `.evidence/` until
reviewed for serial numbers and other private device identifiers.

## USB evidence tooling

```sh
switchbridge usb-snapshot evidence/trial-1-before.txt
switchbridge usb-snapshot evidence/trial-1-after.txt
switchbridge usb-diff evidence/trial-1-before.txt evidence/trial-1-after.txt
```

Snapshot collection requires macOS, redacts serial/UUID/UDID values, and refuses
existing output files. Diff inputs are also redacted before printing. Follow
[the USB baseline procedure](docs/usb-baseline.md): enumeration acceptance needs
**three observed controlled console reconnects**. No console reconnects were
performed during this integration; enumeration remains pending.

Direct USB live capture is **BLOCKED** until repeatable real frames are
attributed to console USB traffic. See the [LEA-9 feasibility gate](docs/research/stock-switch2-usb-feasibility.md).

## macOS distribution

Run `bash scripts/package-macos.sh` with both Apple Rust targets installed to
create `dist/switchbridge-macos-universal.tar.gz` and its checksum. The archive
contains an executable with ARM64 and x86_64 slices and `SHA256SUMS`. CI is
configured to run tests, fmt, and clippy natively on ARM and Intel, then execute
the extracted package on both architectures. Local ARM results do not establish
a passing remote CI run or native Intel execution.

See [macOS packaging and playback](docs/macos-playback.md) for checksum verification,
synthetic playback, and physical projector acceptance. Projector acceptance
remains pending an observed moving image on the physical projector.

Additional local verification:

```sh
cargo llvm-cov --locked --all-targets --fail-under-lines 80 --summary-only
actionlint
shellcheck scripts/package-macos.sh
```

Integration provenance and fresh verification records are in
[evidence/resume](evidence/resume/README.md). Historical source-worktree reviews
are not approval of the integrated code.
