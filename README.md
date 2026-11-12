# SwitchBridge

Rust experiments for transporting prerecorded H.264 on a Mac and investigating
stock Nintendo Switch 2 USB enumeration. Live Switch 2 video capture is
**UNVERIFIED**. Local synthetic transport does not demonstrate console capture.

## Development

Install a Rust toolchain, then run:

```sh
cargo test --all-targets
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

The repository starts with an empty library scaffold. Transport, USB evidence
collection, and macOS packaging are tracked in Linear as LEA-7, LEA-8, and LEA-10.
Hardware claims require observed physical-device evidence.
