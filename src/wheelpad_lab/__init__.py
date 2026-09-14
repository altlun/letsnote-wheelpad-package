"""Circular scrolling for the Panasonic Let's Note touchpad."""
import argparse
from contextlib import ExitStack, closing
import math
import sys

from .gesture import WheelGesture


def positive(value):
    value = float(value)
    if not math.isfinite(value) or value <= 0:
        raise argparse.ArgumentTypeError("must be a finite positive number")
    return value


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--device", help="input event path; default: detect Synaptics touchpad")
    parser.add_argument("--dry-run", action="store_true", help="print steps without injecting input")
    parser.add_argument("--check", action="store_true", help="check device and uinput access, then exit")
    parser.add_argument("--reverse", action="store_true")
    parser.add_argument("--degrees-per-step", type=positive, default=15.0)
    args = parser.parse_args()
    from evdev import InputDevice, UInput, ecodes as e, list_devices
    try:
        with ExitStack() as stack:
            if args.device:
                device = stack.enter_context(closing(InputDevice(args.device)))
            else:
                candidates = []
                for path in list_devices():
                    candidate = stack.enter_context(closing(InputDevice(path)))
                    if candidate.name == "SynPS/2 Synaptics TouchPad":
                        candidates.append(candidate)
                if len(candidates) != 1:
                    raise RuntimeError("Expected one Synaptics touchpad; specify --device. Check /dev/input permissions.")
                device = candidates[0]
            caps = device.capabilities()
            axes = {code for code, _ in caps.get(e.EV_ABS, [])}
            if not {e.ABS_X, e.ABS_Y} <= axes or e.BTN_TOUCH not in caps.get(e.EV_KEY, []):
                raise RuntimeError("Device must expose ABS_X, ABS_Y and BTN_TOUCH")
            print(f"Touchpad: {device.path} ({device.name})", flush=True)
            wheel = None
            if not args.dry_run:
                wheel = stack.enter_context(closing(UInput({
                    e.EV_KEY: [e.BTN_LEFT],
                    e.EV_REL: [e.REL_X, e.REL_Y, e.REL_WHEEL],
                }, name="LetsNote Wheelpad Scroll")))
            if args.check:
                print("Device access OK" + ("; virtual wheel creation OK" if wheel else ""))
                return
            gesture = WheelGesture(degrees_per_step=args.degrees_per_step)
            x = y = None
            touching = False
            blocked = False
            dropped = False
            multi = set()
            print("Trace the outer circle; Ctrl+C to stop.", flush=True)
            for event in device.read_loop():
                if event.type == e.EV_SYN and event.code == e.SYN_DROPPED:
                    gesture.reset()
                    dropped = True
                    continue
                if dropped:
                    if event.type == e.EV_SYN and event.code == e.SYN_REPORT:
                        dropped = False
                        blocked = True
                        touching = e.BTN_TOUCH in device.active_keys()
                        x = y = None
                        multi.clear()
                    continue
                if event.type == e.EV_ABS:
                    if event.code == e.ABS_X:
                        x = event.value
                    elif event.code == e.ABS_Y:
                        y = event.value
                elif event.type == e.EV_KEY:
                    if event.code == e.BTN_TOUCH:
                        touching = bool(event.value)
                    elif event.code in (e.BTN_TOOL_DOUBLETAP, e.BTN_TOOL_TRIPLETAP,
                                        e.BTN_TOOL_QUADTAP, e.BTN_TOOL_QUINTTAP):
                        if event.value:
                            multi.add(event.code)
                        else:
                            multi.discard(event.code)
                elif event.type == e.EV_SYN and event.code == e.SYN_REPORT:
                    if not touching:
                        blocked = False
                        x = y = None
                    if multi:
                        blocked = True
                    steps = gesture.update(x, y, touching and not blocked)
                    if args.reverse:
                        steps = -steps
                    if steps:
                        if wheel:
                            wheel.write(e.EV_REL, e.REL_WHEEL, steps)
                            wheel.syn()
                        else:
                            print(f"wheel={steps:+d}", flush=True)
    except KeyboardInterrupt:
        pass
    except (OSError, RuntimeError) as error:
        print(f"wheelpad-lab: {error}", file=sys.stderr)
        return 1
    return 0
