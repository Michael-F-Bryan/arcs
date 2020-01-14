use cgmath::{Matrix3, Vector3};
use crate::{Vector};

pub struct Transformation {
    pub matrix: Matrix3<f64>
}

impl Transformation {
    pub const fn identity() -> Transformation {
        Transformation {
            matrix: Matrix3::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)
        }
    }

    pub fn scale(factor: f64) -> Transformation {
        let mut trans = Transformation::identity();
        trans.matrix *= factor;

        trans
    }

    fn to_homogenous(v: &Vector) -> Vector3<f64> {
        Vector3 {
            x: v.x,
            y: v.y,
            z: 1.0
        }
    }
}