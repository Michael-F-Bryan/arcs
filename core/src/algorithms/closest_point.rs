use crate::{
    algorithms::Length,
    primitives::{Arc, Line},
};
use euclid::{approxeq::ApproxEq, Point2D, Scale, Vector2D};
use std::iter::FromIterator;

/// Find the location on an object which is closest to a target point.
///
/// # Usage
///
/// When trying to find the closest point to a [`Line`] you have the simple
/// cases, like when the point is directly on or above the line.
///
/// ```rust
/// # use arcs_core::{primitives::Line, algorithms::{ClosestPoint, Closest}};
/// # type Point = euclid::default::Point2D<f64>;
/// let start = Point::new(-10.0, 0.0);
/// let line = Line::new(start, Point::new(10.0, 0.0));
///
/// // a point on the line is closest to itself
/// assert_eq!(line.closest_point(start), Closest::One(start));
///
/// // somewhere directly above the line
/// let random_point = Point::new(8.0, -5.0);
/// assert_eq!(
///     line.closest_point(random_point),
///     Closest::One(Point::new(8.0, 0.0)),
/// );
/// ```
///
/// You can also have situations where there are multiple locations on an object
/// which are closest to the part. For example, somewhere halfway between the
/// start and end of an [`Arc`].
///
/// ```rust
/// # use arcs_core::{primitives::Arc, algorithms::{ClosestPoint, Closest}, Angle};
/// # type Point = euclid::default::Point2D<f64>;
/// let arc = Arc::from_centre_radius(
///     Point::new(0.0, 0.0),
///     10.0,
///     Angle::zero(),
///     Angle::frac_pi_2() * 3.0,
/// );
///
/// let start = arc.start();
/// let end = arc.end();
/// let midpoint = start.lerp(end, 0.5);
///
/// assert_eq!(
///     arc.closest_point(midpoint),
///     Closest::Many(vec![start, end]),
/// );
/// ```
///
/// And by definition, there are infinitely many points on an arc which are
/// close to the centre.
///
/// ```rust
/// # use arcs_core::{primitives::Arc, algorithms::{ClosestPoint, Closest}, Angle};
/// # type Point = euclid::default::Point2D<f64>;
/// let arc = Arc::from_centre_radius(
///     Point::new(0.0, 0.0),
///     10.0,
///     Angle::zero(),
///     Angle::pi(),
/// );
///
/// assert_eq!(arc.closest_point(arc.centre()), Closest::Infinite);
/// ```
pub trait ClosestPoint<Space> {
    /// Calculate the closest point to `target`.
    fn closest_point(&self, target: Point2D<f64, Space>) -> Closest<Space>;
}

impl<'c, Space, C: ClosestPoint<Space> + ?Sized> ClosestPoint<Space> for &'c C {
    fn closest_point(&self, target: Point2D<f64, Space>) -> Closest<Space> {
        (*self).closest_point(target)
    }
}

impl<Space> ClosestPoint<Space> for Point2D<f64, Space> {
    fn closest_point(&self, _target: Point2D<f64, Space>) -> Closest<Space> {
        Closest::One(*self)
    }
}

impl<Space> ClosestPoint<Space> for Line<Space> {
    fn closest_point(&self, target: Point2D<f64, Space>) -> Closest<Space> {
        if self.length().approx_eq(&0.0) {
            return Closest::One(self.start);
        }

        let start = self.start;
        let displacement = self.displacement();

        // equation of the line: start + t * displacement, where 0 <= t <= 1

        let t = Vector2D::dot(target - start, displacement)
            / (self.length() * self.length());

        Closest::One(if t <= 0.0 {
            self.start
        } else if t >= 1.0 {
            self.end
        } else {
            start + Scale::new(t).transform_vector(displacement)
        })
    }
}

impl<Space> ClosestPoint<Space> for Arc<Space> {
    fn closest_point(&self, target: Point2D<f64, Space>) -> Closest<Space> {
        let radial = target - self.centre();

        if radial.length().approx_eq(&0.0) {
            return Closest::Infinite;
        }

        let angle_of_closest_point = radial.angle_from_x_axis();
        let ideal_closest_point =
            self.centre() + radial.normalize() * self.radius();

        if self.contains_angle(angle_of_closest_point) {
            return Closest::One(ideal_closest_point);
        }

        let to_start = (self.start() - ideal_closest_point).length();
        let to_end = (self.end() - ideal_closest_point).length();

        if to_start.approx_eq(&to_end) {
            Closest::Many(vec![self.start(), self.end()])
        } else if to_start < to_end {
            Closest::One(self.start())
        } else {
            Closest::One(self.end())
        }
    }
}

