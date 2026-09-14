#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")"
if (( EUID == 0 )); then
    echo 'Run without sudo; only package installation uses sudo.' >&2
    exit 1
fi
if [[ "${1:-}" != "--configure-only" ]]; then
sudo pacman -S --needed waybar fuzzel foot fish neovim thunar mako hyprpaper \
    hyprlock hypridle pipewire pipewire-pulse pipewire-alsa wireplumber \
    brightnessctl playerctl wl-clipboard noto-fonts-cjk ttf-dejavu gvfs tumbler
fi
python3 apply-desktop.py
fish --no-execute "${XDG_CONFIG_HOME:-$HOME/.config}/fish/conf.d/desktop.fish"
foot --check-config
systemctl --user daemon-reload
systemctl --user enable --now pipewire.socket pipewire-pulse.socket wireplumber.service
xdg-mime default thunar.desktop inode/directory
xdg-mime default nvim.desktop text/plain
if [[ -n "${WAYLAND_DISPLAY:-}" ]]; then
    # tmux can retain an old environment; resolve the live instance first.
    if [[ -z "${HYPRLAND_INSTANCE_SIGNATURE:-}" ]]; then
        HYPRLAND_INSTANCE_SIGNATURE=$(hyprctl instances -j | python3 -c 'import json,sys; a=json.load(sys.stdin); print(a[0]["instance"]) if len(a)==1 else sys.exit("Select the Hyprland session first")')
        export HYPRLAND_INSTANCE_SIGNATURE
    fi
    hyprctl reload
    "$HOME/.local/bin/wheelpad-desktop-start"
    hyprctl configerrors
fi
systemctl --user --no-pager status pipewire.socket pipewire-pulse.socket wireplumber.service
