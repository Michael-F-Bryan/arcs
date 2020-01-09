use crate::{components::Viewport, Vector};
use kurbo::{Affine, Point, Size};

pub fn to_canvas_coordinates(
    point: Vector,
    viewport: &Viewport,
    window: Size,
) -> Point {
    transform_to_canvas_space(viewport, window)
        * kurbo::Point::new(point.x, point.y)
}

pub fn transform_to_canvas_space(viewport: &Viewport, window: Size) -> Affine {
    transform_to_drawing_space(viewport, window).inverse()
}

pub fn transform_to_drawing_space(viewport: &Viewport, window: Size) -> Affine {
    // See https://gamedev.stackexchange.com/a/51435

    let drawing_units_per_pixel = viewport.pixels_per_drawing_unit.recip();

    // calculate the new basis vectors
    let x_axis_basis = Vector::new(drawing_units_per_pixel, 0.0);
    let y_axis_basis = Vector::new(0.0, -drawing_units_per_pixel);
    // and where our origin will now be
    let new_origin = viewport.centre
        + Vector::new(-window.width / 2.0, window.height / 2.0)
            * drawing_units_per_pixel;

    // The transform matrix is then:
    //   | x_basis.x  y_basis.x  origin.x |
    //   | x_basis.y  y_basis.y  origin.y |
    //   |         0          0         1 |

    Affine::new([
        x_axis_basis.x,
        x_axis_basis.y,
        y_axis_basis.x,
        y_axis_basis.y,
        new_origin.x,
        new_origin.y,
    ])
}

pub fn to_drawing_coordinates(
    point: Point,
    viewport: &Viewport,
    window: Size,
) -> Vector {
    transform_to_drawing_space(viewport, window) * Vector::new(point.x, point.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn known_example() -> (Vec<(Vector, Point)>, Viewport, Size) {
        let vertices = vec![
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

        (vertices, viewport, window)
    }

    #[test]
    fn drawing_to_canvas_space() {
        let (inputs, viewport, window) = known_example();

        for (drawing_space, expected) in inputs {
            let got = to_canvas_coordinates(drawing_space, &viewport, window);
            assert_eq!(got, expected);
        }
    }

    #[test]
    fn canvas_to_drawing_space() {
        let (inputs, viewport, window) = known_example();

        for (expected, canvas_space) in inputs {
            let got = to_drawing_coordinates(canvas_space, &viewport, window);
            assert_eq!(got, expected);
        }
    }
}
