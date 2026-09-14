#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")"
if (( EUID == 0 )); then
    echo 'Run without sudo; only package installation uses sudo.' >&2
    exit 1
fi
if [[ "${1:-}" != "--configure-only" ]]; then
sudo pacman -S --needed waybar fuzzel foot fish neovim thunar mako hyprpaper \
    hyprlock hypridle awww pipewire pipewire-pulse pipewire-alsa wireplumber \
    brightnessctl playerctl wl-clipboard noto-fonts-cjk ttf-dejavu gvfs tumbler
fi
./apply-desktop.sh
fish --no-execute "${XDG_CONFIG_HOME:-$HOME/.config}/fish/conf.d/desktop.fish"
foot --check-config
systemctl --user daemon-reload
systemctl --user enable --now pipewire.socket pipewire-pulse.socket wireplumber.service
xdg-mime default thunar.desktop inode/directory
xdg-mime default nvim.desktop text/plain
if [[ -n "${WAYLAND_DISPLAY:-}" ]]; then
    # tmux can retain an old environment; resolve the live instance first.
    [[ -n "${HYPRLAND_INSTANCE_SIGNATURE:-}" ]] || { echo 'HYPRLAND_INSTANCE_SIGNATURE is required in a Hyprland session.' >&2; exit 1; }
    hyprctl reload
    "$HOME/.local/bin/wheelpad-desktop-start"
    hyprctl configerrors
fi
systemctl --user --no-pager status pipewire.socket pipewire-pulse.socket wireplumber.service
