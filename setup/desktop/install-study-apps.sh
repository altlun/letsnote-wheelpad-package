#!/usr/bin/env bash
set -euo pipefail

if (( EUID == 0 )); then
    echo 'Run as your normal desktop user; sudo is used only for packages.' >&2
    exit 1
fi

# Use the existing synchronized databases; never perform a partial upgrade.
sudo pacman -S --needed firefox github-cli xournalpp qalculate-gtk geany
pacman -Q firefox github-cli xournalpp qalculate-gtk geany

if [[ "${1:-}" == "--github-login" ]]; then
    firefox --new-window https://github.com/login >/dev/null 2>&1 &
    echo 'GitHub CLI authorization follows. Complete it in Firefox.'
    GH_BROWSER=firefox gh auth login --hostname github.com --git-protocol https --web
fi
