#!/usr/bin/env bash
# Run as your normal desktop user, after building the package.
set -euo pipefail
cd -- "$(dirname -- "$0")/.."
if (( EUID == 0 )); then
    echo 'Run this script as your normal desktop user (without sudo).' >&2
    exit 1
fi
package=dist/wheelpad-0.2.0-2-x86_64.pkg.tar.zst
test -f "$package"
sudo pacman -U --needed "$package"
wheelpad setup
wheelpad --check