/// An enum containing the different possible solutions for
/// [`ClosestPoint::closest_point()`].
#[derive(Debug, Clone, PartialEq)]
pub enum Closest<Space> {
    /// There are infinitely solutions.
    Infinite,
    /// There is a single closest [`Point2D`].
    One(Point2D<f64, Space>),
    /// There are multiple closest [`Point2D`]s.
    Many(Vec<Point2D<f64, Space>>),
}

impl<Space> Closest<Space> {
    /// Are there infinitely many closest points?
    pub fn is_infinite(&self) -> bool {
        match self {
            Closest::Infinite => true,
            _ => false,
        }
    }

    /// Get a slice of all the closest [`Point2D`]s.
    ///
    /// # Note
    ///
    /// This will be empty if there are infinitely many closest points.
    pub fn points(&self) -> &[Point2D<f64, Space>] {
        match self {
            Closest::Infinite => &[],
            Closest::One(item) => std::slice::from_ref(item),
            Closest::Many(items) => &items,
        }
    }
}

impl<Space> FromIterator<Point2D<f64, Space>> for Closest<Space> {
    fn from_iter<I: IntoIterator<Item = Point2D<f64, Space>>>(
        iter: I,
    ) -> Closest<Space> {
        let items = Vec::from_iter(iter);

        match items.len() {
            0 => Closest::Infinite,
            1 => Closest::One(items[0]),
            _ => Closest::Many(items),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Angle;

    type Point = euclid::default::Point2D<f64>;

    #[test]
    fn on_the_line() {
        let start = Point::new(1.0, 2.0);
        let end = Point::new(3.0, 10.0);
        let line = Line::new(start, end);
        let midpoint = start + line.displacement() / 2.0;

        let got = line.closest_point(midpoint);

        assert_eq!(got, Closest::One(midpoint));
    }

    #[test]
    fn closest_point_to_zero_length_line() {
        let start = Point::new(1.0, 2.0);
        let line = Line::new(start, start);
        assert_eq!(line.length(), 0.0);
        let target = Point::new(10.0, 0.0);

        let got = line.closest_point(target);

        assert_eq!(got, Closest::One(start));
    }

    #[test]
    fn away_from_the_line() {
        let start = Point::new(0.0, 0.0);
        let end = Point::new(10.0, 0.0);
        let line = Line::new(start, end);

        let got = line.closest_point(Point::new(5.0, 5.0));

        assert_eq!(got, Closest::One(Point::new(5.0, 0.0)));
    }

    #[test]
    fn past_the_end_of_the_line() {
        let start = Point::new(0.0, 0.0);
        let end = Point::new(10.0, 0.0);
        let line = Line::new(start, end);

        let got = line.closest_point(Point::new(15.0, 5.0));

        assert_eq!(got, Closest::One(end));
    }

    #[test]
    fn before_the_start_of_the_line() {
        let start = Point::new(0.0, 0.0);
        let end = Point::new(10.0, 0.0);
        let line = Line::new(start, end);

        let got = line.closest_point(Point::new(-5.0, 5.0));

        assert_eq!(got, Closest::One(start));
    }

    #[test]
    fn centre_of_an_arc() {
        let centre = Point::zero();
        let arc =
            Arc::from_centre_radius(centre, 10.0, Angle::zero(), Angle::pi());

        let got = arc.closest_point(centre);

        assert_eq!(got, Closest::Infinite);
    }

    #[test]
    fn arc_start_point() {
        let centre = Point::zero();
        let arc =
            Arc::from_centre_radius(centre, 10.0, Angle::zero(), Angle::pi());

        let got = arc.closest_point(arc.start());

        assert_eq!(got, Closest::One(arc.start()));
    }

    #[test]
    fn arc_end_point() {
        let centre = Point::zero();
        let arc =
            Arc::from_centre_radius(centre, 10.0, Angle::zero(), Angle::pi());

        let got = arc.closest_point(arc.end());

        assert_eq!(got, Closest::One(arc.end()));
    }

    #[test]
    fn midway_between_arc_end_points() {
        let centre = Point::zero();
        let arc =
            Arc::from_centre_radius(centre, 10.0, Angle::zero(), Angle::pi());

        let got = arc.closest_point(Point::new(0.0, -10.0));

        assert_eq!(got, Closest::Many(vec![arc.start(), arc.end()]));
    }
}
