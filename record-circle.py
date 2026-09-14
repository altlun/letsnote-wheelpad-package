import csv
import struct
import time

DEVICE = "/dev/input/event11"
OUTPUT = "circle.csv"

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
points = []

print("please touch touchpad")

with open(DEVICE,"rb") as dev:
    while True:
        data = dev.read(EVENT.size)
        sec,usec,event_type,code,value = EVENT.unpack(data)

        if event_type == EV_ABS:
            if code == ABS_X:
                x = value
            elif code == ABS_Y:
                y = value
        elif event_type == EV_KEY and code == BTN_TOUCH:
            touching = bool(value)
        elif event_type == EV_SYN and code == SYN_REPORT:
            if touching and x is not None and y is not None:
                points.append((time.time(),x,y))
            elif not touching and points:
                break

with open(OUTPUT,"w",newline="") as f:
    wrtier = csv.writer(f)
    wrtier.writerow(["time","x","y"])
    wrtier.writerows(points)

print(f"{len(points)} porints recorded")
print(F"saved: {OUTPUT}")
