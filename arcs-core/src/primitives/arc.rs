use crate::primitives::{Orientation, Vector};
use std::f64::consts::PI;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Arc {
    centre: Vector,
    radius: f64,
    start_angle: f64,
    sweep_angle: f64,
}

impl Arc {
    pub fn from_centre_radius(
        centre: Vector,
        radius: f64,
        start_angle: f64,
        sweep_angle: f64,
    ) -> Self {
        debug_assert!(0.0 <= sweep_angle && sweep_angle <= 2.0 * PI);
        debug_assert!(0.0 <= start_angle && start_angle <= 2.0 * PI);
        debug_assert!(radius > 0.0);

        Arc {
            centre,
            radius,
            start_angle,
            sweep_angle,
        }
    }

    pub fn from_three_points(
        start: Vector,
        middle: Vector,
        end: Vector,
    ) -> Self {
        debug_assert!(
            Vector::orientation(start, middle, end) != Orientation::Collinear
        );

        let centre = Vector::centre_of_three_points(start, middle, end);
        let radius = (start - centre).length();
        let start_angle = (start - centre).angle();
        let sweep_angle = sweep_angle_from_3_points(start, middle, end);

        Arc::from_centre_radius(centre, radius, start_angle, sweep_angle)
    }

    pub fn centre(self) -> Vector { self.centre }

    pub fn radius(self) -> f64 { self.radius }

    pub fn start_angle(self) -> f64 { self.start_angle }

    pub fn sweep_angle(self) -> f64 { self.sweep_angle }

    pub fn end_angle(self) -> f64 { self.start_angle() + self.sweep_angle() }

    pub fn is_anticlockwise(self) -> bool { self.sweep_angle > 0.0 }

    pub fn is_clockwise(self) -> bool { self.sweep_angle < 0.0 }

    pub fn start(self) -> Vector { self.point_at(0.0) }

    pub fn end(self) -> Vector { self.point_at(self.sweep_angle()) }

    pub fn point_at(self, angle: f64) -> Vector {
        assert!(0.0 <= angle && angle <= self.sweep_angle().abs());

        self.centre()
            + Vector::from_r_theta(self.radius(), self.start_angle() + angle)
    }

    pub fn approximate(self, quality: f64) -> impl Iterator<Item = Vector> {
        // Draw a chord between points A and B on a circle with centre C.
        // Draw a line which bisects the angle ACB and intersects with the
        // chord at point D.
        // The distance from D to the arc is our "quality"
        // (i.e. |CD| + quality = radius).
        //
        // From the triangle DCB:
        //   cos(θ/2) = |CD|/R
        //   cos(θ/2) = 1 - quality/R
        //
        //  where θ is the angle swept by a chord with the desired "quality".
        //
        // # line segments to approximate with the specified quality:
        //   N = ⌈SweepAngle/θ⌉

        let (steps, delta) = if quality <= 0.0 || self.radius() <= quality {
            (1, self.sweep_angle())
        } else {
            let cos_theta_on_two = 1.0 - quality / self.radius();
            let theta = cos_theta_on_two.acos() * 2.0;
            let line_segment_count = self.sweep_angle() / theta;

            // make sure we always have at least 2 points
            let line_segment_count = f64::max(line_segment_count, 2.0);
            let actual_step = self.sweep_angle() / line_segment_count;

            (line_segment_count.ceil().abs() as usize, actual_step)
        };

        (0..steps + 1).map(move |i| self.point_at(i as f64 * delta))
    }
}

fn sweep_angle_from_3_points(
    start: Vector,
    centre: Vector,
    end: Vector,
) -> f64 {
    debug_assert!(
        Vector::orientation(start, centre, end) != Orientation::Collinear
    );

    let start_ray = start - centre;
    let end_ray = end - centre;
    let orientation = Vector::orientation(start, centre, end);
    let angular_difference = end_ray.angle() - start_ray.angle();

    if angular_difference > 0.0 && orientation == Orientation::Clockwise {
        angular_difference - 2.0 * PI
    } else if angular_difference < 0.0
        && orientation == Orientation::Anticlockwise
    {
        2.0 * PI - angular_difference
    } else {
        angular_difference
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approximate_arc_with_points() {
        let arc = Arc::from_centre_radius(Vector::zero(), 100.0, 0.0, PI / 2.0);
        let quality = 10.0;

        let pieces: Vec<_> = arc.approximate(quality).collect();

        for &piece in &pieces {
            let error = arc.radius() - (piece - arc.centre()).length();
            assert!(error < quality);
        }
        assert_eq!(arc.start(), *pieces.first().unwrap());
        assert_eq!(arc.end(), *pieces.last().unwrap());
    }
}
