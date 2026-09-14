#!/usr/bin/env bash
set -euo pipefail
export XDG_CURRENT_DESKTOP="${XDG_CURRENT_DESKTOP:-Hyprland}"
export XDG_SESSION_TYPE="${XDG_SESSION_TYPE:-wayland}"
systemctl --user import-environment WAYLAND_DISPLAY XDG_CURRENT_DESKTOP XDG_SESSION_TYPE
if [[ -n "${HYPRLAND_INSTANCE_SIGNATURE:-}" ]]; then
    systemctl --user import-environment HYPRLAND_INSTANCE_SIGNATURE
fi
systemctl --user start pipewire.socket pipewire-pulse.socket wireplumber.service
systemctl --user start wheelpad-desktop.target
