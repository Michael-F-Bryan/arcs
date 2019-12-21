use crate::{
    primitives::{Arc, Line, Point},
    Vector,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BoundingBox {
    pub bottom_left: Vector,
    pub top_right: Vector,
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
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(self.location, self.location)
    }
}

impl Bounded for Line {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(self.start, self.end)
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
