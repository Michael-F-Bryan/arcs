use crate::{
    primitives::{Arc},
    Vector,
    algorithms::{AffineTransformable},
};
use kurbo::Affine;

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

impl<A: AffineTransformable> Translate for A {
    fn translate(&mut self, displacement: Vector) {
        self.transform(Affine::translate(displacement));
    }
}

impl Translate for Arc {
    fn translate(&mut self, displacement: Vector) {
        *self = Arc::from_centre_radius(
            self.centre().translated(displacement),
            self.radius(),
            self.start_angle(),
            self.sweep_angle(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_vector() {
        let original = Vector::new(3.0, 4.0);
        let delta = Vector::new(-5.0, 2.5);

        let got = original.translated(delta);

        assert_eq!(got, original + delta);
    }
}
