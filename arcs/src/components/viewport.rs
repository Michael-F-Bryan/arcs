use crate::{CanvasSpace, DrawingSpace, Point};
use euclid::Scale;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Debug, Clone, PartialEq, Component)]
#[storage(HashMapStorage)]
pub struct Viewport {
    /// The location (in drawing units) this viewport is centred on.
    pub centre: Point,
    /// The number of pixels each drawing unit should take up on the screen.
    pub pixels_per_drawing_unit: Scale<f64, DrawingSpace, CanvasSpace>,
}
