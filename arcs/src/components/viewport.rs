use crate::{algorithms::Translate, CanvasSpace, DrawingSpace, Point, Vector};
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

impl crate::algorithms::Scale for Viewport {
    /// Zoom the viewport, where a positive `scale_factor` will zoom in.
    fn scale(&mut self, scale_factor: f64) {
        assert!(scale_factor.is_finite() && scale_factor != 0.0);
        self.pixels_per_drawing_unit = euclid::Scale::new(
            self.pixels_per_drawing_unit.get() / scale_factor,
        );
    }
}

impl Translate<DrawingSpace> for Viewport {
    fn translate(&mut self, displacement: Vector) {
        self.centre.translate(displacement);
    }
}
