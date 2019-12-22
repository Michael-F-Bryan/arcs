use crate::{canvas::Canvas, px, Point};
use rgb::RGBA8;
use std::convert::TryInto;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

/// A [`Canvas`] backed by a HTML5 `<canvas>` element.
#[derive(Debug, Clone)]
pub struct Html5Canvas {
    ctx: DrawingContext,
}

impl Html5Canvas {
    pub const fn new(ctx: CanvasRenderingContext2d) -> Self {
        Html5Canvas {
            ctx: DrawingContext(ctx),
        }
    }

    pub fn for_element(elem: HtmlCanvasElement) -> Self {
        let ctx = elem
            .get_context("2d")
            .expect("Retrieving the 2D context threw an error")
            .expect("Unable to retrieve the 2D drawing context")
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Html5Canvas::new(ctx)
    }
}

#[derive(Debug, Clone)]
pub struct DrawingContext(CanvasRenderingContext2d);

impl crate::canvas::DrawingContext for DrawingContext {
    fn line(
        &mut self,
        _start: Point,
        _end: Point,
        _line_width: px,
        _line_colour: RGBA8,
    ) {
        unimplemented!()
    }

    /// Draw a rectangle.
    fn rect(
        &mut self,
        _top_left: Point,
        _width: px,
        _height: px,
        _line_width: Option<px>,
        _line_colour: RGBA8,
        _fill_colour: RGBA8,
    ) {
        unimplemented!()
    }

    fn ellipse(
        &mut self,
        _centre: Point,
        _minor_radius: px,
        _major_radius: px,
        _start_angle: f64,
        _end_angle: f64,
        _rotation_angle: f64,
        _line_width: Option<px>,
        _line_colour: RGBA8,
        _fill_colour: RGBA8,
    ) {
        unimplemented!()
    }
}

impl Canvas for Html5Canvas {
    type DrawingContext = DrawingContext;

    fn dimensions(&self) -> (px, px) {
        let elem = self.ctx.0.canvas().expect(
            "Unable to retrieve the canvas element this context is attached to",
        );
        let width = elem.width().try_into().unwrap();
        let height = elem.height().try_into().unwrap();

        (px::new(width), px::new(height))
    }

    fn drawing_context(&mut self) -> &mut DrawingContext { &mut self.ctx }
}
