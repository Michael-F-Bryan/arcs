use crate::{
    primitives::{Arc, Line},
    BoundingBox,
};
use euclid::{Angle, Point2D};

/// Calculate an axis-aligned bounding box around the item.
pub trait Bounded<S> {
    /// Calculate the approximate location this object is located in.
    fn bounding_box(&self) -> BoundingBox<S>;
}

impl<'a, S, B: Bounded<S> + ?Sized> Bounded<S> for &'a B {
    fn bounding_box(&self) -> BoundingBox<S> { (*self).bounding_box() }
}

impl<S> Bounded<S> for BoundingBox<S> {
    fn bounding_box(&self) -> BoundingBox<S> { *self }
}

impl<S> Bounded<S> for Point2D<f64, S> {
    fn bounding_box(&self) -> BoundingBox<S> { BoundingBox::new(*self, *self) }
}

impl<S> Bounded<S> for Line<S> {
    fn bounding_box(&self) -> BoundingBox<S> {
        BoundingBox::new(self.start, self.end)
    }
}

impl<S> Bounded<S> for Arc<S> {
    fn bounding_box(&self) -> BoundingBox<S> {
        let (x, y) = self.centre().to_tuple();
        let r = self.radius();

        let mut bounds = BoundingBox::new(self.start(), self.end());

        if self.contains_angle(Angle::zero()) {
            let right = Point2D::new(x + r, y);
            bounds = BoundingBox::new(bounds.bottom_left(), right);
        }
        if self.contains_angle(Angle::frac_pi_2()) {
            let top = Point2D::new(x, y + r);
            bounds = BoundingBox::new(bounds.bottom_left(), top);
        }
        if self.contains_angle(Angle::pi()) {
            let left = Point2D::new(x - r, y);
            bounds = BoundingBox::new(bounds.top_right(), left);
        }
        if self.contains_angle(Angle::pi() + Angle::frac_pi_2()) {
            let bottom = Point2D::new(x, y - r);
            bounds = BoundingBox::new(bounds.top_right(), bottom);
        }

        bounds
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use euclid::default::{Length, Point2D};

    #[test]
    fn bounding_box_around_line() {
        let start = Point2D::<f64>::zero();
        let end = Point2D::<f64>::new(3.0, 4.0);
        let line = Line::new(start, end);

        let bounds = line.bounding_box();

        assert_eq!(bounds.width(), Length::new(3.0));
        assert_eq!(bounds.height(), Length::new(4.0));
        assert_eq!(bounds.bottom_left(), start);
        assert_eq!(bounds.top_right(), end);
    }
}
