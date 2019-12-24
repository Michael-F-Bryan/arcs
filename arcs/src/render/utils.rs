use crate::{algorithms::BoundingBox, Vector};
use kurbo::{Affine, Point, Size};

#[derive(Debug, Clone, PartialEq)]
pub struct Viewport {
    /// The location (in drawing units) this viewport is centred on.
    pub centre: Vector,
    /// The number of pixels each drawing unit should take up on the screen.
    pub pixels_per_drawing_unit: f64,
}

pub fn to_canvas_coordinates(
    point: Vector,
    viewport: BoundingBox,
    window: Size,
) -> Point {
    let transform = Affine::FLIP_Y * Affine::translate((0.0, -window.width));
    // From the ratio:
    //
    //   point.x - bottom_left.x   X - window.bottom_left.x
    //   ----------------------- = ------------------------
    //      viewport.width()           window.width()

    let bl = viewport.bottom_left();
    let dx = point.x - bl.x;
    let dy = point.y - bl.y;

    kurbo::Point {
        x: dx * window.width / viewport.width(),
        y: window.height - dy * window.height / viewport.height(),
    }
}

pub fn to_drawing_coordinates(point: Point, viewport: &Viewport) -> Vector {
    Vector {
        x: point.x,
        y: point.y,
    }
}
