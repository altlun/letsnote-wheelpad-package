#!/usr/bin/env python3
"""Install the reviewed desktop files, preserving replaced files in one backup."""
import datetime
import os
from pathlib import Path
import re
import shutil

source = Path(__file__).resolve().parent
home = Path.home()
config = Path(os.environ.get('XDG_CONFIG_HOME', home / '.config'))
backup = home / '.local/state/wheelpad-desktop/backups' / datetime.datetime.now().strftime('%Y%m%d-%H%M%S-%f')
hypr = config / 'hypr/hyprland.lua'
original = hypr.read_text()
updated = original
for name, command in [('terminal', 'foot'), ('fileManager', 'thunar'), ('menu', 'fuzzel')]:
    updated, count = re.subn(r'(local\s+' + name + r'\s*=\s*)"[^"]*"', lambda m: m[1] + '"' + command + '"', updated)
    if count != 1:
        raise SystemExit(f'Expected one local {name} assignment; no files changed.')
line = 'dofile(os.getenv("HOME") .. "/.config/hypr/desktop.lua")'
# Use the actual XDG config path for non-default locations.
if config != home / '.config':
    import json
    line = 'dofile(' + json.dumps(str(config / 'hypr/desktop.lua')) + ')'
if line not in updated:
    updated += '\n-- Desktop tools configured by wheelpad-lab.\n' + line + '\n'

def install(data, dest, relative, mode=0o644):
    if dest.exists() or dest.is_symlink():
        if dest.read_bytes() == data:
            return
        saved = backup / relative
        saved.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(dest, saved)
    dest.parent.mkdir(parents=True, exist_ok=True)
    # Replace a symlink rather than modifying its external target.
    temp = dest.with_name(dest.name + '.wheelpad-new')
    temp.write_bytes(data)
    temp.chmod(mode)
    temp.replace(dest)

for src in sorted((source / 'config').rglob('*')):
    if src.is_file():
        rel = src.relative_to(source / 'config')
        install(src.read_bytes(), config / rel, Path('config') / rel)
install(updated.encode(), hypr, Path('config/hypr/hyprland.lua'))
install((source / 'start-desktop.sh').read_bytes(), home / '.local/bin/wheelpad-desktop-start', Path('bin/wheelpad-desktop-start'), 0o755)
print(f'Desktop configuration installed. Backups (if needed): {backup}')
