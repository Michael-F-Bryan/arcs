use arcs::window::Window;
use kurbo::Size;
use log::Level;
use piet::Color;
use piet_web::WebRenderContext;
use seed::{prelude::*, *};
use specs::prelude::*;
use std::convert::TryFrom;
use web_sys::{HtmlCanvasElement, HtmlElement, MouseEvent};

const CANVAS_ID: &str = "canvas";

pub struct Model {
    world: World,
    window: Window,
    canvas_size: Size,
}

impl Default for Model {
    fn default() -> Model {
        let mut world = World::new();
        arcs::components::register(&mut world);

        let window = Window::create(&mut world);
        window
            .style_mut(&mut world.write_storage())
            .background_colour = Color::rgb8(0xff, 0, 0);

        Model {
            world,
            window,
            canvas_size: Size::new(300.0, 150.0),
        }
    }
}

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders
        .after_next_render(|_| Msg::Rendered)
        .after_next_render(|_| Msg::WindowResized);

    AfterMount::new(Model::default())
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Msg {
    Rendered,
    Clicked,
    WindowResized,
}

impl Msg {
    pub fn from_click_event(_ev: MouseEvent) -> Self { Msg::Clicked }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    log::debug!("Handling {:?}", msg);

    match msg {
        Msg::Rendered => {
            if let Some(canvas) = seed::canvas(CANVAS_ID) {
                draw(&canvas, model);
                orders.skip();
            }
        },
        Msg::Clicked => unimplemented!(),
        Msg::WindowResized => {
            if let Some(parent_size) =
                seed::canvas(CANVAS_ID).and_then(|canvas| parent_size(&canvas))
            {
                log::debug!("Changing the canvas to {}", parent_size);
                model.canvas_size = parent_size;

                orders.render();
            }
        },
    }

    // make sure we redraw the canvas
    orders.after_next_render(|_| Msg::Rendered);
}

fn draw(canvas: &HtmlCanvasElement, model: &mut Model) {
    let mut canvas_ctx = seed::canvas_context_2d(&canvas);
    let browser_window = seed::window();
    let ctx = WebRenderContext::new(&mut canvas_ctx, &browser_window);

    let mut system = model.window.render_system(ctx, model.canvas_size);
    RunNow::setup(&mut system, &mut model.world);
    RunNow::run_now(&mut system, &mut model.world);
}

fn parent_size(element: &HtmlElement) -> Option<Size> {
    let window = seed::window();
    let height = window.inner_height().ok()?.as_f64()?
        - f64::try_from(element.offset_top()).ok()?;
    let width = window.inner_width().ok()?.as_f64()?;

    Some(Size::new(
        f64::try_from(width).ok()?,
        f64::try_from(height).ok()?,
    ))
}

fn view(model: &Model) -> impl View<Msg> {
    div![div![
        attrs![ At::Class => "canvas-container" ],
        style! {
            St::Width => "100%",
            St::Height => "100%",
            St::OverflowY => "hidden",
            St::OverflowX => "hidden",
        },
        canvas![attrs![
            At::Id => CANVAS_ID,
            At::Width => model.canvas_size.width,
            At::Height => model.canvas_size.height,
        ],],
    ]]
}

pub fn window_events(_model: &Model) -> Vec<Listener<Msg>> {
    vec![
        mouse_ev(Ev::KeyDown, Msg::from_click_event),
        simple_ev(Ev::Resize, Msg::WindowResized),
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    console_log::init_with_level(Level::Debug)
        .expect("Unable to initialize the log");

    seed::App::builder(update, view)
        .after_mount(after_mount)
        .window_events(window_events)
        .build_and_start();
}
