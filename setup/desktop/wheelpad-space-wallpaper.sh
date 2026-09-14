#!/usr/bin/env bash
set -euo pipefail
wallpaper_dir="${XDG_DATA_HOME:-$HOME/.local/share}/wheelpad/wallpapers"
mapfile -t frames < <(find "$wallpaper_dir" -maxdepth 1 -type f -name 'space-*.png' | sort)
(( ${#frames[@]} > 0 )) || { echo "No space wallpaper frames in $wallpaper_dir" >&2; exit 1; }
awww-daemon &
daemon_pid=$!
trap 'kill "$daemon_pid" 2>/dev/null || true' EXIT INT TERM
sleep 1
index=0
while true; do
    awww img "${frames[$index]}" --transition-type fade --transition-duration 6
    index=$(( (index + 1) % ${#frames[@]} ))
    sleep 45
done
