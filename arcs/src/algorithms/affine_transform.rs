use crate::{
    primitives::{Point, Line,},
    Vector,
};
use kurbo::Affine;

pub trait AffineTransformable {
    fn transform(&mut self, affine: Affine, base: Vector);

    fn transformed(&self, affine: Affine, base: Vector) -> Self
    where 
        Self: Sized + Clone,
        {
            let mut clone = self.clone();
            clone.transform(affine, base);

            clone
        }
}

impl <'t, T: AffineTransformable + ?Sized> AffineTransformable for &'t mut T {
    fn transform(&mut self, affine: Affine, base: Vector) {
        (*self).transform(affine, base);
    }
}

impl AffineTransformable for Vector {
    fn transform(&mut self, affine: Affine, base: Vector) {
        let translate_to_base = Affine::translate(base * -1.0);
        let translate_back = Affine::translate(base);
        let combined_transform = translate_back * affine * translate_to_base;
        let new_pos = combined_transform * *self;
        self.x = new_pos.x;
        self.y = new_pos.y;
    }
}

impl AffineTransformable for Point {
    fn transform(&mut self, affine: Affine, base: Vector) {
        self.location.transform(affine, base);
    }
}

impl AffineTransformable for Line {
    fn transform(&mut self, affine: Affine, base: Vector) {
        self.start.transform(affine, base);
        self.end.transform(affine, base);
    }
}

