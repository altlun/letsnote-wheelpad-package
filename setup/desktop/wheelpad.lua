-- Load from hyprland.lua with dofile("/usr/share/wheelpad/wheelpad.lua").
hl.on("hyprland.start", function ()
    hl.exec_cmd("systemctl --user start wheelpad.service")
end)
hl.on("hyprland.shutdown", function ()
    hl.exec_cmd("systemctl --user stop wheelpad.service")
end)
