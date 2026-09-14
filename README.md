# wheelpad — Let's Note / Hyprland 向け円形スクロール

CF-SZ6 の `SynPS/2 Synaptics TouchPad` の外周をなぞる動きを、
evdev/uinput 経由でマウスホイールへ変換する Rust 製プログラムです。
時計回りで下、反時計回りで上へスクロールします。
Hyprland のプラグインではなく仮想マウスとして動くため、X11 は不要です。

## Arch Linux でパッケージを作る

```sh
sudo pacman -S --needed rust base-devel
./setup/build-package.sh
sudo pacman -U dist/wheelpad-0.2.0-3-x86_64.pkg.tar.zst
wheelpad setup
wheelpad --check
```

udev ルールはアクティブなローカルセッションにタッチパッドと uinput への
アクセスを付与します。uinput のアクセス権は仮想入力デバイスの作成を許可します。
権限エラーが残る場合はグラフィカルセッションへログインし直してください。

## 動作確認

古い wheelpad を Ctrl+C で停止してから実行します。

```sh
wheelpad --dry-run
# 外周をなぞり wheel=+1 / wheel=-1 を確認。Ctrl+C で終了。
wheelpad
```

通常起動では外周から触れた操作をスクロール専用にし、ポインターを固定します。
中央から触れた操作は通常のポインター移動です。切り替えには指を離してください。
外周から始めたタップや複数指操作は指を離すまで抑制されます。
中央から始めた操作は外周へ移動してもスクロールへ切り替わりません。
ログを出さず、カーソルのあるウィンドウへスクロールを送ります。
長いページなどで試してください。停止は Ctrl+C です。
`--reverse` で方向反転、`--degrees-per-step 10` で高速化（既定値15）、
`--device /dev/input/event11` で入力デバイスを明示できます。
通常は名前で自動検出するので再起動時の番号変更に対応します。

## Hyprland で自動起動

パッケージには `/usr/lib/systemd/user/wheelpad.service` と Lua 設定を含みます。
現在の `~/.config/hypr/hyprland.lua` の末尾に次を追加します。

```lua
dofile("/usr/share/wheelpad/wheelpad.lua")
```

次回 Hyprland 起動時に開始し、終了時に停止します。
今のセッションでは以下で開始できます。

```sh
systemctl --user daemon-reload
systemctl --user start wheelpad.service
journalctl --user -u wheelpad.service -n 30 --no-pager
```

古い `~/.config/systemd/user/wheelpad.service` があるとパッケージ版より優先されるため、
古い ExecStart が残っていないか `systemctl --user cat wheelpad.service` で確認してください。
旧設定は退避してから daemon-reload してください。
旧 hyprland.conf を使う環境では `exec-once = systemctl --user start wheelpad.service` を使用します。

停止: `systemctl --user stop wheelpad.service`。
自動起動の解除: 追加した `dofile` 行を削除します。
アンインストール: 停止・設定行の削除後に `sudo pacman -R wheelpad`。

セットアップは Rust CLI からも実行できます。`wheelpad setup` は uinput/udev を
再読み込みし、Hyprland の Lua 設定と user systemd サービスを設定します。
設定だけ行う場合は `wheelpad setup --no-start` を使用します。

## 開発・検証

```sh
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked
./target/release/wheelpad --help
```

中心 (3618.68, 2941.66)、半径 1734.19 は同梱の `circle.csv` から得た CF-SZ6 用の値です。
通常起動では元のタッチパッドを排他的に取得し、仮想タッチパッドを経由して通常操作を転送します。
外周の接触情報だけを抑制し、物理ボタンは転送します。終了すると排他取得を解除します。
入力イベント欠落時は終了し、サービスによる再起動で状態を再構築します。
起動時は指を離してください。`--dry-run` は排他取得しないためポインターは動きます。
デバイス名ごとのHyprland設定がある場合、仮想タッチパッド名にも適用が必要です。

配布パッケージは Rust バイナリと実行設定だけを含み、MIT ライセンスで公開しています。

参考: [evdev](https://docs.rs/evdev/0.13.2/evdev/)、
[Hyprland 自動起動](https://wiki.hypr.land/Configuring/Basics/Autostart/)。
