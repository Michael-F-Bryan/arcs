#![allow(missing_docs)]

use crate::{Angle, Orientation};
use euclid::{Point2D, Vector2D};
use std::f64::consts::PI;

/// A circle segment.
#[derive(Debug, PartialEq)]
pub struct Arc<S> {
    centre: Point2D<f64, S>,
    radius: f64,
    start_angle: Angle,
    sweep_angle: Angle,
}

impl<S> Arc<S> {
    /// Create an [`Arc`] based upon its centre and radius.
    ///
    /// # Examples
    pub fn from_centre_radius(
        centre: Point2D<f64, S>,
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

    /// Try to find the [`Arc`] which will pass through three points.
    ///
    /// # Examples
    ///
    /// You can use this constructor in the normal way.
    ///
    /// ```rust
    /// use arcs_core::{Arc, Point};
    ///
    /// let right = Point::new(10.0, 0.0);
    /// let above = Point::new(0.0, 10.0);
    /// let left = Point::new(-10.0, 0.0);
    ///
    /// let got = Arc::from_three_points(right, above, left).unwrap();
    ///
    /// assert_eq!(got.centre(), Point::zero());
    /// assert_eq!(got.radius(), 10.0);
    /// assert!(got.is_anticlockwise());
    /// ```
    ///
    /// This will fail if the three points are [`Orientation::Collinear`].
    ///
    /// ```rust
    /// use arcs_core::{Arc, Point};
    ///
    /// let start = Point::new(0.0, 0.0);
    /// let middle = Point::new(10.0, 0.0);
    /// let end = Point::new(20.0, 0.0);
    ///
    /// let got = Arc::from_three_points(start, middle, end);
    ///
    /// assert!(got.is_none());
    /// ```
    pub fn from_three_points(
        start: Point2D<f64, S>,
        middle: Point2D<f64, S>,
        end: Point2D<f64, S>,
    ) -> Option<Self> {
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

    /// The [`Arc`]'s centre point.
    pub const fn centre(self) -> Point2D<f64, S> { self.centre }

    /// The [`Arc`]'s radius.
    pub const fn radius(self) -> f64 { self.radius }

    pub const fn start_angle(self) -> Angle { self.start_angle }

    pub const fn sweep_angle(self) -> Angle { self.sweep_angle }

    pub fn end_angle(self) -> Angle { self.start_angle() + self.sweep_angle() }

    pub fn is_anticlockwise(self) -> bool { self.sweep_angle > Angle::zero() }

    pub fn is_clockwise(self) -> bool { self.sweep_angle < Angle::zero() }

    pub fn start(self) -> Point2D<f64, S> { self.point_at(Angle::zero()) }

    pub fn end(self) -> Point2D<f64, S> { self.point_at(self.sweep_angle()) }

    pub fn point_at(self, angle: Angle) -> Point2D<f64, S> {
        let angle = self.start_angle() + angle;
        let (sin, cos) = angle.sin_cos();
        let r = self.radius();

        self.centre() + Vector2D::new(r * cos, r * sin)
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

fn sweep_angle_from_3_points<S>(
    start: Point2D<f64, S>,
    middle: Point2D<f64, S>,
    end: Point2D<f64, S>,
    centre: Point2D<f64, S>,
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

impl<S> Copy for Arc<S> {}

impl<S> Clone for Arc<S> {
    fn clone(&self) -> Self { *self }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DrawingSpace, Point, Vector};
    use euclid::approxeq::ApproxEq;

    macro_rules! test_contains_angle {
        ($name:ident, $arc:expr, $degrees:expr => $expected:expr) => {
            #[test]
            fn $name() {
                let arc: Arc<DrawingSpace> = $arc;
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

    #[test]
    fn basic_properties() {
        let centre = Point::new(5.0, 100.0);
        let radius = 10.0;
        let start_angle = Angle::zero();
        let sweep_angle = Angle::frac_pi_2();

        let arc =
            Arc::from_centre_radius(centre, 10.0, start_angle, sweep_angle);

        assert_eq!(arc.start_angle(), start_angle);
        assert_eq!(arc.sweep_angle(), sweep_angle);
        assert_eq!(arc.end_angle(), start_angle + sweep_angle);
        assert_eq!(arc.centre(), centre);
        assert_eq!(arc.radius(), radius);
        assert_eq!(arc.radius(), radius);
        assert_eq!(arc.start(), centre + Vector::new(radius, 0.0));
        let expected_end = centre + Vector::new(0.0, radius);
        assert!(arc.end().approx_eq(&expected_end));
    }
}
