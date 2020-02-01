use crate::{Angle, Orientation, Point, Vector};
use std::f64::consts::PI;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Arc {
    centre: Point,
    radius: f64,
    start_angle: Angle,
    sweep_angle: Angle,
}

impl Arc {
    pub fn from_centre_radius(
        centre: Point,
        radius: f64,
        start_angle: Angle,
        sweep_angle: Angle,
    ) -> Self {
        debug_assert!(radius > 0.0);

        Arc {
            centre,
            radius,
            start_angle,
            sweep_angle,
        }
    }

    pub fn from_three_points(
        start: Point,
        middle: Point,
        end: Point,
    ) -> Option<Self> {
        debug_assert!(
            Orientation::of(start, middle, end) != Orientation::Collinear
        );

        let centre = crate::centre_of_three_points(start, middle, end)?;
        let radius = (start - centre).length();
        let start_angle = (start - centre).angle_from_x_axis();
        let sweep_angle = sweep_angle_from_3_points(start, middle, end, centre);

        Some(Arc::from_centre_radius(
            centre,
            radius,
            start_angle,
            sweep_angle,
        ))
    }

    pub const fn centre(self) -> Point { self.centre }

    pub const fn radius(self) -> f64 { self.radius }

    pub const fn start_angle(self) -> Angle { self.start_angle }

    pub const fn sweep_angle(self) -> Angle { self.sweep_angle }

    pub fn end_angle(self) -> Angle { self.start_angle() + self.sweep_angle() }

    pub fn is_anticlockwise(self) -> bool { self.sweep_angle > Angle::zero() }

    pub fn is_clockwise(self) -> bool { self.sweep_angle < Angle::zero() }

    pub fn start(self) -> Point { self.point_at(Angle::zero()) }

    pub fn end(self) -> Point { self.point_at(self.sweep_angle()) }

    pub fn point_at(self, angle: Angle) -> Point {
        let angle = self.start_angle() + angle;
        let (x, y) = angle.sin_cos();
        let r = self.radius();
        let displacement_from_centre = Vector::new(r * x, r * y);

        self.centre() + displacement_from_centre
    }

    pub fn contains_angle(self, angle: Angle) -> bool {
        let start_angle = self.start_angle();
        let end_angle = self.end_angle();

        let (min, max) = if start_angle < end_angle {
            (start_angle, end_angle)
        } else {
            (end_angle, start_angle)
        };

        (min <= angle) && (angle <= max)
    }

    pub fn is_minor_arc(&self) -> bool {
        self.sweep_angle().radians.abs() <= PI
    }

    pub fn is_major_arc(&self) -> bool { !self.is_minor_arc() }
}

fn sweep_angle_from_3_points(
    start: Point,
    middle: Point,
    end: Point,
    centre: Point,
) -> Angle {
    debug_assert!(
        Orientation::of(start, middle, end) != Orientation::Collinear
    );

    let start_ray = start - centre;
    let end_ray = end - centre;
    let orientation = Orientation::of(start, middle, end);
    let angular_difference =
        end_ray.angle_from_x_axis() - start_ray.angle_from_x_axis();

    if angular_difference.radians > 0.0 && orientation == Orientation::Clockwise
    {
        angular_difference - Angle::two_pi()
    } else if angular_difference.radians < 0.0
        && orientation == Orientation::Anticlockwise
    {
        Angle::two_pi() - angular_difference
    } else {
        angular_difference
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_contains_angle {
        ($name:ident, $arc:expr, $degrees:expr => $expected:expr) => {
            #[test]
            fn $name() {
                let arc: Arc = $arc;
                let angle = Angle::degrees($degrees);

                let got = arc.contains_angle(angle);

                assert_eq!(got, $expected);
            }
        };
    }

    test_contains_angle!(middle_of_ne_quadrant,
        Arc::from_centre_radius(Point::zero(), 1.0, Angle::zero(), Angle::frac_pi_2()),
        45.0 => true);
    test_contains_angle!(start_of_arc,
        Arc::from_centre_radius(Point::zero(), 1.0, Angle::zero(), Angle::frac_pi_2()),
        0.0 => true);
    test_contains_angle!(end_of_arc,
        Arc::from_centre_radius(Point::zero(), 1.0, Angle::zero(), Angle::frac_pi_2()),
        90.0 => true);
    test_contains_angle!(outside_of_arc,
        Arc::from_centre_radius(Point::zero(), 1.0, Angle::zero(), Angle::frac_pi_4()),
        90.0 => false);
    test_contains_angle!(inside_reverse_arc,
        Arc::from_centre_radius(Point::zero(), 1.0, Angle::frac_pi_4(), -Angle::frac_pi_4()),
        45.0 => true);

    #[test]
    fn arc_from_three_points() {
        let a = Point::new(10.0, 0.0);
        let b = Point::new(0.0, 10.0);
        let c = Point::new(-10.0, 0.0);
        let expected = Arc::from_centre_radius(
            Point::zero(),
            10.0,
            Angle::zero(),
            Angle::pi(),
        );

        let got = Arc::from_three_points(a, b, c).unwrap();

        assert_eq!(got, expected);
    }

    #[test]
    fn clockwise_arc_from_three_points() {
        let a = Point::new(10.0, 0.0);
        let b = Point::new(0.0, 10.0);
        let c = Point::new(-10.0, 0.0);
        let expected = Arc::from_centre_radius(
            Point::zero(),
            10.0,
            Angle::pi(),
            -Angle::pi(),
        );

        let got = Arc::from_three_points(c, b, a).unwrap();

        assert_eq!(got, expected);
    }
}
