pub mod modes;

use arcs::{
    components::{Dimension, Layer, Name, PointStyle},
    window::Window,
    CanvasSpace, DrawingSpace,
};
use euclid::{Point2D, Size2D};
use log::Level;
use modes::{ApplicationContext, Idle, State, Transition};
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
    default_layer: Entity,
    canvas_size: Size2D<f64, CanvasSpace>,
    current_state: Box<dyn State>,
}

impl Default for Model {
    fn default() -> Model {
        let mut world = World::new();
        arcs::components::register(&mut world);
        let builder = world.create_entity().with(PointStyle {
            radius: Dimension::Pixels(10.0),
            ..Default::default()
        });
        let default_layer =
            Layer::create(builder, Name::new("default"), Layer::default());

        let window = Window::create(&mut world);
        window
            .style_mut(&mut world.write_storage())
            .background_colour = Color::rgb8(0xff, 0xcc, 0xcb);

        Model {
            world,
            window,
            default_layer,
            canvas_size: Size2D::new(300.0, 150.0),
            current_state: Box::new(Idle::default()),
        }
    }
}

impl Model {
    fn on_mouse_down(
        &mut self,
        location: Point2D<f64, DrawingSpace>,
        cursor: Point2D<f64, CanvasSpace>,
    ) {
        let args = modes::MouseEventArgs {
            location,
            cursor,
            button_state: modes::MouseButtons::LEFT_BUTTON,
        };

        let mut ctx = Context {
            world: &mut self.world,
            window: &mut self.window,
        };
        let trans = self.current_state.on_mouse_down(&mut ctx, &args);
        self.handle_transition(trans);
    }

    fn handle_transition(&mut self, transition: Transition) {
        match transition {
            Transition::ChangeState(new_state) => {
                log::trace!(
                    "Changing state {:?} => {:?}",
                    self.current_state,
                    new_state
                );
                self.current_state = new_state
            },
            Transition::DoNothing => {},
        }
    }
}

/// A temporary struct which presents a "view" of [`Model`] which can be used
/// as a [`ApplicationContext`].
struct Context<'model> {
    world: &'model mut World,
    window: &'model mut Window,
}

impl<'model> ApplicationContext for Context<'model> {
    fn world(&self) -> &World { &self.world }

    fn world_mut(&mut self) -> &mut World { &mut self.world }

    fn viewport(&self) -> Entity { self.window.0 }
}

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders
        .after_next_render(|_| Msg::Rendered)
        .after_next_render(|_| Msg::WindowResized);

    AfterMount::new(Model::default())
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
        Msg::Clicked(cursor) => {
            let location = {
                let viewports = model.world.read_storage();
                let viewport = model.window.viewport(&viewports);
                arcs::window::to_drawing_coordinates(
                    cursor,
                    viewport,
                    model.canvas_size,
                )
            };
            model.on_mouse_down(location, cursor);
        },
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
    RunNow::run_now(&mut system, &model.world);
}

fn parent_size(element: &HtmlElement) -> Option<Size2D<f64, CanvasSpace>> {
    let window = seed::window();
    let height = window.inner_height().ok()?.as_f64()?
        - f64::try_from(element.offset_top()).ok()?;
    let width = window.inner_width().ok()?.as_f64()?;
    log::debug!("parent size is {}x{}", height, width);

    Some(Size2D::new(
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
        canvas![
            attrs![
                At::Id => CANVAS_ID,
                At::Width => model.canvas_size.width,
                At::Height => model.canvas_size.height,
            ],
            mouse_ev(Ev::MouseDown, Msg::from_click_event)
        ],
    ]]
}

pub fn window_events(_model: &Model) -> Vec<Listener<Msg>> {
    vec![simple_ev(Ev::Resize, Msg::WindowResized)]
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Msg {
    Rendered,
    Clicked(Point2D<f64, CanvasSpace>),
    WindowResized,
}

impl Msg {
    pub fn from_click_event(ev: MouseEvent) -> Self {
        let x = ev.offset_x().into();
        let y = ev.offset_y().into();

        Msg::Clicked(Point2D::new(x, y))
    }
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
