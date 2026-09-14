# Japanese input and tmux

## Study applications

Run `bash setup/desktop/install-study-apps.sh` from the repository root.
Add `--github-login` to open GitHub in Firefox and authorize GitHub CLI.
Enter the sudo password in the terminal and GitHub credentials in the browser.

Applications are available from Super+R:

- Firefox: web lessons, documentation, and GitHub.
- Xournal++ (`xournalpp`): handwritten notes and PDF annotations.
- Qalculate! (`qalculate-gtk`): formulas and unit calculations.
- Geany (`geany`): a lightweight editor for programming exercises.

These packages come from Arch's official repositories. On stale mirrors or
databases, perform the normal full `sudo pacman -Syu` upgrade before retrying.

Fish does not restore a previous directory in the supplied configuration.
The tmux bindings for new windows and splits explicitly use
`-c '#{pane_current_path}'`, inheriting the active pane's directory.
Reattaching a running tmux session also preserves its shells and directories.

Prepared for a Japanese (jp106) keyboard, retaining the English desktop locale.
Fcitx5 + Mozc requires a graphical session; it does not supply Japanese input
to a Linux virtual console. Japanese input inside tmux is handled by the
terminal emulator's input method.

Required packages:

```sh
sudo pacman -S --needed fcitx5 fcitx5-mozc fcitx5-configtool fcitx5-gtk fcitx5-qt noto-fonts-cjk tmux wl-clipboard
```

If package downloads fail because the system is outdated, perform the normal
Arch full upgrade (`sudo pacman -Syu`) before retrying; do not use `pacman -Sy`
alone.

Japanese input: Ctrl+Space or 半角/全角 toggles Mozc. Type romaji, press Space
to convert, and Enter to commit. 変換 activates input; 無変換 deactivates it.

For KDE Plasma Wayland, select **Fcitx 5** under **System Settings → Keyboard →
Virtual Keyboard**. Use native Wayland input integration; do not globally set
GTK_IM_MODULE or QT_IM_MODULE. Set XMODIFIERS=@im=fcitx for XWayland clients.
Other compositors may need different startup/input-module configuration.

Tmux: start `tmux new -s work`. Prefix is Ctrl+b, then:

- `|` splits side by side; `-` splits top/bottom.
- `c` creates a window; `n` / `p` switches windows.
- `h` / `j` / `k` / `l` selects a pane.
- `[` enters copy mode; `v` selects; `y` copies.
- `d` detaches; `tmux attach -t work` reconnects.
- `r` reloads the configuration.

Mouse scrolling and pane selection are enabled. Hold Shift for the terminal's
own mouse selection. Wayland clipboard integration is enabled when a new tmux
server starts inside a Wayland session; console copy uses tmux's own buffer.

References:
- https://fcitx-im.org/wiki/Using_Fcitx_5_on_Wayland/en
- https://github.com/tmux/tmux/wiki/Getting-Started

## Desktop suite (Arch / Hyprland Lua)

Run from the checkout as your normal user in the Hyprland session:

```sh
./setup/desktop/setup-desktop.sh
```

This installs Waybar, Fuzzel, Foot, Fish, Neovim, Thunar, Mako, Hyprpaper,
Hyprlock, Hypridle, PipeWire and WirePlumber, plus clipboard, Japanese fonts,
volume/media/backlight and file-manager helpers. `hyprlock` / `hypridle` are
used for the requested lock and idle tools.

The installer backs up replaced files under
`~/.local/state/wheelpad-desktop/backups/<timestamp>/` before installing the
tracked configs. It changes the three application variables in the existing
Hyprland Lua config and adds a separate desktop module, preserving the other
Hyprland settings, Japanese input and wheelpad integration.

- Super+Q: Foot, starting Fish (the login shell is unchanged).
- Super+R: Fuzzel application launcher.
- Waybar の `Apps`: Fuzzel のアプリ一覧。`Super+R` でも開けます。
- Waybar の `⏻`: ロック、ログアウト、再起動、シャットダウン。
- Super+E: Thunar; it is also the default directory handler.
- Super+L: Hyprlock, using the login password.
- 10 minutes idle: lock; 11 minutes: screen off. No automatic suspend timer.
- Neovim: `nvim`, system clipboard, line numbers, Space then W to save.
- Waybar: workspaces 1〜5, clock, volume, network, battery and tray. Workspace
  buttons are persistent, so an empty workspace can be selected directly.
- Hyprland はデスクトップアイコンを表示しない構成です。アプリは `Apps` または
  `Super+R` から起動します。
- PipeWire/PulseAudio compatibility and WirePlumber use their packaged defaults.
- Hyprpaper uses the installed `/usr/share/hypr/wall0.png` wallpaper.

Desktop processes run as user services grouped in `wheelpad-desktop.target`.
The Hyprland start hook imports the session environment before starting them;
the exit hook stops them. Inspect with:

```sh
systemctl --user status wheelpad-desktop.target
journalctl --user -u wheelpad-waybar -u wheelpad-mako -u wheelpad-hyprpaper -u wheelpad-hypridle -n 50
wpctl status
hyprctl configerrors
```

Package download failures on an outdated Arch install require a full
`sudo pacman -Syu` before retrying, not a standalone `pacman -Sy`.

Configuration references:
- https://wiki.hypr.land/Configuring/Basics/Autostart/
- https://wiki.hypr.land/Hypr-Ecosystem/hyprpaper/
- https://wiki.hypr.land/Hypr-Ecosystem/hypridle/
- https://wiki.hypr.land/Hypr-Ecosystem/hyprlock/
