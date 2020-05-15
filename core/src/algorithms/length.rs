use crate::primitives::{Arc, Line};
use euclid::Vector2D;

/// Something which has a finite length.
pub trait Length {
    /// Calculate the length.
    fn length(&self) -> f64;
}

impl<'a, L: Length + ?Sized> Length for &'a L {
    fn length(&self) -> f64 { (*self).length() }
}

impl<Space> Length for Line<Space> {
    /// Calculates the length of the line.
    ///
    /// ```rust
    /// # use arcs_core::{algorithms::Length, Line, Point};
    /// let line = Line::new(Point::zero(), Point::new(5.0, 0.0));
    ///
    /// assert_eq!(line.length(), 5.0);
    /// ```
    fn length(&self) -> f64 { self.displacement().length() }
}

impl<Space> Length for Vector2D<f64, Space> {
    /// Calculates the [`Vector`]'s magnitude.
    ///
    /// ```rust
    /// # use arcs_core::{algorithms::Length, Vector};
    /// let vector = Vector::new(3.0, 4.0);
    ///
    /// assert_eq!(vector.length(), 5.0);
    /// ```
    fn length(&self) -> f64 { euclid::Vector2D::length(self) }
}

impl<Space> Length for Arc<Space> {
    /// Calculates the length of an [`Arc`].
    ///
    /// ```rust
    /// # use arcs_core::{algorithms::Length, Arc, Point, Angle};
    /// # use std::f64::consts::PI;
    /// let radius = 50.0;
    /// let arc = Arc::from_centre_radius(
    ///     Point::zero(),
    ///     radius,
    ///     Angle::zero(),
    ///     Angle::two_pi(),
    /// );
    ///
    /// assert_eq!(arc.length(), 2.0 * radius * PI);
    /// ```
    fn length(&self) -> f64 { self.radius() * self.sweep_angle().radians.abs() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Angle;

    type Point = euclid::default::Point2D<f64>;

    #[test]
    fn line() {
        let thing = Line::new(Point::zero(), Point::new(3.0, 4.0));

        assert_eq!(thing.length(), 5.0);
    }

    #[test]
    fn arc() {
        let arc = Arc::from_centre_radius(
            Point::zero(),
            10.0,
            Angle::zero(),
            Angle::pi(),
        );

        assert_eq!(
            arc.length(),
            arc.sweep_angle().radians.abs() * arc.radius()
        );
    }
}
