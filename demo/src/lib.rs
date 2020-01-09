use arcs::window::Window;
use kurbo::Size;
use piet_web::WebRenderContext;
use seed::{prelude::*, *};
use specs::prelude::*;
use std::convert::TryInto;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, HtmlElement, MouseEvent};

const CANVAS_ID: &str = "canvas";

pub struct Model {
    world: World,
    window: Window,
}

impl Default for Model {
    fn default() -> Model {
        let mut world = World::new();
        arcs::components::register(&mut world);
        let window = Window::create(&mut world);

        Model { world, window }
    }
}

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.after_next_render(|_| Msg::Rendered);
    AfterMount::new(Model::default())
}

#[derive(Copy, Clone)]
pub enum Msg {
    Rendered,
    Clicked,
    WindowResized,
}

impl Msg {
    pub fn from_click_event(_ev: MouseEvent) -> Self { Msg::Clicked }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Rendered => {
            draw(&mut model.world, &model.window);
            // We want to call `.skip` to prevent infinite loop.
            orders.after_next_render(|_| Msg::Rendered).skip();
        },
        Msg::Clicked => unimplemented!(),
        Msg::WindowResized => {
            if let Some(mut canvas) = seed::canvas(CANVAS_ID) {
                resize_to_fill_parent(&mut canvas);
            }
        },
    }
}

fn draw(world: &mut World, window: &Window) {
    let canvas = seed::canvas(CANVAS_ID).unwrap();
    let mut canvas_ctx = seed::canvas_context_2d(&canvas);
    let browser_window = seed::window();
    let ctx = WebRenderContext::new(&mut canvas_ctx, &browser_window);
    let window_size = Size::new(canvas.width().into(), canvas.height().into());

    let mut system = window.render_system(ctx, window_size);
    RunNow::setup(&mut system, world);
    RunNow::run_now(&mut system, world);
}

fn resize_to_fill_parent(canvas: &mut HtmlCanvasElement) {
    if let Some(parent) = canvas
        .parent_element()
        .and_then(|e| e.dyn_into::<HtmlElement>().ok())
    {
        canvas.set_width(parent.offset_width().try_into().unwrap());
        canvas.set_height(parent.offset_height().try_into().unwrap());
    }
}

fn view(_model: &Model) -> impl View<Msg> {
    div![
        style! {St::Display => "flex"},
        div![canvas![
            attrs![ At::Id => CANVAS_ID ],
            style![
                St::Border => "1px solid black",
            ],
        ],]
    ]
}

pub fn window_events(_model: &Model) -> Vec<Listener<Msg>> {
    vec![
        mouse_ev(Ev::KeyDown, Msg::from_click_event),
        simple_ev(Ev::Resize, Msg::WindowResized),
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::builder(update, view)
        .after_mount(after_mount)
        .window_events(window_events)
        .build_and_start();
}
