#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Dimension {
    Pixels(f64),
    DrawingUnits(f64),
}

impl Dimension {
    pub fn in_pixels(self, pixels_per_drawing_unit: f64) -> f64 {
        match self {
            Dimension::Pixels(px) => px,
            Dimension::DrawingUnits(units) => units * pixels_per_drawing_unit,
        }
    }
}

impl Default for Dimension {
    fn default() -> Dimension { Dimension::Pixels(1.0) }
}
