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
sudo modprobe uinput
sudo udevadm control --reload-rules
sudo udevadm trigger --subsystem-match=input
sudo udevadm trigger --subsystem-match=misc --sysname-match=uinput
sudo udevadm settle
wheelpad --check
# Preserve a previous checkout-specific user service if present.
user_unit="${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user/wheelpad.service"
if [[ -f "$user_unit" ]]; then
    mv -- "$user_unit" "$user_unit.backup-$(date +%s)"
fi
config="${XDG_CONFIG_HOME:-$HOME/.config}/hypr/hyprland.lua"
if [[ ! -f "$config" ]]; then
    echo 'hyprland.lua was not found; see README.md for manual autostart setup.' >&2
    exit 1
fi
line='dofile("/usr/share/wheelpad/wheelpad.lua")'
if ! grep -Fxq "$line" "$config"; then
    cp -p -- "$config" "$config.backup-$(date +%s)"
    printf '\n%s\n' "$line" >> "$config"
fi
systemctl --user daemon-reload
systemctl --user restart wheelpad.service
systemctl --user --no-pager status wheelpad.service
