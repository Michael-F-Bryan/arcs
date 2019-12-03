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

    pub fn is_anticlockwise(self) -> f64 { self.sweep_angle > 0.0 }

    pub fn is_clockwise(self) -> f64 { self.sweep_angle < 0.0 }
}

fn sweep_angle_from_3_points(
    start: Vector,
    centre: Vector,
    end: Vector,
) -> f64 {
    debug_assert!(
        Vector::orientation(start, middle, end) != Orientation::Collinear
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
