use crate::Vector;
use kurbo::{Affine, Point, Size, Vec2};

#[derive(Debug, Clone, PartialEq)]
pub struct Viewport {
    /// The location (in drawing units) this viewport is centred on.
    pub centre: Vector,
    /// The number of pixels each drawing unit should take up on the screen.
    pub pixels_per_drawing_unit: f64,
}

pub fn to_canvas_coordinates(
    point: Vector,
    viewport: &Viewport,
    window: Size,
) -> Point {
    transform_to_canvas_space(viewport, window)
        * kurbo::Point::new(point.x, point.y)
}

pub fn transform_to_canvas_space(viewport: &Viewport, window: Size) -> Affine {
    Affine::default()
}

pub fn transform_to_drawing_space(viewport: &Viewport, window: Size) -> Affine {
    transform_to_canvas_space(viewport, window).inverse()
}

pub fn to_drawing_coordinates(
    point: Point,
    viewport: &Viewport,
    window: Size,
) -> Vector {
    Vector::new(point.x, point.y) * transform_to_drawing_space(viewport, window)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drawing_to_canvas_space() {
        let inputs = vec![
            // viewport centre
            (Vector::new(300.0, 150.0), Point::new(400.0, 200.0)),
            // top-left
            (Vector::new(200.0, 200.0), Point::new(0.0, 0.0)),
            // bottom-left
            (Vector::new(200.0, 100.0), Point::new(0.0, 400.0)),
            // bottom-right
            (Vector::new(400.0, 100.0), Point::new(800.0, 400.0)),
            // top-right
            (Vector::new(400.0, 200.0), Point::new(800.0, 0.0)),
        ];
        let viewport = Viewport {
            centre: Vector::new(300.0, 150.0),
            pixels_per_drawing_unit: 4.0,
        };
        let window = Size::new(800.0, 400.0);

        for (drawing_space, expected) in inputs {
            let got = to_canvas_coordinates(drawing_space, &viewport, window);
            assert_eq!(got, expected);
        }
    }
}
