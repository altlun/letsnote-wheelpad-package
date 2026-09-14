#!/usr/bin/env bash
set -euo pipefail
source_dir=$(cd -- "$(dirname -- "$0")" && pwd)
config_home=${XDG_CONFIG_HOME:-$HOME/.config}
backup="$HOME/.local/state/wheelpad-desktop/backups/$(date +%Y%m%d-%H%M%S)"
hypr="$config_home/hypr/hyprland.lua"
[[ -f "$hypr" ]] || { echo "Hyprland Lua config not found: $hypr" >&2; exit 1; }
mkdir -p "$backup"
cp -a -- "$hypr" "$backup/hyprland.lua"
sed -i -E 's/^(local[[:space:]]+terminal[[:space:]]*=[[:space:]]*)"[^"]*"/\1"foot"/; s/^(local[[:space:]]+fileManager[[:space:]]*=[[:space:]]*)"[^"]*"/\1"thunar"/; s/^(local[[:space:]]+menu[[:space:]]*=[[:space:]]*)"[^"]*"/\1"fuzzel"/' "$hypr"
grep -qF 'dofile(os.getenv("HOME") .. "/.config/hypr/desktop.lua")' "$hypr" || printf '\n-- Desktop tools configured by wheelpad-lab.\ndofile(os.getenv("HOME") .. "/.config/hypr/desktop.lua")\n' >> "$hypr"
while IFS= read -r -d '' src; do
    rel=${src#"$source_dir/config/"}
    dest="$config_home/$rel"
    mkdir -p "$(dirname -- "$dest")"
    if [[ -e "$dest" || -L "$dest" ]]; then
        mkdir -p "$backup/$(dirname -- "$rel")"
        cp -a -- "$dest" "$backup/$rel"
    fi
    install -Dm644 -- "$src" "$dest"
done < <(find "$source_dir/config" -type f -print0 | sort -z)
install -Dm755 "$source_dir/start-desktop.sh" "$HOME/.local/bin/wheelpad-desktop-start"
install -Dm755 "$source_dir/power-menu.sh" "$HOME/.local/bin/wheelpad-power-menu"
install -Dm755 "$source_dir/wheelpad-space-wallpaper.sh" "$HOME/.local/bin/wheelpad-space-wallpaper"
mkdir -p "$HOME/.local/share/wheelpad/wallpapers"
cp -f -- "$source_dir"/../../assets/wallpapers/space-*.png "$HOME/.local/share/wheelpad/wallpapers/"
printf 'Desktop configuration installed. Backups: %s\n' "$backup"
