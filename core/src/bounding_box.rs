use crate::algorithms::Bounded;
use euclid::{num::Zero, Length, Point2D, Size2D, Vector2D};

/// An axis-aligned bounding box.
#[derive(Debug, PartialEq)]
pub struct BoundingBox<S> {
    bottom_left: Point2D<f64, S>,
    top_right: Point2D<f64, S>,
}

impl<S> BoundingBox<S> {
    /// Create a new [`BoundingBox`] around two points.
    pub fn new(first: Point2D<f64, S>, second: Point2D<f64, S>) -> Self {
        let min_x = f64::min(first.x, second.x);
        let min_y = f64::min(first.y, second.y);
        let max_x = f64::max(first.x, second.x);
        let max_y = f64::max(first.y, second.y);

        BoundingBox::new_unchecked(
            Point2D::new(min_x, min_y),
            Point2D::new(max_x, max_y),
        )
    }

    /// Create a new [`BoundingBox`] without ensuring the bottom-left and
    /// top-right corners are actually in the bottom-left and top-right.
    pub fn new_unchecked(
        bottom_left: Point2D<f64, S>,
        top_right: Point2D<f64, S>,
    ) -> Self {
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
        centre: Point2D<f64, S>,
        size: Size2D<f64, S>,
    ) -> Self {
        BoundingBox::from_centre_and_dimensions(
            centre,
            Length::new(size.width),
            Length::new(size.height),
        )
    }

    /// Create a [`BoundingBox`] based on it's centre and dimensions.
    pub fn from_centre_and_dimensions(
        centre: Point2D<f64, S>,
        width: Length<f64, S>,
        height: Length<f64, S>,
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

        let diagonal = Vector2D::from_lengths(width / 2.0, height / 2.0);
        let bottom_left = centre - diagonal;
        let top_right = centre + diagonal;
        BoundingBox::new_unchecked(bottom_left, top_right)
    }

    /// How wide is the [`BoundingBox`] in the X direction.
    pub fn width(self) -> Length<f64, S> { Length::new(self.diagonal().x) }

    /// How high is the [`BoundingBox`] in the Y direction.
    pub fn height(self) -> Length<f64, S> { Length::new(self.diagonal().y) }

    /// Calculate the box's area.
    pub fn area(self) -> f64 {
        let Vector2D { x, y, .. } = self.diagonal();
        x * y
    }

    /// A vector from the bottom-left corner to the top-right corner.
    pub fn diagonal(self) -> Vector2D<f64, S> {
        self.top_right - self.bottom_left
    }

    /// Merge two [`BoundingBox`]es.
    pub fn merge(
        left: BoundingBox<S>,
        right: BoundingBox<S>,
    ) -> BoundingBox<S> {
        BoundingBox::new(left.bottom_left, right.top_right)
    }

    /// Create a [`BoundingBox`] which fully encompasses a set of [`Bounded`]
    /// items.
    pub fn around<I, B>(items: I) -> Option<BoundingBox<S>>
    where
        I: IntoIterator<Item = B>,
        B: Bounded<S>,
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
    pub fn bottom_left(self) -> Point2D<f64, S> { self.bottom_left }

    /// The bottom-right corner.
    pub fn bottom_right(self) -> Point2D<f64, S> {
        self.bottom_left + Vector2D::from_lengths(self.width(), Length::zero())
    }

    /// The top-right corner.
    pub fn top_right(self) -> Point2D<f64, S> { self.top_right }

    /// The top-left corner.
    pub fn top_left(self) -> Point2D<f64, S> {
        self.bottom_left + Vector2D::from_lengths(Length::zero(), self.height())
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
    pub fn fully_contains(self, other: BoundingBox<S>) -> bool {
        self.min_x() <= other.min_x()
            && other.max_x() <= self.max_x()
            && self.min_y() <= other.min_y()
            && other.max_y() <= self.max_y()
    }

    /// Do these two [`BoundingBox`]es overlap?
    pub fn intersects_with(&self, other: BoundingBox<S>) -> bool {
        // FIXME: Actually implement this
        self.fully_contains(other)
    }
}

impl<Space> Copy for BoundingBox<Space> {}
impl<Space> Clone for BoundingBox<Space> {
    fn clone(&self) -> Self { *self }
}

#[cfg(feature = "ecs")]
impl<S: 'static> specs::Component for BoundingBox<S> {
    type Storage = specs::FlaggedStorage<Self, specs::DenseVecStorage<Self>>;
}

// The builtin impl for euclid::Point2D saw a type parameter and because it's
// conservative, it only automatically implemented Send + Sync for S: Send +
// Sync.
//
// A bounding box is just a couple numbers, so this is perfectly safe.
unsafe impl<S> Send for BoundingBox<S> {}
unsafe impl<S> Sync for BoundingBox<S> {}

#[cfg(test)]
mod tests {
    use super::*;
    use euclid::default::Point2D;

    #[test]
    fn bounding_box_around_corners_gives_same_bounding_box() {
        let original =
            BoundingBox::new(Point2D::zero(), Point2D::new(10.0, 10.0));
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
