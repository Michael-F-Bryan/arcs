use arcs::window::Window;
use seed::{prelude::*, *};
use specs::prelude::*;

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
enum Msg {
    Rendered,
    Clicked,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Rendered => {
            draw(&model.world, &model.window);
            // We want to call `.skip` to prevent infinite loop.
            orders.after_next_render(|_| Msg::Rendered).skip();
        },
        Msg::Clicked => {
            unimplemented!();
        },
    }
}

fn draw(world: &World, window: &Window) {
    let canvas = seed::canvas(CANVAS_ID).unwrap();
    let ctx = seed::canvas_context_2d(&canvas);
}

fn view(_model: &Model) -> impl View<Msg> {
    div![
        style! {St::Display => "flex"},
        canvas![
            attrs![
                At::Id => CANVAS_ID,
                At::Width => px(200),
                At::Height => px(100),
            ],
            style![
                St::Border => "1px solid black",
            ],
        ],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}
