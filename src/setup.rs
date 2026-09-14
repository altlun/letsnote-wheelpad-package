use std::{env, error::Error, fs, path::Path, process::Command};

const HYPRLAND_HOOK: &str = "dofile(\"/usr/share/wheelpad/wheelpad.lua\")";

fn command(program: &str, args: &[&str]) -> Result<(), Box<dyn Error>> {
    let status = Command::new(program).args(args).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{program} failed with {status}").into())
    }
}

fn configure_hyprland() -> Result<(), Box<dyn Error>> {
    let config_home = env::var_os("XDG_CONFIG_HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            env::var_os("HOME")
                .map(|h| Path::new(&h).join(".config"))
                .unwrap()
        });
    let config = config_home.join("hypr/hyprland.lua");
    if !config.exists() {
        eprintln!(
            "wheelpad: Hyprland Lua config not found at {}; add {HYPRLAND_HOOK} manually",
            config.display()
        );
        return Ok(());
    }
    let contents = fs::read_to_string(&config)?;
    if contents.lines().any(|line| line.trim() == HYPRLAND_HOOK) {
        return Ok(());
    }
    let backup = config.with_extension(format!("lua.backup-{}", std::process::id()));
    fs::copy(&config, backup)?;
    let mut updated = contents;
    if !updated.ends_with('\n') {
        updated.push('\n');
    }
    updated.push_str(HYPRLAND_HOOK);
    updated.push('\n');
    fs::write(config, updated)?;
    Ok(())
}

pub fn run(no_start: bool) -> Result<(), Box<dyn Error>> {
    if !cfg!(target_os = "linux") {
        return Err("wheelpad setup is supported on Linux only".into());
    }
    command("modprobe", &["uinput"])?;
    command("udevadm", &["control", "--reload-rules"])?;
    command("udevadm", &["trigger", "--subsystem-match=input"])?;
    command(
        "udevadm",
        &[
            "trigger",
            "--subsystem-match=misc",
            "--sysname-match=uinput",
        ],
    )?;
    command("udevadm", &["settle"])?;
    configure_hyprland()?;
    command("systemctl", &["--user", "daemon-reload"])?;
    if !no_start {
        command(
            "systemctl",
            &["--user", "enable", "--now", "wheelpad.service"],
        )?;
    }
    println!(
        "wheelpad setup complete{}",
        if no_start {
            " (service not started)"
        } else {
            ""
        }
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hook_is_exact_and_idempotent() {
        assert_eq!(
            HYPRLAND_HOOK,
            "dofile(\"/usr/share/wheelpad/wheelpad.lua\")"
        );
        let text = format!("before\n{HYPRLAND_HOOK}\nafter\n");
        assert!(text.lines().any(|line| line.trim() == HYPRLAND_HOOK));
    }
}
