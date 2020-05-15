use crate::{algorithms::AffineTransformable, BoundingBox, Transform};

/// Something who's dimensions can be scaled independently (the *non-uniform*
/// bit) in the x and y directions.
///
/// # Examples
///
/// ```rust
/// use arcs_core::{Line, Point, algorithms::ScaleNonUniform};
///
/// let original = Line::new(Point::zero(), Point::new(10.0, 10.0));
///
/// let scaled = original.scaled_non_uniform(2.0, -0.5);
///
/// assert_eq!(scaled.start, Point::zero());
/// assert_eq!(scaled.end, Point::new(20.0, -5.0));
/// ```
pub trait ScaleNonUniform {
    /// Scale the object in-place.
    fn scale_non_uniform(&mut self, factor_x: f64, factor_y: f64);

    /// Convenience method for getting a scaled copy of this object.
    fn scaled_non_uniform(&self, factor_x: f64, factor_y: f64) -> Self
    where
        Self: Sized + Clone,
    {
        let mut clone = self.clone();
        clone.scale_non_uniform(factor_x, factor_y);

        clone
    }
}

impl<A: AffineTransformable> ScaleNonUniform for A {
    fn scale_non_uniform(&mut self, factor_x: f64, factor_y: f64) {
        self.transform(Transform::create_scale(factor_x, factor_y));
    }
}

impl ScaleNonUniform for BoundingBox {
    fn scale_non_uniform(&mut self, factor_x: f64, factor_y: f64) {
        let bottom_left =
            self.bottom_left().scaled_non_uniform(factor_x, factor_y);
        let top_right = self.top_right().scaled_non_uniform(factor_x, factor_y);

        *self = BoundingBox::new(bottom_left, top_right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{algorithms::Translate, Line, Point, Vector};

    #[test]
    fn scale_point() {
        let x = -1.0;
        let y = 5.0;
        let original = Point::new(x, y);
        let factor_x = 2.0;
        let factor_y = 2.5;
        let should_be = Point::new(x * factor_x, y * factor_y);

        let actual = original.scaled_non_uniform(factor_x, factor_y);

        assert_eq!(actual, should_be);
    }

    #[test]
    fn scale_point_around_base() {
        let original = Point::new(-1.0, 5.0);
        let factor_x = 2.0;
        let factor_y = 2.5;
        let base = Vector::new(2.0, 0.0);

        let expected = Point::new(-4.0, 12.5);

        // We can either use explicit transformation methods:
        let mut transformed = original.translated(-base);
        transformed.scale_non_uniform(factor_x, factor_y);
        transformed.translate(base);

        assert_eq!(transformed, expected);

        // Or compose an `Affine` and pass it directly to the `transform`
        // method: keep in mind that transforms get composed *in reverse
        // execution order*
        let combined_transform =
            Transform::create_translation(-base.x, -base.y)
                .post_scale(factor_x, factor_y)
                .post_translate(base);

        let transformed = original.transformed(combined_transform);

        assert_eq!(transformed, expected);
    }

    #[test]
    fn scale_line() {
        let start = Point::new(2.0, 4.0);
        let end = Point::new(3.0, -5.0);
        let original = Line::new(start, end);
        let factor_x = 1.5;
        let factor_y = -2.0;

        let actual = original.scaled_non_uniform(factor_x, factor_y);
        let expected = Line::new(
            Point::new(2.0 * factor_x, 4.0 * factor_y),
            Point::new(3.0 * factor_x, -5.0 * factor_y),
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn scale_line_around_base() {
        let start = Point::new(2.0, 4.0);
        let end = Point::new(3.0, -5.0);
        let original = Line::new(start, end);
        let factor_x = 1.5;
        let factor_y = -2.0;

        let expected = Line::new(Point::new(1.75, -9.5), Point::new(3.25, 8.5));

        // scale by line mid-point as reference
        let mid_point = (start + original.displacement() * 0.5).to_vector();
        let mut transformed = original.translated(-mid_point);
        transformed.scale_non_uniform(factor_x, factor_y);
        transformed.translate(mid_point);

        assert_eq!(transformed, expected);
    }
}
