use crate::{
    primitives::{Point, Line,},
    Vector,
};
use kurbo::Affine;

pub trait AffineTransformable {
    fn transform(&mut self, affine: Affine);

    fn transformed(&self, affine: Affine) -> Self
    where 
        Self: Sized + Clone,
        {
            let mut clone = self.clone();
            clone.transform(affine);

            clone
        }
}

impl <'t, T: AffineTransformable + ?Sized> AffineTransformable for &'t mut T {
    fn transform(&mut self, affine: Affine) {
        (*self).transform(affine);
    }
}

impl AffineTransformable for Vector {
    fn transform(&mut self, affine: Affine) {
        let new_pos = affine * *self;
        self.x = new_pos.x;
        self.y = new_pos.y;
    }
}

impl AffineTransformable for Point {
    fn transform(&mut self, affine: Affine) {
        self.location.transform(affine);
    }
}

impl AffineTransformable for Line {
    fn transform(&mut self, affine: Affine) {
        self.start.transform(affine);
        self.end.transform(affine);
    }
}

