use crate::{CanvasSpace, DrawingSpace, Length};
use euclid::Scale;

/// A dimension on the canvas.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Dimension {
    /// The dimension should always be the same size in pixels, regardless of
    /// the zoom level.
    Pixels(f64),
    /// A "real" dimension defined in *Drawing Space*, which should be scaled
    /// appropriately when we zoom.
    DrawingUnits(Length),
}

impl Dimension {
    pub fn in_pixels(
        self,
        pixels_per_drawing_unit: Scale<f64, DrawingSpace, CanvasSpace>,
    ) -> f64 {
        match self {
            Dimension::Pixels(px) => px,
            Dimension::DrawingUnits(length) => {
                length.get() * pixels_per_drawing_unit.get()
            },
        }
    }
}

impl Default for Dimension {
    fn default() -> Dimension { Dimension::Pixels(1.0) }
}
