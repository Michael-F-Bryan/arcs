use crate::{components::Viewport, CanvasSpace, DrawingSpace};
use euclid::{Point2D, Size2D, Transform2D, Vector2D};

pub fn to_canvas_coordinates(
    point: Point2D<f64, DrawingSpace>,
    viewport: &Viewport,
    window: Size2D<f64, CanvasSpace>,
) -> Point2D<f64, CanvasSpace> {
    transform_to_canvas_space(viewport, window).transform_point(point)
}

pub fn transform_to_canvas_space(
    viewport: &Viewport,
    window: Size2D<f64, CanvasSpace>,
) -> Transform2D<f64, DrawingSpace, CanvasSpace> {
    transform_to_drawing_space(viewport, window)
        .inverse()
        .expect("The transform matrix should always be invertible")
}

pub fn transform_to_drawing_space(
    viewport: &Viewport,
    window: Size2D<f64, CanvasSpace>,
) -> Transform2D<f64, CanvasSpace, DrawingSpace> {
    // See https://gamedev.stackexchange.com/a/51435

    let drawing_units_per_pixel = viewport.pixels_per_drawing_unit.inverse();

    // calculate the new basis vectors
    let x_axis = Vector2D::new(1.0, 0.0);
    let x_axis_basis = drawing_units_per_pixel.transform_vector(x_axis);
    let y_axis = Vector2D::new(0.0, -1.0);
    let y_axis_basis = drawing_units_per_pixel.transform_vector(y_axis);
    // and where our origin will now be
    let new_origin = Vector2D::new(viewport.centre.x, viewport.centre.y)
        + Vector2D::new(-window.width / 2.0, window.height / 2.0)
            * drawing_units_per_pixel;

    // This gives us a column-order matrix (x * T => x'):
    //   | x_basis.x  x_basis.y  0 |
    //   | y_basis.x  y_basis.y  0 |
    //   | origin.x   origin.y   1 |

    Transform2D::from_arrays([
        x_axis_basis.to_array(),
        y_axis_basis.to_array(),
        new_origin.to_array(),
    ])
}

pub fn to_drawing_coordinates(
    point: Point2D<f64, CanvasSpace>,
    viewport: &Viewport,
    window: Size2D<f64, CanvasSpace>,
) -> Point2D<f64, DrawingSpace> {
    transform_to_drawing_space(viewport, window).transform_point(point)
}

#[cfg(test)]
mod tests {
    use super::*;
    use euclid::Scale;

    /// These are the numbers from an example I drew out on paper and calculated
    /// by hand.
    fn known_example() -> (
        Vec<(Point2D<f64, DrawingSpace>, Point2D<f64, CanvasSpace>)>,
        Viewport,
        Size2D<f64, CanvasSpace>,
    ) {
        let vertices = vec![
            // viewport centre
            (Point2D::new(300.0, 150.0), Point2D::new(400.0, 200.0)),
            // top-left
            (Point2D::new(200.0, 200.0), Point2D::new(0.0, 0.0)),
            // bottom-left
            (Point2D::new(200.0, 100.0), Point2D::new(0.0, 400.0)),
            // bottom-right
            (Point2D::new(400.0, 100.0), Point2D::new(800.0, 400.0)),
            // top-right
            (Point2D::new(400.0, 200.0), Point2D::new(800.0, 0.0)),
        ];
        let viewport = Viewport {
            centre: Point2D::new(300.0, 150.0),
            pixels_per_drawing_unit: Scale::new(4.0),
        };
        let window = Size2D::new(800.0, 400.0);

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

    #[test]
    fn known_transform_matrix() {
        // We already know the transform matrix for this example from our use
        // of kurbo::Affine,
        //   canvas -> drawing: Affine([0.25, 0.0, 0.0, -0.25, 200.0, 200.0])
        //   drawing -> canvas: Affine([4.0, 0.0, 0.0, -4.0, -800.0, 800.0])
        let (_, viewport, window) = known_example();

        assert_eq!(
            transform_to_drawing_space(&viewport, window).to_array(),
            [0.25, 0.0, 0.0, -0.25, 200.0, 200.0]
        );
        assert_eq!(
            transform_to_canvas_space(&viewport, window).to_array(),
            [4.0, 0.0, 0.0, -4.0, -800.0, 800.0]
        );
    }
}
