use crate::{Orientation, Vector};
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
        debug_assert!(0.0 <= start_angle && start_angle <= 2.0 * PI);
        debug_assert!(-2.0 * PI <= sweep_angle && sweep_angle <= 2.0 * PI);
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
    ) -> Option<Self> {
        debug_assert!(
            Vector::orientation(start, middle, end) != Orientation::Collinear
        );

        let centre = Vector::centre_of_three_points(start, middle, end)?;
        let radius = (start - centre).length();
        let start_angle = (start - centre).angle();
        let sweep_angle = sweep_angle_from_3_points(start, middle, end);

        Some(Arc::from_centre_radius(
            centre,
            radius,
            start_angle,
            sweep_angle,
        ))
    }

    pub const fn centre(self) -> Vector { self.centre }

    pub const fn radius(self) -> f64 { self.radius }

    pub const fn start_angle(self) -> f64 { self.start_angle }

    pub const fn sweep_angle(self) -> f64 { self.sweep_angle }

    pub fn end_angle(self) -> f64 { self.start_angle() + self.sweep_angle() }

    pub fn is_anticlockwise(self) -> bool { self.sweep_angle > 0.0 }

    pub fn is_clockwise(self) -> bool { self.sweep_angle < 0.0 }

    pub fn start(self) -> Vector { self.point_at(0.0) }

    pub fn end(self) -> Vector { self.point_at(self.sweep_angle()) }

    pub fn point_at(self, angle: f64) -> Vector {
        assert!(0.0 <= angle && angle <= self.sweep_angle());

        self.centre()
            + Vector::from_r_theta(self.radius(), self.start_angle() + angle)
    }

    pub fn contains_angle(self, angle: f64) -> bool {
        let start_angle = self.start_angle();
        let end_angle = self.end_angle();

        let (min, max) = if start_angle < end_angle {
            (start_angle, end_angle)
        } else {
            (end_angle, start_angle)
        };

        let angular_difference = (max - min).abs();
        let between_max_and_min = min <= angle && angle <= max;

        if (self.is_major_arc() && angular_difference > PI)
            || (self.is_minor_arc() && angular_difference < PI)
        {
            between_max_and_min
        } else {
            !between_max_and_min
        }
    }

    pub fn is_minor_arc(&self) -> bool { self.sweep_angle().abs() < PI }

    pub fn is_major_arc(&self) -> bool { !self.is_minor_arc() }
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
