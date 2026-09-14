import math
import struct

DEVICE = "/dev/input/event11"

CX = 3618.68
CY = 2941.66
RADIUS =1734.19

EVENT = struct.Struct("llHHi")

EV_SYN = 0
EV_KEY = 1
EV_ABS = 3

SYN_REPORT = 0
BTN_TOUCH = 330
ABS_X = 0
ABS_y = 1

x = None
y = None
touching = False
last_angle = None

with open(DEVICE,"rb") as dev:
    print("Trace the wheel pad. Ctrl+C to stop.")

    while True:
        data = dev.read(EVENT.size)
        sec, usec, event_type, code,value = EVENT.unpack(data)

        if event_type == EV_ABS:
            if code == ABS_X:
                x = value
            elif code == ABS_y:
                y=value
        elif event_type == EV_KEY and code == BTN_TOUCH:
            touching = bool(value)
            if not touching:
                last_angle = None
        elif event_type == EV_SYN and code == SYN_REPORT:
            if not touching or x is None or y is None:
                continue

            dx = x - CX
            dy = y - CY

            radius = math.hypot(dx,dy)

            if not (RADIUS * 0.75 <=radius <= RADIUS*1.25):
                last_angle = None
                continue

            angle = math.atan2(dx,dy)

            if last_angle is not None:
                delta = angle - last_angle

                if delta > math.pi:
                    delta -= 2*math.pi
                elif delta < -math.pi:
                    delta += 2*math.pi

                print(
                        f"x={x:4d} y ={y:4d}"
                        f"angle={math.degrees(angle):7.2f}"
                        f"delta={math.degrees(delta):+6.2f}"
                )
            last_angle = angle
