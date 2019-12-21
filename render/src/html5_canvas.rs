use web_sys::CanvasRenderingContext2d;
use crate::{Canvas, px};
use rgb::RGBA8;

#[derive(Debug, Clone)]
pub struct Html5Canvas {
    ctx: CanvasRenderingContext2d,
}

impl Html5Canvas {
    pub fn new(ctx: CanvasRenderingContext2d) -> Self { Html5Canvas { ctx } }
}

impl Canvas for Html5Canvas {
    fn dimensions(&self) -> (px, px) { unimplemented!()}

    fn line(
        &mut self,
        _start_x: px,
        _start_y: px,
        _end_x: px,
        _end_y: px,
        _line_width: px,
        _line_colour: RGBA8,
    ) { unimplemented!()}

    fn rect(
        &mut self,
        _bottom_left_x: px,
        _bottom_left_y: px,
        _width: px,
        _height: px,
        _line_width: Option<px>,
        _line_colour: RGBA8,
        _fill_colour: RGBA8,
    ) { unimplemented!()}

    fn arc(
        &mut self,
        _centre_x: px,
        _centre_y: px,
        _radius: px,
        _start_angle: f64,
        _end_angle: f64,
        _line_width: Option<px>,
        _line_colour: RGBA8,
        _fill_colour: RGBA8,
    ) { unimplemented!()}

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