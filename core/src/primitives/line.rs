use euclid::{Length, Point2D, Vector2D};

/// A line connecting [`Line::start`] to [`Line::end`].
#[derive(Debug, Default, PartialEq)]
pub struct Line<S> {
    /// The [`Line`]'s starting point.
    pub start: Point2D<f64, S>,
    /// The [`Line`]'s ending point.
    pub end: Point2D<f64, S>,
}

impl<S> Line<S> {
    /// Create a new [`Line`].
    pub const fn new(start: Point2D<f64, S>, end: Point2D<f64, S>) -> Self {
        Line { start, end }
    }

    /// The displacement vector from [`Line::start`] to [`Line::end`].
    pub fn displacement(&self) -> Vector2D<f64, S> { self.end - self.start }

    /// The [`Line::displacement()`], normalised to a unit vector.
    pub fn direction(&self) -> Vector2D<f64, S> {
        self.displacement().normalize()
    }

    /// The [`Line`]'s length.
    pub fn length(self) -> f64 { self.displacement().length() }

    ///  How close would the [`Point2D`] get if this line were extended
    /// forever?
    ///
    /// See also [*Distance from a point to a line*][wiki] on Wikipedia.
    ///
    /// [wiki]: https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line
    pub fn perpendicular_distance_to(
        self,
        point: Point2D<f64, S>,
    ) -> Length<f64, S> {
        const SOME_SMALL_NUMBER: f64 = std::f64::EPSILON * 100.0;

        let side_a = self.start - point;
        let side_b = self.end - point;
        let area = Vector2D::cross(side_a, side_b) / 2.0;

        // area = base * height / 2
        let base_length = self.length();

        Length::new(if base_length.abs() < SOME_SMALL_NUMBER {
            side_a.length()
        } else {
            area.abs() * 2.0 / base_length
        })
    }
}

impl<S> Copy for Line<S> {}

impl<S> Clone for Line<S> {
    fn clone(&self) -> Self { *self }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Point = euclid::default::Point2D<f64>;
    type Vector = euclid::default::Vector2D<f64>;

    #[test]
    fn calculate_length() {
        let start = Point::new(1.0, 1.0);
        let displacement = Vector::new(3.0, 4.0);
        let v = Line::new(start, start + displacement);

        assert_eq!(v.length(), 5.0);
        assert_eq!(v.displacement(), displacement);
    }
}
