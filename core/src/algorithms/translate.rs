use crate::{algorithms::AffineTransformable, primitives::Arc, BoundingBox};
use euclid::{Transform2D, Vector2D};

/// Something which can be moved around "rigidly" in *Drawing Space*.
pub trait Translate<Space> {
    /// Translate this object in-place.
    fn translate(&mut self, displacement: Vector2D<f64, Space>);

    /// A convenience method for getting a translated copy of this object.
    fn translated(&self, displacement: Vector2D<f64, Space>) -> Self
    where
        Self: Sized + Clone,
    {
        let mut clone = self.clone();
        clone.translate(displacement);

        clone
    }
}

impl<Space, A: AffineTransformable> Translate<Space> for A {
    fn translate(&mut self, displacement: Vector2D<f64, Space>) {
        self.transform(Transform2D::create_translation(
            displacement.x,
            displacement.y,
        ));
    }
}

impl<Space> Translate<Space> for Arc<Space> {
    fn translate(&mut self, displacement: Vector2D<f64, Space>) {
        *self = Arc::from_centre_radius(
            self.centre().translated(displacement),
            self.radius(),
            self.start_angle(),
            self.sweep_angle(),
        );
    }
}

impl<Space> Translate<Space> for BoundingBox<Space> {
    fn translate(&mut self, displacement: Vector2D<f64, Space>) {
        *self = BoundingBox::new_unchecked(
            self.bottom_left().translated(displacement),
            self.top_right().translated(displacement),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Point = euclid::default::Point2D<f64>;
    type Vector = euclid::default::Vector2D<f64>;

    #[test]
    fn translate_point() {
        let original = Point::new(3.0, 4.0);
        let delta = Vector::new(-5.0, 2.5);

        let got = original.translated(delta);

        assert_eq!(got, original + delta);
    }

    #[test]
    fn translate_bounding_box() {
        let first = Point::new(-2.0, 1.5);
        let second = Point::new(4.0, 3.7);
        let displacement = Vector::new(1.0, -1.0);
        let original = BoundingBox::new(first, second);

        let expected = BoundingBox::new(
            Point::new(-2.0 + 1.0, 1.5 + -1.0),
            Point::new(4.0 + 1.0, 3.7 + -1.0),
        );
        let actual = original.translated(displacement);

        assert_eq!(actual, expected);
    }
}
