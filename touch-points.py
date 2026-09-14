import struct

DEVICE = "/dev/input/event11"

EVENT = struct.Struct("llHHi")

EV_SYN = 0
EV_KEY = 1
EV_ABS = 3

SYN_REPORT = 0
BTN_TOUCH = 330
ABS_X = 0
ABS_Y = 1

x = None
y = None
touching = False
last_touching = False

with open(DEVICE, "rb") as dev:
    print("Touch one point and release it.")
    print("Touch the pad. Ctrl+C to stop.")

    while True:
        data = dev.read(EVENT.size)
        sec, user, event_type, code, value = EVENT.unpack(data)

        if event_type == EV_ABS:
            if code == ABS_X:
                x = value
            elif code == ABS_Y:
                y = value

        elif event_type == EV_KEY and code == BTN_TOUCH:
            touching = bool(value)
        elif event_type == EV_SYN and code == SYN_REPORT:
            if touching and not last_touching:
                print(f"DOWN x={x} y={y}")
            elif not touching and last_touching:
                print(f"UP x={x} y={y}")

            last_touching = touching
