# macOS builds and synthetic playback

SwitchBridge transports prerecorded H.264 Annex B over localhost. It does not
provide a native Switch 2 capture source. Chunk timestamps are not frame timing.

## Build and download

The `macOS` workflow tests and lints natively on Apple Silicon and Intel, using
Rust 1.96.0. After both checks pass, it builds both targets and combines them
with `lipo`. Open a successful run's **Artifacts** section and download the
artifact named `switchbridge-macos-universal-<commit SHA>` within 14 days.
The final smoke jobs download the archive and execute its sender and receiver
natively on both architectures. Use a run whose **entire workflow** passed.

The artifact contains a tar archive and its SHA256 checksum. Extract the GitHub
artifact ZIP, then run these commands inside that directory:

```sh
shasum -a 256 -c switchbridge-macos-universal.tar.gz.sha256
mkdir unpacked
tar -xzf switchbridge-macos-universal.tar.gz -C unpacked
cd unpacked
shasum -a 256 -c SHA256SUMS
lipo switchbridge -verify_arch arm64 x86_64
test -x switchbridge
```

Compare the hashes with the packaging job's run summary. Checksums detect
corruption; they do not provide a Developer ID signature. For a CLI download,
run this in an empty directory, replacing the run ID and commit SHA:

```sh
gh run download RUN_ID -R Kaungkhant97/switchbridge \
  -n switchbridge-macos-universal-COMMIT_SHA
```

The binary is not notarized. If macOS blocks a trusted browser download, follow Apple's
[first-run guidance](https://support.apple.com/en-us/102445) in Privacy & Security.

Build locally on macOS with Xcode command line tools and Rust installed:

```sh
rustup toolchain install 1.96.0 --profile minimal --component clippy,rustfmt \
  --target aarch64-apple-darwin,x86_64-apple-darwin
rustup override set 1.96.0
cargo fmt --all --check
cargo test --all-targets --locked
cargo clippy --all-targets --locked -- -D warnings
bash scripts/package-macos.sh
```

The script targets macOS 11 or later. It produces an unsigned development
binary in `dist/`; signing and notarization are outside this workflow.

## Play synthetic video

Install FFmpeg separately so `ffmpeg` and `ffplay` are on your PATH. Generate a
short test clip in the extracted binary directory:

```sh
ffmpeg -f lavfi -i testsrc2=size=640x360:rate=30 -t 5 \
  -c:v libx264 -pix_fmt yuv420p -f h264 synthetic.h264
```

From the extracted binary directory, start the receiver in one terminal:

```sh
./switchbridge receive 127.0.0.1:7878 received.h264
```

In another terminal, send the clip. Use absolute input/output paths if the
terminals have different working directories:

```sh
./switchbridge send 127.0.0.1:7878 synthetic.h264
```

The receiver exits after one completed stream. Wait for Terminal A's receiver
to exit successfully (`echo $?` prints `0`).
Then compare hashes and play the completed output from its directory:

```sh
shasum -a 256 synthetic.h264 received.h264
ffplay -fs -autoexit -framerate 30 -f h264 received.h264
```

Matching hashes prove local byte transport only. Press `q` to leave playback early.

## Projector acceptance evidence

Connect the projector to the Mac, enable mirroring in System Settings →
Displays, and play the received synthetic clip fullscreen. Record the Mac and
macOS version, projector model, connection/adapter, display configuration,
binary commit and checksum, command exit status, and an observed moving image
on the projector. Record results on LEA-10 with the passing Actions URL and
artifact link. A successful player process alone does not verify projector
mirroring. Keep hardware acceptance open until someone observes this test.
