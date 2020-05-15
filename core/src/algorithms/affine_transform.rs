use crate::primitives::Line;
use euclid::default::Transform2D;

/// Something which can be transformed using an arbitrary [`Transform2D`] matrix
/// and still be semantically valid.
///
/// This is often referred to as an [Affine Transformation][wiki] in
/// mathematics.
///
/// # Examples
///
/// You can use the various methods and constructors on [`euclid::Transform2D`]
/// to build up the overall transform matrix to be applied.
///
/// ```rust
/// use arcs_core::{Angle, algorithms::AffineTransformable};
/// # type Point = euclid::default::Point2D<f64>;
/// use euclid::{Transform2D, approxeq::ApproxEq};
///
/// let point = Point::new(10.0, 10.0);
/// let transform_matrix = Transform2D::create_translation(-1.0, 1.0) // move the point
///     .post_rotate(Angle::degrees(180.0)) // then rotate 180 degrees
///     .post_scale(-1.0, 1.0); // then flip about y-axis
///
/// let got = point.transformed(transform_matrix);
///
/// // (Note: floating point operations are inherently inaccurate)
/// let expected = Point::new(9.0, -11.0);
/// assert!(got.approx_eq(&expected));
/// ```
///
/// [wiki]: https://en.wikipedia.org/wiki/Affine_transformation
pub trait AffineTransformable {
    /// Apply a transform matrix in-place.
    fn transform(&mut self, transform: Transform2D<f64>);

    /// A convenience method for getting a transformed copy of this object.
    fn transformed(&self, transform: Transform2D<f64>) -> Self
    where
        Self: Sized + Clone,
    {
        let mut clone = self.clone();
        clone.transform(transform);

        clone
    }
}

impl<'t, T: AffineTransformable + ?Sized> AffineTransformable for &'t mut T {
    fn transform(&mut self, transform: Transform2D<f64>) {
        (*self).transform(transform);
    }
}

impl<Space> AffineTransformable for euclid::Vector2D<f64, Space> {
    fn transform(&mut self, transform: Transform2D<f64>) {
        *self = transform
            .pre_transform(&euclid::Transform2D::<f64, Space, _>::identity())
            .post_transform(&euclid::Transform2D::<f64, _, Space>::identity())
            .transform_vector(*self);
    }
}

impl<Space> AffineTransformable for euclid::Point2D<f64, Space> {
    fn transform(&mut self, transform: Transform2D<f64>) {
        *self = transform
            .pre_transform(&euclid::Transform2D::<f64, Space, _>::identity())
            .post_transform(&euclid::Transform2D::<f64, _, Space>::identity())
            .transform_point(*self);
    }
}

impl<Space> AffineTransformable for Line<Space> {
    fn transform(&mut self, transform: Transform2D<f64>) {
        self.start.transform(transform);
        self.end.transform(transform);
    }
}
