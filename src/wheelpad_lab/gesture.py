"""Circle recognition, independent of Linux device access."""
import math
from dataclasses import dataclass


@dataclass
class WheelGesture:
    cx: float = 3618.68
    cy: float = 2941.66
    radius: float = 1734.19
    degrees_per_step: float = 15.0
    last_angle: float | None = None
    remainder: float = 0.0

    def reset(self):
        self.last_angle = None
        self.remainder = 0.0

    def update(self, x, y, touching=True):
        if not touching or x is None or y is None:
            self.reset()
            return 0
        dx, dy = x - self.cx, y - self.cy
        if not self.radius * 0.75 <= math.hypot(dx, dy) <= self.radius * 1.25:
            self.reset()
            return 0
        angle = math.atan2(dy, dx)
        previous, self.last_angle = self.last_angle, angle
        if previous is None:
            return 0
        delta = (angle - previous + math.pi) % math.tau - math.pi
        # Reject discontinuities, including a finger landing elsewhere.
        if abs(delta) > math.pi / 4:
            self.remainder = 0.0
            return 0
        self.remainder += math.degrees(delta)
        steps = math.trunc(self.remainder / self.degrees_per_step)
        self.remainder -= steps * self.degrees_per_step
        return -steps  # Screen coordinates: clockwise scrolls down.
