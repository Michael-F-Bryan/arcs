pub mod modes;

use arcs::{
    components::{Dimension, Layer, Name, PointStyle},
    window::Window,
    CanvasSpace,
};
use euclid::{Point2D, Size2D};
use log::Level;
use modes::{
    ApplicationContext, Idle, KeyboardEventArgs, MouseButtons, MouseEventArgs,
    State, Transition, VirtualKeyCode, AddPointMode, AddLineMode
};
use piet::Color;
use piet_web::WebRenderContext;
use seed::{prelude::*, *};
use specs::prelude::*;
use std::convert::TryFrom;
use web_sys::{HtmlCanvasElement, HtmlElement, KeyboardEvent, MouseEvent};

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
            current_state: Box::new(AddLineMode::default()),
        }
    }
}

impl Model {
    fn handle_event<F>(&mut self, handler: F) -> bool
    where
        F: FnOnce(&mut dyn State, &mut Context<'_>) -> Transition,
    {
        let mut suppress_redraw = false;
        let transition = handler(
            &mut *self.current_state,
            &mut Context {
                world: &mut self.world,
                window: &mut self.window,
                default_layer: self.default_layer,
                suppress_redraw: &mut suppress_redraw,
            },
        );
        self.handle_transition(transition);
        !suppress_redraw
    }

    fn on_mouse_down(&mut self, cursor: Point2D<f64, CanvasSpace>) -> bool {
        let args = self.mouse_event_args(cursor);
        log::debug!("[ON_MOUSE_DOWN] {:?}, {:?}", args, self.current_state);
        self.handle_event(|state, ctx| state.on_mouse_down(ctx, &args))
    }

    fn on_mouse_up(&mut self, cursor: Point2D<f64, CanvasSpace>) -> bool {
        let args = self.mouse_event_args(cursor);
        log::debug!("[ON_MOUSE_UP] {:?}, {:?}", args, self.current_state);
        self.handle_event(|state, ctx| state.on_mouse_up(ctx, &args))
    }

    fn on_mouse_move(&mut self, cursor: Point2D<f64, CanvasSpace>) -> bool {
        let args = self.mouse_event_args(cursor);
        self.handle_event(|state, ctx| state.on_mouse_move(ctx, &args))
    }

    fn on_key_pressed(&mut self, args: KeyboardEventArgs) -> bool {
        log::debug!("[ON_KEY_PRESSED] {:?}, {:?}", args, self.current_state);
        self.handle_event(|state, ctx| state.on_key_pressed(ctx, &args))
    }

    fn handle_transition(&mut self, transition: Transition) {
        match transition {
            Transition::ChangeState(new_state) => {
                log::debug!(
                    "Changing state {:?} => {:?}",
                    self.current_state,
                    new_state
                );
                self.current_state = new_state
            },
            Transition::DoNothing => {},
        }
    }

    fn mouse_event_args(
        &self,
        cursor: Point2D<f64, CanvasSpace>,
    ) -> MouseEventArgs {
        let viewports = self.world.read_storage();
        let viewport = self.window.viewport(&viewports);
        let location = arcs::window::to_drawing_coordinates(
            cursor,
            viewport,
            self.canvas_size,
        );

        MouseEventArgs {
            location,
            cursor,
            button_state: MouseButtons::LEFT_BUTTON,
        }
    }
}

/// A temporary struct which presents a "view" of [`Model`] which can be used
/// as a [`ApplicationContext`].
struct Context<'model> {
    world: &'model mut World,
    window: &'model mut Window,
    default_layer: Entity,
    suppress_redraw: &'model mut bool,
}

impl<'model> ApplicationContext for Context<'model> {
    fn world(&self) -> &World { &self.world }

    fn world_mut(&mut self) -> &mut World { &mut self.world }

    fn viewport(&self) -> Entity { self.window.0 }

    fn default_layer(&self) -> Entity { self.default_layer }

    fn suppress_redraw(&mut self) { *self.suppress_redraw = true; }
}

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders
        .after_next_render(|_| Msg::Rendered)
        .after_next_render(|_| Msg::WindowResized);

    AfterMount::new(Model::default())
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    log::trace!("Handling {:?}", msg);

    let needs_render = match msg {
        Msg::Rendered => true,
        Msg::MouseDown(cursor) => model.on_mouse_down(cursor),
        Msg::MouseUp(cursor) => model.on_mouse_up(cursor),
        Msg::MouseMove(cursor) => model.on_mouse_move(cursor),
        Msg::KeyPressed(args) => model.on_key_pressed(args),
        Msg::WindowResized => {
            if let Some(parent_size) =
                seed::canvas(CANVAS_ID).and_then(|canvas| parent_size(&canvas))
            {
                log::debug!("Changing the canvas to {}", parent_size);
                model.canvas_size = parent_size;
            }

            true
        },
    };

    if needs_render {
        if let Some(canvas) = seed::canvas(CANVAS_ID) {
            draw(&canvas, model);
        }
    }
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
                At::TabIndex => "1",
            ],
            mouse_ev(Ev::MouseDown, |e| Msg::MouseDown(canvas_location(e))),
            mouse_ev(Ev::MouseUp, |e| Msg::MouseUp(canvas_location(e))),
            mouse_ev(Ev::MouseMove, |e| Msg::MouseMove(canvas_location(e))),
            keyboard_ev(Ev::KeyDown, Msg::from_key_press)
        ],
    ]]
}

pub fn window_events(_model: &Model) -> Vec<Listener<Msg>> {
    vec![simple_ev(Ev::Resize, Msg::WindowResized)]
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Msg {
    Rendered,
    MouseDown(Point2D<f64, CanvasSpace>),
    MouseUp(Point2D<f64, CanvasSpace>),
    MouseMove(Point2D<f64, CanvasSpace>),
    KeyPressed(KeyboardEventArgs),
    WindowResized,
}

fn canvas_location(ev: MouseEvent) -> Point2D<f64, CanvasSpace> {
    let x = ev.offset_x().into();
    let y = ev.offset_y().into();

    Point2D::new(x, y)
}

impl Msg {
    pub fn from_key_press(ev: KeyboardEvent) -> Self {
        let key = match ev.key().parse::<VirtualKeyCode>() {
            Ok(got) => Some(got),
            Err(_) => {
                // encountered an unknown key code, log it so we can update the
                // FromStr impl
                log::warn!("Encountered an unknown key: {}", ev.key());
                None
            },
        };

        Msg::KeyPressed(KeyboardEventArgs {
            shift_pressed: ev.shift_key(),
            control_pressed: ev.ctrl_key(),
            key,
        })
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
