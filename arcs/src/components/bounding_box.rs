use crate::{algorithms::Bounded, Vector};
use specs::prelude::*;
use specs_derive::Component;
use aabb_quadtree::{Spatial};
use quadtree_euclid::{TypedRect, TypedPoint2D, TypedSize2D};

/// An axis-aligned bounding box.
#[derive(Debug, Copy, Clone, PartialEq, Component)]
#[storage(DenseVecStorage)]
pub struct BoundingBox {
    bottom_left: Vector,
    top_right: Vector,
}

impl Spatial<f64> for BoundingBox {
    fn aabb(&self) -> TypedRect<f32, f64> {
        let bb = self;
        TypedRect::<f32, f64>::new(
            // TypedRects have their origin at the bottom left corner (this is undocumented!)
            TypedPoint2D::new(bb.bottom_left().x as f32, bb.bottom_left().y as f32),
            TypedSize2D::new(bb.width() as f32, bb.height() as f32))
    }
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

    pub fn center(self) -> Vector { self.bottom_left() + self.diagonal() * 0.5 }

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

    pub fn bottom_left(self) -> Vector { self.bottom_left }

    pub fn bottom_right(self) -> Vector {
        self.bottom_left + Vector::new(self.width(), 0.0)
    }

    pub fn top_right(self) -> Vector { self.top_right }

    pub fn top_left(self) -> Vector {
        self.bottom_left + Vector::new(0.0, self.height())
    }

    pub fn min_x(self) -> f64 { self.bottom_left.x }

    pub fn min_y(self) -> f64 { self.bottom_left.y }

    pub fn max_x(self) -> f64 { self.top_right.x }

    pub fn max_y(self) -> f64 { self.top_right.y }

    pub fn fully_contains(self, other: BoundingBox) -> bool {
        self.min_x() <= other.min_x()
            && other.max_x() <= self.max_x()
            && self.min_y() <= other.min_y()
            && other.max_y() <= self.max_y()
    }

    pub fn intersects_with(&self, other: BoundingBox) -> bool {
        // FIXME: Actually implement this
        self.fully_contains(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
