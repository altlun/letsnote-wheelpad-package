use std::f64::consts::{PI, TAU};

pub fn on_rim((x, y): (i32, i32)) -> bool {
    (1734.19 * 0.75..=1734.19 * 1.25).contains(&(x as f64 - 3618.68).hypot(y as f64 - 2941.66))
}

pub struct Gesture {
    last: Option<f64>,
    remainder: f64,
    degrees: f64,
}
impl Gesture {
    pub fn new(degrees: f64) -> Self {
        Self {
            last: None,
            remainder: 0.0,
            degrees,
        }
    }
    pub fn reset(&mut self) {
        self.last = None;
        self.remainder = 0.0;
    }
    pub fn update(&mut self, point: Option<(i32, i32)>) -> i32 {
        let Some((x, y)) = point else {
            self.reset();
            return 0;
        };
        let (dx, dy) = (x as f64 - 3618.68, y as f64 - 2941.66);
        if !on_rim((x, y)) {
            self.reset();
            return 0;
        }
        let angle = dy.atan2(dx);
        let previous = self.last.replace(angle);
        let Some(previous) = previous else {
            return 0;
        };
        let delta = (angle - previous + PI).rem_euclid(TAU) - PI;
        if delta.abs() > PI / 4.0 {
            self.remainder = 0.0;
            return 0;
        }
        self.remainder += delta.to_degrees();
        let steps = (self.remainder / self.degrees).trunc() as i32;
        self.remainder -= steps as f64 * self.degrees;
        -steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn point(a: f64) -> Option<(i32, i32)> {
        let a = a.to_radians();
        Some((
            (3618.68 + 1734.19 * a.cos()).round() as i32,
            (2941.66 + 1734.19 * a.sin()).round() as i32,
        ))
    }
    #[test]
    fn both_directions_and_wrap() {
        for direction in [-1, 1] {
            let mut g = Gesture::new(15.0);
            let steps: i32 = (0..=360)
                .step_by(5)
                .map(|a| g.update(point((a * direction) as f64)))
                .sum();
            assert!((23..=24).contains(&steps.abs()));
            assert_eq!(steps.signum(), -direction);
        }
    }
    #[test]
    fn release_center_jump_stationary() {
        let mut g = Gesture::new(15.0);
        g.update(point(0.0));
        g.update(point(10.0));
        g.update(None);
        assert_eq!(g.update(point(30.0)), 0);
        assert_eq!(g.update(point(150.0)), 0);
        assert_eq!(g.update(Some((3619, 2942))), 0);
        for _ in 0..100 {
            assert_eq!(g.update(point(45.0)), 0);
        }
    }
}
