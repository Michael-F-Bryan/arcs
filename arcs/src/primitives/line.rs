use euclid::{Point2D, Vector2D};

/// A line connecting [`Line::start`] to [`Line::end`].
#[derive(Debug, Default, PartialEq)]
pub struct Line<S> {
    pub start: Point2D<f64, S>,
    pub end: Point2D<f64, S>,
}

impl<S> Line<S> {
    pub const fn new(start: Point2D<f64, S>, end: Point2D<f64, S>) -> Self {
        Line { start, end }
    }

    pub fn displacement(&self) -> Vector2D<f64, S> { self.end - self.start }

    pub fn direction(&self) -> Vector2D<f64, S> {
        self.displacement().normalize()
    }

    pub fn length(self) -> f64 { self.displacement().length() }

    pub fn perpendicular_distance_to(self, point: Point2D<f64, S>) -> f64 {
        let side_a = self.start - point;
        let side_b = self.end - point;
        let area = Vector2D::cross(side_a, side_b) / 2.0;

        // area = base * height / 2
        let base_length = self.length();

        if base_length.abs() < 0.0001 {
            side_a.length()
        } else {
            area * 2.0 / base_length
        }
    }
}

impl<S> Copy for Line<S> {}

impl<S> Clone for Line<S> {
    fn clone(&self) -> Self { *self }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Point, Vector};

    #[test]
    fn calculate_length() {
        let start = Point::new(1.0, 1.0);
        let displacement = Vector::new(3.0, 4.0);
        let v = Line::new(start, start + displacement);

        assert_eq!(v.length(), 5.0);
        assert_eq!(v.displacement(), displacement);
    }
}
