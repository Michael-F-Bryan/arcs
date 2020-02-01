use crate::{DrawingSpace, Line, Point};
use euclid::Transform2D;

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
/// use arcs::{Point, Angle, algorithms::AffineTransformable};
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
pub trait AffineTransformable<Space = DrawingSpace> {
    /// Apply a transform matrix in-place.
    fn transform(&mut self, transform: Transform2D<f64, Space, Space>);

    /// A convenience method for getting a transformed copy of this object.
    fn transformed(&self, transform: Transform2D<f64, Space, Space>) -> Self
    where
        Self: Sized + Clone,
    {
        let mut clone = self.clone();
        clone.transform(transform);

        clone
    }
}

impl<'t, S, T: AffineTransformable<S> + ?Sized> AffineTransformable<S>
    for &'t mut T
{
    fn transform(&mut self, transform: Transform2D<f64, S, S>) {
        (*self).transform(transform);
    }
}

impl<Space> AffineTransformable<Space> for euclid::Vector2D<f64, Space> {
    fn transform(&mut self, transform: Transform2D<f64, Space, Space>) {
        *self = transform.transform_vector(*self);
    }
}

impl AffineTransformable for Point {
    fn transform(
        &mut self,
        transform: Transform2D<f64, DrawingSpace, DrawingSpace>,
    ) {
        *self = transform.transform_point(*self);
    }
}

impl AffineTransformable for Line {
    fn transform(
        &mut self,
        transform: Transform2D<f64, DrawingSpace, DrawingSpace>,
    ) {
        self.start.transform(transform);
        self.end.transform(transform);
    }
}
