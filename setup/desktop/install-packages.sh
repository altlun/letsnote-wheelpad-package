#!/usr/bin/env bash
set -euo pipefail
sudo pacman -S --needed fcitx5 fcitx5-mozc fcitx5-configtool fcitx5-gtk fcitx5-qt noto-fonts-cjk tmux wl-clipboard
# Check config in an isolated server, leaving existing tmux sessions untouched.
tmux_check_socket=$(mktemp -u /tmp/wheelpad-tmux-check.XXXXXXXX)
trap 'tmux -S "$tmux_check_socket" kill-server 2>/dev/null || true' EXIT
tmux -S "$tmux_check_socket" -f /dev/null new-session -d -s config-check
tmux -S "$tmux_check_socket" source-file "$HOME/.tmux.conf"
tmux -S "$tmux_check_socket" show-options -g mouse
printf '%s\n' 'Packages installed and tmux configuration checked.' 'Japanese input still needs your Wayland desktop integration.'
