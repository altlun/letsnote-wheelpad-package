#!/usr/bin/env bash
set -euo pipefail
choice=$(printf '%s\n' 'Lock' 'Log out' 'Restart' 'Shut down' | fuzzel --dmenu --prompt='Power: ')
case "$choice" in
    Lock)    hyprlock ;;
    'Log out') hyprctl dispatch exit ;;
    Restart) systemctl reboot ;;
    'Shut down') systemctl poweroff ;;
esac
