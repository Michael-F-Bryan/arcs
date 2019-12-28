use crate::{
    primitives::{Arc, Line},
    Vector,
};

pub trait Length {
    fn length(&self) -> f64;
}

impl Length for Line {
    fn length(&self) -> f64 { self.displacement().length() }
}

impl Length for Vector {
    fn length(&self) -> f64 { Vector::length(*self) }
}

impl Length for Arc {
    fn length(&self) -> f64 { self.radius() * self.sweep_angle().abs() }
}
