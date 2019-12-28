use crate::{
    components::{BoundingBox, Geometry},
    primitives::{Arc, Line, Point},
    Vector,
};

/// Calculate an axis-aligned bounding box around the item.
pub trait Bounded {
    /// Calculate the approximate location this object is located in.
    fn bounding_box(&self) -> BoundingBox;
}

impl<'a, B: Bounded + ?Sized> Bounded for &'a B {
    fn bounding_box(&self) -> BoundingBox { (*self).bounding_box() }
}

impl Bounded for BoundingBox {
    fn bounding_box(&self) -> BoundingBox { *self }
}

impl Bounded for Point {
    fn bounding_box(&self) -> BoundingBox { self.location.bounding_box() }
}

impl Bounded for Vector {
    fn bounding_box(&self) -> BoundingBox { BoundingBox::new(*self, *self) }
}

impl Bounded for Line {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(self.start, self.end)
    }
}

impl Bounded for Geometry {
    fn bounding_box(&self) -> BoundingBox {
        match self {
            Geometry::Line(line) => line.bounding_box(),
            Geometry::Arc(arc) => arc.bounding_box(),
            Geometry::Point(point) => point.bounding_box(),
        }
    }
}

impl Bounded for Arc {
    fn bounding_box(&self) -> BoundingBox {
        use std::f64::consts::{FRAC_PI_2, PI};

        let Vector { x, y } = self.centre();
        let r = self.radius();

        let mut bounds = BoundingBox::new(self.start(), self.end());

        if self.contains_angle(0.0) {
            let right = Vector::new(x + r, y);
            bounds = BoundingBox::new(bounds.bottom_left(), right);
        }
        if self.contains_angle(FRAC_PI_2) {
            let top = Vector::new(x, y + r);
            bounds = BoundingBox::new(bounds.bottom_left(), top);
        }
        if self.contains_angle(PI) {
            let left = Vector::new(x - r, y);
            bounds = BoundingBox::new(bounds.top_right(), left);
        }
        if self.contains_angle(3.0 * FRAC_PI_2) {
            let bottom = Vector::new(x, y - r);
            bounds = BoundingBox::new(bounds.top_right(), bottom);
        }

        bounds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounding_box_around_line() {
        let start = Vector::zero();
        let end = Vector::new(3.0, 4.0);
        let line = Line::new(start, end);

        let bounds = line.bounding_box();

        assert_eq!(bounds.width(), 3.0);
        assert_eq!(bounds.height(), 4.0);
        assert_eq!(bounds.bottom_left(), start);
        assert_eq!(bounds.top_right(), end);
    }
}
