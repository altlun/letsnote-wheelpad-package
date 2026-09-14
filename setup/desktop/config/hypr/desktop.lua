-- Desktop services are started together with the Wayland session environment.
hl.on("hyprland.start", function ()
    hl.exec_cmd("~/.local/bin/wheelpad-desktop-start")
end)
hl.on("hyprland.shutdown", function ()
    hl.exec_cmd("systemctl --user stop wheelpad-desktop.target")
end)
hl.bind("SUPER + L", hl.dsp.exec_cmd("pidof hyprlock || hyprlock"))
for i = 1, 5 do
    hl.bind("SUPER + " .. i, hl.dsp.focus({ workspace = i }))
    hl.bind("SUPER + SHIFT + " .. i, hl.dsp.window.move({ workspace = i }))
end
hl.bind("SUPER + TAB", hl.dsp.exec_cmd("hyprctl dispatch cyclenext"))
hl.bind("SUPER + CTRL + left", hl.dsp.exec_cmd("hyprctl dispatch resizeactive -40 0"))
hl.bind("SUPER + CTRL + right", hl.dsp.exec_cmd("hyprctl dispatch resizeactive 40 0"))
hl.bind("SUPER + CTRL + up", hl.dsp.exec_cmd("hyprctl dispatch resizeactive 0 -40"))
hl.bind("SUPER + CTRL + down", hl.dsp.exec_cmd("hyprctl dispatch resizeactive 0 40"))
