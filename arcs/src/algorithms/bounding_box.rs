use crate::{
    components::{BoundingBox, Geometry},
    Angle, Arc, Line, Point,
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

        let (x, y) = self.centre().to_tuple();
        let r = self.radius();

        let mut bounds = BoundingBox::new(self.start(), self.end());

        if self.contains_angle(Angle::zero()) {
            let right = Point::new(x + r, y);
            bounds = BoundingBox::new(bounds.bottom_left(), right);
        }
        if self.contains_angle(Angle::frac_pi_2()) {
            let top = Point::new(x, y + r);
            bounds = BoundingBox::new(bounds.bottom_left(), top);
        }
        if self.contains_angle(Angle::pi()) {
            let left = Point::new(x - r, y);
            bounds = BoundingBox::new(bounds.top_right(), left);
        }
        if self.contains_angle(Angle::pi() + Angle::frac_pi_2()) {
            let bottom = Point::new(x, y - r);
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
        let start = Point::zero();
        let end = Point::new(3.0, 4.0);
        let line = Line::new(start, end);

        let bounds = line.bounding_box();

        assert_eq!(bounds.width(), Length::new(3.0));
        assert_eq!(bounds.height(), Length::new(4.0));
        assert_eq!(bounds.bottom_left(), start);
        assert_eq!(bounds.top_right(), end);
    }
}
