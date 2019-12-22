use crate::{px, Point};
use rgb::RGBA8;

/// A canvas things can be drawn on.
pub trait Canvas {
    type DrawingContext: DrawingContext;

    /// Get the viewport's dimensions as the pixels, `(width, height)`.
    fn dimensions(&self) -> (px, px);

    /// Get a drawing context that can be used to draw things on the canvas.
    fn drawing_context(&mut self) -> &mut Self::DrawingContext;
}

/// A convenience function for filling the entire canvas with a single colour.
pub fn clear<C: Canvas>(canvas: &mut C, fill_colour: RGBA8) {
    let (width, height) = canvas.dimensions();

    canvas.drawing_context().rect(
        Point::zero(),
        width,
        height,
        None,
        RGBA8::default(),
        fill_colour,
    );
}

/// Describes visual content using imperative draw commands.
///
/// # Note to Implementors
///
/// By convention the origin is taken as the top-left corner of the canvas.
pub trait DrawingContext {
    /// Draw a line between two points.
    fn line(
        &mut self,
        start: Point,
        end: Point,
        line_width: px,
        line_colour: RGBA8,
    );

    /// Draw a rectangle.
    fn rect(
        &mut self,
        bottom_left: Point,
        width: px,
        height: px,
        line_width: Option<px>,
        line_colour: RGBA8,
        fill_colour: RGBA8,
    );

    /// Draw a section of an elipse.
    ///
    /// When the ellipse a rotation of zero, the major radius is the radius
    /// from the centre in the direction of positive x. The minor radius is the
    /// radius from centre in the direction of positive y.
    fn ellipse(
        &mut self,
        centre: Point,
        minor_radius: px,
        major_radius: px,
        start_angle: f64,
        end_angle: f64,
        rotation_angle: f64,
        line_width: Option<px>,
        line_colour: RGBA8,
        fill_colour: RGBA8,
    );

    /// Draw an arc.
    fn arc(
        &mut self,
        centre: Point,
        radius: px,
        start_angle: f64,
        end_angle: f64,
        line_width: Option<px>,
        line_colour: RGBA8,
        fill_colour: RGBA8,
    ) {
        self.ellipse(
            centre,
            radius,
            radius,
            start_angle,
            end_angle,
            0.0,
            line_width,
            line_colour,
            fill_colour,
        );
    }
}
