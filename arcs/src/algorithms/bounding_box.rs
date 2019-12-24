use crate::{
    components::Geometry,
    primitives::{Arc, Line, Point},
    Vector,
};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Debug, Copy, Clone, PartialEq, Component)]
#[storage(DenseVecStorage)]
pub struct BoundingBox {
    bottom_left: Vector,
    top_right: Vector,
}

impl BoundingBox {
    /// Create a new [`BoundingBox`] around two points.
    pub fn new(first: Vector, second: Vector) -> Self {
        let min_x = f64::min(first.x, second.x);
        let min_y = f64::min(first.y, second.y);
        let max_x = f64::max(first.x, second.x);
        let max_y = f64::max(first.y, second.y);

        BoundingBox::new_unchecked(
            Vector::new(min_x, min_y),
            Vector::new(max_x, max_y),
        )
    }

    /// Create a new [`BoundingBox`] without ensuring the bottom-left and
    /// top-right corners are actually in the bottom-left and top-right.
    pub fn new_unchecked(bottom_left: Vector, top_right: Vector) -> Self {
        debug_assert!(bottom_left.x <= top_right.x);
        debug_assert!(bottom_left.y <= top_right.y);

        BoundingBox {
            bottom_left,
            top_right,
        }
    }

    pub fn from_centre_and_dimensions(
        centre: Vector,
        width: f64,
        height: f64,
    ) -> Self {
        debug_assert!(width >= 0.0);
        debug_assert!(height >= 0.0);

        let diagonal = Vector::new(width / 2.0, height / 2.0);
        let bottom_left = centre - diagonal;
        let top_right = centre + diagonal;
        BoundingBox::new_unchecked(bottom_left, top_right)
    }

    pub fn width(self) -> f64 { self.top_right.x - self.bottom_left.x }

    pub fn height(self) -> f64 { self.top_right.y - self.bottom_left.y }

    pub fn area(self) -> f64 { self.width() * self.height() }

    pub fn diagonal(self) -> Vector { self.top_right - self.bottom_left }

    /// Merge two [`BoundingBox`]es.
    pub fn merge(left: BoundingBox, right: BoundingBox) -> BoundingBox {
        BoundingBox::new(left.bottom_left, right.top_right)
    }

    pub fn around<I, B>(items: I) -> Option<BoundingBox>
    where
        I: IntoIterator<Item = B>,
        B: Bounded,
    {
        items
            .into_iter()
            .map(|b| b.bounding_box())
            .fold(None, |acc, item| match acc {
                Some(acc) => Some(BoundingBox::merge(acc, item)),
                None => Some(item),
            })
    }

    pub fn bottom_left(&self) -> Vector { self.bottom_left }

    pub fn bottom_right(&self) -> Vector {
        self.bottom_left + Vector::new(self.width(), 0.0)
    }

    pub fn top_right(&self) -> Vector { self.top_right }

    pub fn top_left(&self) -> Vector {
        self.bottom_left + Vector::new(0.0, self.height())
    }

    pub fn intersects_with(&self, other: BoundingBox) -> bool {
        unimplemented!()
    }
}

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
            bounds = BoundingBox::new(bounds.bottom_left, right);
        }
        if self.contains_angle(FRAC_PI_2) {
            let top = Vector::new(x, y + r);
            bounds = BoundingBox::new(bounds.bottom_left, top);
        }
        if self.contains_angle(PI) {
            let left = Vector::new(x - r, y);
            bounds = BoundingBox::new(bounds.top_right, left);
        }
        if self.contains_angle(3.0 * FRAC_PI_2) {
            let bottom = Vector::new(x, y - r);
            bounds = BoundingBox::new(bounds.top_right, bottom);
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

    #[test]
    fn bounding_box_around_corners_gives_same_bounding_box() {
        let original =
            BoundingBox::new(Vector::zero(), Vector::new(10.0, 10.0));
        let corners = vec![
            original.bottom_left(),
            original.bottom_right(),
            original.top_left(),
            original.top_right(),
        ];

        let got = BoundingBox::around(corners).unwrap();

        assert_eq!(got, original);
    }
}
