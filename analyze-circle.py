import csv
import numpy as np

with open("circle.csv") as f:
    rows = list(csv.DictReader(f))

x = np.array([int(r["x"]) for r in rows], dtype=float)
y = np.array([int(r["y"]) for r in rows], dtype=float)

A = np.column_stack((2*x,2*y,np.ones(len(x))))
b = x**2 + y**2

cx,cy,c = np.linalg.lstsq(A,b,rcond=None)[0]
radius = np.sqrt(c + cx**2 + cy**2)

distance = np.sqrt((x - cx)**2+(y- cy)**2)
error = distance - radius

print(f"points : {len(x)}")
print(f"center : ({cx:.2f},{cy:.2f})")
print(f"radius : {radius:.2f}")
print(f"RMSE: {np.sqrt(np.mean(error**2)):.2f}")
print(f"max err: {np.max(np.abs(error)):.2f}")
