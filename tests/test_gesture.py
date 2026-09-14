import math
import unittest
from wheelpad_lab.gesture import WheelGesture


class GestureTests(unittest.TestCase):
    def point(self, g, angle):
        a = math.radians(angle)
        return g.update(g.cx + g.radius * math.cos(a), g.cy + g.radius * math.sin(a))

    def test_clockwise_and_counterclockwise(self):
        for direction in (1, -1):
            g = WheelGesture()
            total = sum(self.point(g, direction * a) for a in range(0, 361, 5))
            self.assertIn(total, (-24, -23) if direction == 1 else (23, 24))

    def test_wrap_is_small(self):
        g = WheelGesture()
        self.point(g, 179)
        self.assertEqual(self.point(g, -179), 0)
        self.assertAlmostEqual(g.remainder, 2)

    def test_release_resets_fraction(self):
        g = WheelGesture()
        self.point(g, 0)
        self.point(g, 10)
        g.update(None, None, False)
        self.assertEqual(self.point(g, 30), 0)
        self.assertEqual(g.remainder, 0)

    def test_center_and_jump_do_not_scroll(self):
        g = WheelGesture()
        self.point(g, 0)
        self.assertEqual(self.point(g, 100), 0)
        self.assertEqual(g.update(g.cx, g.cy), 0)
        self.assertEqual(self.point(g, 180), 0)

    def test_stationary(self):
        g = WheelGesture()
        self.assertEqual(sum(self.point(g, 45) for _ in range(100)), 0)


if __name__ == '__main__':
    unittest.main()
