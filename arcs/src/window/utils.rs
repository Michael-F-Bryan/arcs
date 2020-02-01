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
        .expect("The transform should always be invertible")
}

pub fn transform_to_drawing_space(
    viewport: &Viewport,
    window: Size2D<f64, CanvasSpace>,
) -> Transform2D<f64, CanvasSpace, DrawingSpace> {
    // See https://gamedev.stackexchange.com/a/51435

    let drawing_units_per_pixel = viewport.pixels_per_drawing_unit.recip();

    // calculate the new basis vectors
    let x_axis_basis =
        Vector2D::<f64, DrawingSpace>::new(drawing_units_per_pixel, 0.0);
    let y_axis_basis =
        Vector2D::<f64, DrawingSpace>::new(0.0, -drawing_units_per_pixel);
    // and where our origin will now be
    let new_origin = viewport.centre
        + Vector2D::new(-window.width / 2.0, window.height / 2.0)
            * drawing_units_per_pixel;

    // The transform matrix is then:
    //   | x_basis.x  y_basis.x  origin.x |
    //   | x_basis.y  y_basis.y  origin.y |
    //   |         0          0         1 |

    Transform2D::row_major(
        x_axis_basis.x,
        x_axis_basis.y,
        y_axis_basis.x,
        y_axis_basis.y,
        new_origin.x,
        new_origin.y,
    )
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
            pixels_per_drawing_unit: 4.0,
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
}
