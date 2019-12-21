use rgb::RGBA8;

/// A canvas things can be drawn on.
pub trait Canvas {
    /// The viewport's dimensions, in pixels.
    fn dimensions(&self) -> (px, px);

    /// Draw a line between two points.
    fn line(
        &mut self,
        start_x: px,
        start_y: px,
        end_x: px,
        end_y: px,
        line_width: px,
        line_colour: RGBA8,
    );

    /// Draw a rectangle.
    fn rect(
        &mut self,
        bottom_left_x: px,
        bottom_left_y: px,
        width: px,
        height: px,
        line_width: Option<px>,
        line_colour: RGBA8,
        fill_colour: RGBA8,
    );

    /// Draw an arc.
    fn arc(
        &mut self,
        centre_x: px,
        centre_y: px,
        radius: px,
        start_angle: f64,
        end_angle: f64,
        line_width: Option<px>,
        line_colour: RGBA8,
        fill_colour: RGBA8,
    );

    /// Clear the canvas.
    fn clear(&mut self, fill_colour: RGBA8) {
        let (width, height) = self.dimensions();

        self.rect(
            px::new(0),
            px::new(0),
            width,
            height,
            None,
            RGBA8::default(),
            fill_colour,
        );
    }
}

/// A dimension in pixels.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct px(pub i32);

impl px {
    pub const fn new(pixel: i32) -> Self { px(pixel) }
}
