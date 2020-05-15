use crate::{algorithms::Bounded, DrawingSpace, Length, Point, Vector};
use euclid::{num::Zero, Size2D};

/// An axis-aligned bounding box.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BoundingBox {
    bottom_left: Point,
    top_right: Point,
}

impl BoundingBox {
    /// Create a new [`BoundingBox`] around two points.
    pub fn new(first: Point, second: Point) -> Self {
        let min_x = f64::min(first.x, second.x);
        let min_y = f64::min(first.y, second.y);
        let max_x = f64::max(first.x, second.x);
        let max_y = f64::max(first.y, second.y);

        BoundingBox::new_unchecked(
            Point::new(min_x, min_y),
            Point::new(max_x, max_y),
        )
    }

    /// Create a new [`BoundingBox`] without ensuring the bottom-left and
    /// top-right corners are actually in the bottom-left and top-right.
    pub fn new_unchecked(bottom_left: Point, top_right: Point) -> Self {
        debug_assert!(bottom_left.x <= top_right.x);
        debug_assert!(bottom_left.y <= top_right.y);

        BoundingBox {
            bottom_left,
            top_right,
        }
    }

    /// Create a [`BoundingBox`] based on it's centre and dimensions (as an
    /// [`euclid::Size2D`]).
    pub fn from_centre_and_size(
        centre: Point,
        size: Size2D<f64, DrawingSpace>,
    ) -> Self {
        BoundingBox::from_centre_and_dimensions(
            centre,
            Length::new(size.width),
            Length::new(size.height),
        )
    }

    /// Create a [`BoundingBox`] based on it's centre and dimensions.
    pub fn from_centre_and_dimensions(
        centre: Point,
        width: Length,
        height: Length,
    ) -> Self {
        debug_assert!(
            width >= Length::zero(),
            "{} should not be negative",
            width
        );
        debug_assert!(
            height >= Length::zero(),
            "{} should not be negative",
            height
        );

        let diagonal = Vector::from_lengths(width / 2.0, height / 2.0);
        let bottom_left = centre - diagonal;
        let top_right = centre + diagonal;
        BoundingBox::new_unchecked(bottom_left, top_right)
    }

    /// How wide is the [`BoundingBox`] in the X direction.
    pub fn width(self) -> Length { Length::new(self.diagonal().x) }

    /// How high is the [`BoundingBox`] in the Y direction.
    pub fn height(self) -> Length { Length::new(self.diagonal().y) }

    /// Calculate the box's area.
    pub fn area(self) -> f64 {
        let Vector { x, y, .. } = self.diagonal();
        x * y
    }

    /// A vector from the bottom-left corner to the top-right corner.
    pub fn diagonal(self) -> Vector { self.top_right - self.bottom_left }

    /// Merge two [`BoundingBox`]es.
    pub fn merge(left: BoundingBox, right: BoundingBox) -> BoundingBox {
        BoundingBox::new(left.bottom_left, right.top_right)
    }

    /// Create a [`BoundingBox`] which fully encompasses a set of [`Bounded`]
    /// items.
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

    /// The bottom-left corner.
    pub fn bottom_left(self) -> Point { self.bottom_left }

    /// The bottom-right corner.
    pub fn bottom_right(self) -> Point {
        self.bottom_left + Vector::from_lengths(self.width(), Length::zero())
    }

    /// The top-right corner.
    pub fn top_right(self) -> Point { self.top_right }

    /// The top-left corner.
    pub fn top_left(self) -> Point {
        self.bottom_left + Vector::from_lengths(Length::zero(), self.height())
    }

    /// The minimum X value.
    pub fn min_x(self) -> f64 { self.bottom_left.x }

    /// The minimum Y value.
    pub fn min_y(self) -> f64 { self.bottom_left.y }

    /// The maximum X value.
    pub fn max_x(self) -> f64 { self.top_right.x }

    /// The maximum Y value.
    pub fn max_y(self) -> f64 { self.top_right.y }

    /// Does this [`BoundingBox`] fully contain another?
    pub fn fully_contains(self, other: BoundingBox) -> bool {
        self.min_x() <= other.min_x()
            && other.max_x() <= self.max_x()
            && self.min_y() <= other.min_y()
            && other.max_y() <= self.max_y()
    }

    /// Do these two [`BoundingBox`]es overlap?
    pub fn intersects_with(&self, other: BoundingBox) -> bool {
        // FIXME: Actually implement this
        self.fully_contains(other)
    }
}

#[cfg(feature = "ecs")]
impl specs::Component for BoundingBox {
    type Storage = specs::FlaggedStorage<Self, specs::DenseVecStorage<Self>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounding_box_around_corners_gives_same_bounding_box() {
        let original = BoundingBox::new(Point::zero(), Point::new(10.0, 10.0));
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
