-- Desktop services are started together with the Wayland session environment.
hl.on("hyprland.start", function ()
    hl.exec_cmd("~/.local/bin/wheelpad-desktop-start")
end)
hl.on("hyprland.shutdown", function ()
    hl.exec_cmd("systemctl --user stop wheelpad-desktop.target")
end)
hl.bind("SUPER + L", hl.dsp.exec_cmd("pidof hyprlock || hyprlock"))
