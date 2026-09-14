mod gesture;
mod pointer;
mod setup;
use clap::{Parser, Subcommand};
use evdev::{
    raw_stream::RawDevice, uinput::VirtualDevice, AbsoluteAxisCode as Abs, AttributeSet, EventType,
    InputEvent, KeyCode as Key, RelativeAxisCode as Rel, UinputAbsSetup,
};
use std::{error::Error, path::PathBuf};

#[derive(Parser)]
#[command(
    version,
    about = "Let's Note circular scrolling for Wayland / Hyprland"
)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
    #[arg(long)]
    device: Option<PathBuf>,
    #[arg(long)]
    check: bool,
    #[arg(long)]
    dry_run: bool,
    #[arg(long)]
    reverse: bool,
    #[arg(long, default_value = "15", value_parser = positive)]
    degrees_per_step: f64,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Install udev access and configure the user systemd/Hyprland startup.
    Setup {
        /// Configure files and reload udev, but do not start the service.
        #[arg(long)]
        no_start: bool,
    },
}
fn positive(s: &str) -> Result<f64, String> {
    let v: f64 = s.parse().map_err(|_| "Expected a number")?;
    if v.is_finite() && v > 0.0 {
        Ok(v)
    } else {
        Err("Expected a finite positive number".into())
    }
}
fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let mut device = if let Some(path) = args.device {
        RawDevice::open(path)?
    } else {
        let mut found = Vec::new();
        for entry in std::fs::read_dir("/dev/input")? {
            let path = entry?.path();
            if !path
                .file_name()
                .is_some_and(|n| n.to_string_lossy().starts_with("event"))
            {
                continue;
            }
            if let Ok(d) = RawDevice::open(path) {
                if d.name() == Some("SynPS/2 Synaptics TouchPad") {
                    found.push(d);
                }
            }
        }
        if found.len() != 1 {
            return Err("Expected one Synaptics touchpad; use --device /dev/input/eventN and check permissions".into());
        }
        found.remove(0)
    };
    if !device
        .supported_absolute_axes()
        .is_some_and(|a| a.contains(Abs::ABS_X) && a.contains(Abs::ABS_Y))
        || !device
            .supported_keys()
            .is_some_and(|k| k.contains(Key::BTN_TOUCH))
    {
        return Err("Device must expose ABS_X, ABS_Y and BTN_TOUCH".into());
    }
    println!("Touchpad: {}", device.name().unwrap_or("unknown"));
    let mut wheel = if args.dry_run {
        None
    } else {
        let axes: AttributeSet<Rel> = [Rel::REL_X, Rel::REL_Y, Rel::REL_WHEEL]
            .into_iter()
            .collect();
        let keys: AttributeSet<Key> = [Key::BTN_LEFT].into_iter().collect();
        Some(
            VirtualDevice::builder()?
                .name("LetsNote Wheelpad Scroll")
                .with_keys(&keys)?
                .with_relative_axes(&axes)?
                .build()?,
        )
    };
    if args.check {
        println!(
            "Device access OK{}",
            if wheel.is_some() {
                "; virtual wheel creation OK"
            } else {
                ""
            }
        );
        return Ok(());
    }
    let mut pointer = if args.dry_run {
        None
    } else {
        Some(clone_touchpad(&device)?)
    };
    if pointer.is_some() {
        device.grab()?;
        if device.get_key_state()?.contains(Key::BTN_TOUCH) {
            return Err("Lift your fingers from the touchpad, then start wheelpad again".into());
        }
    }
    let mut state = State::default();
    for (axis, info) in device.get_absinfo()? {
        if axis == Abs::ABS_X {
            state.x = Some(info.value());
        }
        if axis == Abs::ABS_Y {
            state.y = Some(info.value());
        }
    }
    let mut filter = pointer::PointerFilter::default();
    let mut gesture = gesture::Gesture::new(args.degrees_per_step);
    let mut frame = Vec::new();
    println!("Outer rim: scroll with pointer fixed. Center: normal pointer. Ctrl+C to stop.");
    loop {
        let events: Vec<_> = device.fetch_events()?.collect();
        for event in events {
            if event.event_type() == EventType::SYNCHRONIZATION && event.code() == 3 {
                // Drop virtual devices and release the grab rather than replay corrupt contacts.
                return Err("Input queue overflow; restarting input forwarding".into());
            }
            frame.push(event);
            let steps = state.event(event, &mut gesture);
            if event.event_type() != EventType::SYNCHRONIZATION || event.code() != 0 {
                continue;
            }
            let (output, scrolling) =
                filter.frame(&frame, state.x.zip(state.y), state.touching, state.multi);
            frame.clear();
            if let Some(pointer) = &mut pointer {
                pointer.emit(&output)?;
            }
            if !scrolling {
                gesture.reset();
                continue;
            }
            let steps = if args.reverse { -steps } else { steps };
            if steps != 0 {
                if let Some(wheel) = &mut wheel {
                    wheel.emit(&[InputEvent::new(
                        EventType::RELATIVE.0,
                        Rel::REL_WHEEL.0,
                        steps,
                    )])?;
                } else {
                    println!("wheel={steps:+}");
                }
            }
        }
    }
}
fn clone_touchpad(device: &RawDevice) -> Result<VirtualDevice, Box<dyn Error>> {
    let mut builder = VirtualDevice::builder()?
        .name("LetsNote Wheelpad Touchpad")
        .input_id(device.input_id())
        .with_properties(device.properties())?;
    if let Some(keys) = device.supported_keys() {
        builder = builder.with_keys(keys)?;
    }
    if let Some(axes) = device.supported_relative_axes() {
        builder = builder.with_relative_axes(axes)?;
    }
    for (axis, info) in device.get_absinfo()? {
        builder = builder.with_absolute_axis(&UinputAbsSetup::new(axis, info))?;
    }
    builder.build().map_err(Into::into)
}

