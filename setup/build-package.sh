#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")/.."
test -f Cargo.lock || cargo generate-lockfile
cargo fmt --check
cargo test --locked
mkdir -p dist
# Explicit source list keeps recordings, virtualenvs and build outputs out.
tar -czf packaging/arch/wheelpad-0.2.0.tar.gz \
    --transform='s,^,wheelpad-0.2.0/,' \
    Cargo.toml Cargo.lock src/main.rs src/gesture.rs src/pointer.rs src/setup.rs setup/wheelpad.service \
    setup/70-wheelpad.rules setup/wheelpad.conf setup/desktop/wheelpad.lua README.md LICENSE
cd packaging/arch
checksum=$(sha256sum wheelpad-0.2.0.tar.gz)
checksum=${checksum%% *}
sed -i "s/^sha256sums=.*/sha256sums=('$checksum')/" PKGBUILD
makepkg --force
cp -- wheelpad-0.2.0-3-*.pkg.tar.zst ../../dist/
