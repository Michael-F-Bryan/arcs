use crate::{
    primitives::{Arc, Line, Point},
    Vector,
};

/// Something which can be moved around "rigidly" in *Drawing Space*.
pub trait Translate {
    fn translate(&mut self, displacement: Vector);

    fn translated(&self, displacement: Vector) -> Self
    where
        Self: Sized + Clone,
    {
        let mut clone = self.clone();
        clone.translate(displacement);

        clone
    }
}

impl Translate for Vector {
    fn translate(&mut self, displacement: Vector) { *self += displacement; }
}

impl Translate for Point {
    fn translate(&mut self, displacement: Vector) {
        self.location += displacement;
    }
}

impl Translate for Line {
    fn translate(&mut self, displacement: Vector) {
        self.start += displacement;
        self.end += displacement;
    }
}

impl Translate for Arc {
    fn translate(&mut self, displacement: Vector) {
        *self = Arc::from_centre_radius(
            self.centre() + displacement,
            self.radius(),
            self.start_angle(),
            self.sweep_angle(),
        );
    }
}
