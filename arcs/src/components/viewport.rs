use crate::Vector;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Debug, Clone, PartialEq, Component)]
#[storage(HashMapStorage)]
pub struct Viewport {
    /// The location (in drawing units) this viewport is centred on.
    pub centre: Vector,
    /// The number of pixels each drawing unit should take up on the screen.
    pub pixels_per_drawing_unit: f64,
}
