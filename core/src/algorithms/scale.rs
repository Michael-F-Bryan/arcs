use crate::{algorithms::ScaleNonUniform, Arc};

/// Something who's dimensions can be scaled uniformly.
pub trait Scale {
    /// Scale the object in-place.
    fn scale(&mut self, scale_factor: f64);

    /// Convenience method for getting a scaled copy of this object.
    fn scaled(&self, scale_factor: f64) -> Self
    where
        Self: Sized + Clone,
    {
        let mut clone = self.clone();
        clone.scale(scale_factor);

        clone
    }
}

impl<S: ScaleNonUniform> Scale for S {
    fn scale(&mut self, scale_factor: f64) {
        self.scale_non_uniform(scale_factor, scale_factor);
    }
}

impl Scale for Arc {
    fn scale(&mut self, scale_factor: f64) {
        *self = Arc::from_centre_radius(
            self.centre().scaled(scale_factor),
            self.radius() * scale_factor,
            self.start_angle(),
            self.sweep_angle(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        algorithms::{AffineTransformable, Translate},
        Arc, BoundingBox, Line, Point, Transform, Vector,
    };
    use euclid::Angle;

    #[test]
    fn scale_vector() {
        let original = Vector::new(1.0, 1.0);
        let scale_factor = 2.0;

        let actual = original.scaled(scale_factor);
        let expected = Vector::new(2.0, 2.0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn scale_line() {
        let start = Point::new(2.0, 4.0);
        let end = Point::new(3.0, -5.0);
        let original = Line::new(start, end);
        let scale_factor = 1.5;

        let actual = original.scaled(scale_factor);
        let expected = Line::new(Point::new(3.0, 6.0), Point::new(4.5, -7.5));

        assert_eq!(actual, expected);
    }

    #[test]
    fn scale_line_around_mid_point() {
        let start = Point::new(2.0, 4.0);
        let end = Point::new(3.0, -5.0);
        let original = Line::new(start, end);
        let scale_factor = 1.5;
        let mid_point = (start + original.displacement() * 0.5).to_vector();
        let expected =
            Line::new(Point::new(1.75, 6.25), Point::new(3.25, -7.25));

        // we can either use explicit transformation methods:
        let mut transformed = original.translated(Vector::zero() - mid_point);
        transformed.scale(scale_factor);
        transformed.translate(mid_point);

        assert_eq!(transformed, expected);

        // Or compose an `Affine` and pass it directly to the `transform`
        // method: keep in mind that transforms get composed *in reverse
        // execution order*
        let combined_transform =
            Transform::create_translation(-mid_point.x, -mid_point.y)
                .post_scale(scale_factor, scale_factor)
                .post_translate(mid_point);

        let transformed = original.transformed(combined_transform);

        assert_eq!(transformed, expected);
    }

    #[test]
    fn scale_arc() {
        let x = -1.4;
        let y = 2.0;
        let centre = Point::new(x, y);
        let radius = 5.0;
        let start_angle = Angle::radians(0.5);
        let sweep_angle = Angle::radians(1.0);
        let original =
            Arc::from_centre_radius(centre, radius, start_angle, sweep_angle);
        let scale_factor = 2.0;

        let actual = original.scaled(scale_factor);
        let expected = Arc::from_centre_radius(
            Point::new(x * scale_factor, y * scale_factor),
            radius * scale_factor,
            start_angle,
            sweep_angle,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn scale_arc_around_centre() {
        let x = -1.4;
        let y = 2.0;
        let centre = Point::new(x, y);
        let radius = 5.0;
        let start_angle = Angle::radians(0.5);
        let sweep_angle = Angle::radians(1.0);
        let original =
            Arc::from_centre_radius(centre, radius, start_angle, sweep_angle);
        let scale_factor = 2.0;

        let expected = Arc::from_centre_radius(
            centre,
            radius * scale_factor,
            start_angle,
            sweep_angle,
        );

        let mut transformed =
            original.translated(Vector::zero() - centre.to_vector());
        transformed.scale(scale_factor);
        transformed.translate(centre.to_vector());

        assert_eq!(transformed, expected);
    }

    #[test]
    fn scale_bounding_box() {
        let first = Point::new(-2.0, 1.5);
        let second = Point::new(4.0, 3.5);
        let scale_factor = 1.5;
        let original = BoundingBox::new(first, second);

        let expected =
            BoundingBox::new(Point::new(-3.0, 2.25), Point::new(6.0, 5.25));
        let actual = original.scaled(scale_factor);

        assert_eq!(actual, expected);
    }
}
