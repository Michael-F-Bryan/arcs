use crate::{DrawingSpace, Line, Point};
use euclid::Transform2D;

pub trait AffineTransformable<Space = DrawingSpace> {
    fn transform(&mut self, transform: Transform2D<f64, Space, Space>);

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
