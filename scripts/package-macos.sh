#!/usr/bin/env bash
set -euo pipefail

if [[ "$(uname -s)" != Darwin ]]; then
  echo "Universal packaging requires macOS and the Xcode command line tools." >&2
  exit 1
fi

cd "$(dirname "$0")/.."
export MACOSX_DEPLOYMENT_TARGET=11.0
output_dir="$PWD/dist"
staging_dir="$(mktemp -d "${TMPDIR:-/tmp}/switchbridge-package.XXXXXX")"
trap 'rm -rf "$staging_dir"' EXIT
mkdir -p "$output_dir"

for target in aarch64-apple-darwin x86_64-apple-darwin; do
  cargo build --locked --release --bin switchbridge --target "$target" --target-dir target
done

lipo -create target/aarch64-apple-darwin/release/switchbridge \
  target/x86_64-apple-darwin/release/switchbridge -output "$staging_dir/switchbridge"
lipo "$staging_dir/switchbridge" -verify_arch arm64 x86_64
chmod 755 "$staging_dir/switchbridge"
(
  cd "$staging_dir"
  shasum -a 256 switchbridge > SHA256SUMS
)
COPYFILE_DISABLE=1 tar --no-xattrs --uid 0 --gid 0 \
  -czf "$output_dir/switchbridge-macos-universal.tar.gz" \
  -C "$staging_dir" switchbridge SHA256SUMS
(
  cd "$output_dir"
  shasum -a 256 switchbridge-macos-universal.tar.gz > switchbridge-macos-universal.tar.gz.sha256
)
echo "Package and checksum written to $output_dir"