#[derive(Default)]
struct State {
    x: Option<i32>,
    y: Option<i32>,
    touching: bool,
    multi: u8,
    blocked: bool,
    dropped: bool,
}
impl State {
    fn event(&mut self, event: InputEvent, gesture: &mut gesture::Gesture) -> i32 {
        let (kind, code, value) = (event.event_type(), event.code(), event.value());
        if kind == EventType::SYNCHRONIZATION && code == 3 {
            gesture.reset();
            self.dropped = true;
            self.blocked = true;
            self.x = None;
            self.y = None;
            return 0;
        }
        if self.dropped {
            if kind == EventType::SYNCHRONIZATION && code == 0 {
                self.dropped = false;
            }
            return 0;
        }
        if kind == EventType::ABSOLUTE {
            if code == Abs::ABS_X.0 {
                self.x = Some(value);
            }
            if code == Abs::ABS_Y.0 {
                self.y = Some(value);
            }
        } else if kind == EventType::KEY {
            if code == Key::BTN_TOUCH.0 {
                self.touching = value != 0;
                if !self.touching {
                    self.blocked = false;
                    self.multi = 0;
                }
            }
            for (i, key) in [
                Key::BTN_TOOL_DOUBLETAP,
                Key::BTN_TOOL_TRIPLETAP,
                Key::BTN_TOOL_QUADTAP,
                Key::BTN_TOOL_QUINTTAP,
            ]
            .iter()
            .enumerate()
            {
                if code == key.0 {
                    if value != 0 {
                        self.multi |= 1 << i;
                        self.blocked = true;
                    } else {
                        self.multi &= !(1 << i);
                    }
                }
            }
        } else if kind == EventType::SYNCHRONIZATION && code == 0 {
            return gesture.update(if self.touching && !self.blocked && self.multi == 0 {
                self.x.zip(self.y)
            } else {
                None
            });
        }
        0
    }
}
fn main() {
    let args = Args::parse();
    let result = match args.command {
        Some(Command::Setup { no_start }) => setup::run(no_start),
        None => run(args),
    };
    if let Err(error) = result {
        eprintln!("wheelpad: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn send(
        s: &mut State,
        g: &mut gesture::Gesture,
        kind: EventType,
        code: u16,
        value: i32,
    ) -> i32 {
        s.event(InputEvent::new(kind.0, code, value), g)
    }
    #[test]
    fn multitouch_blocks_until_release() {
        let mut s = State::default();
        let mut g = gesture::Gesture::new(15.0);
        send(&mut s, &mut g, EventType::KEY, Key::BTN_TOUCH.0, 1);
        send(&mut s, &mut g, EventType::KEY, Key::BTN_TOOL_DOUBLETAP.0, 1);
        send(&mut s, &mut g, EventType::KEY, Key::BTN_TOOL_DOUBLETAP.0, 0);
        assert!(s.blocked);
        send(&mut s, &mut g, EventType::KEY, Key::BTN_TOUCH.0, 0);
        assert!(!s.blocked);
        assert_eq!(s.multi, 0);
    }
    #[test]
    fn dropped_events_require_explicit_release() {
        let mut s = State::default();
        let mut g = gesture::Gesture::new(15.0);
        send(&mut s, &mut g, EventType::KEY, Key::BTN_TOUCH.0, 1);
        send(&mut s, &mut g, EventType::SYNCHRONIZATION, 3, 0);
        send(&mut s, &mut g, EventType::KEY, Key::BTN_TOUCH.0, 0);
        send(&mut s, &mut g, EventType::SYNCHRONIZATION, 0, 0);
        assert!(s.blocked);
        send(&mut s, &mut g, EventType::KEY, Key::BTN_TOUCH.0, 0);
        assert!(!s.blocked);
    }
    #[test]
    fn rejects_invalid_sensitivity() {
        for value in ["0", "-1", "NaN", "inf", "oops"] {
            assert!(positive(value).is_err());
        }
        assert_eq!(positive("10").unwrap(), 10.0);
    }
}
