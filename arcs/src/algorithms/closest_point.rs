use crate::{
    components::{DrawingObject, Geometry},
    Arc, Line, Point, Vector,
};
use euclid::Scale;
use std::iter::FromIterator;

/// Find the location on an object which is closest to a target point.
pub trait ClosestPoint {
    /// Calculate the closest point to `target`.
    fn closest_point(&self, target: Point) -> Closest;
}

impl<'c, C: ClosestPoint + ?Sized> ClosestPoint for &'c C {
    fn closest_point(&self, target: Point) -> Closest {
        (*self).closest_point(target)
    }
}

impl ClosestPoint for Point {
    fn closest_point(&self, _target: Point) -> Closest { Closest::One(*self) }
}

impl ClosestPoint for Line {
    fn closest_point(&self, target: Point) -> Closest {
        if self.length() == 0.0 {
            return Closest::One(self.start);
        }

        let start = self.start;
        let displacement = self.displacement();

        // equation of the line: start + t * displacement, where 0 <= t <= 1

        let t = Vector::dot(target - start, displacement)
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

impl ClosestPoint for Arc {
    fn closest_point(&self, _target: Point) -> Closest { unimplemented!() }
}

impl ClosestPoint for Geometry {
    fn closest_point(&self, target: Point) -> Closest {
        match self {
            Geometry::Point(p) => p.closest_point(target),
            Geometry::Line(l) => l.closest_point(target),
            Geometry::Arc(a) => a.closest_point(target),
        }
    }
}

impl ClosestPoint for DrawingObject {
    fn closest_point(&self, target: Point) -> Closest {
        self.geometry.closest_point(target)
    }
}

/// An enum containing the different possible solutions for
/// [`ClosestPoint::closest_point()`].
#[derive(Debug, Clone, PartialEq)]
pub enum Closest {
    /// There are infinitely solutions.
    Infinite,
    /// There is a single closest [`Point`].
    One(Point),
    /// There are multiple closest [`Point`]s.
    Many(Vec<Point>),
}

impl Closest {
    /// Are there infinitely many closest points?
    pub fn is_infinite(&self) -> bool {
        match self {
            Closest::Infinite => true,
            _ => false,
        }
    }

    /// Get a slice of all the closest [`Point`]s.
    ///
    /// # Note
    ///
    /// This will be empty if there are infinitely many closest points.
    pub fn points(&self) -> &[Point] {
        match self {
            Closest::Infinite => &[],
            Closest::One(item) => std::slice::from_ref(item),
            Closest::Many(items) => &items,
        }
    }
}

impl FromIterator<Point> for Closest {
    fn from_iter<I: IntoIterator<Item = Point>>(iter: I) -> Closest {
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
}
