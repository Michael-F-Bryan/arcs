use crate::Msg;
use anyhow::{Context, Error};
use arcs::{
    components::{Dimension, DrawingObject, Geometry, Layer, Name, PointStyle},
    primitives::Point,
    render::{Renderer, Viewport},
    Vector,
};
use js_sys::Function;
use kurbo::Size;
use piet::Color;
use piet_web::WebRenderContext;
use specs::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct App {
    link: ComponentLink<Self>,
    canvas: Option<CanvasRenderingContext2d>,
    renderer: Renderer,
    world: World,
    layer_id: Entity,
}

impl App {
    fn redraw_canvas(&mut self) {
        let window = window().unwrap();
        let ctx = self.canvas.as_mut().unwrap();
        let canvas = canvas_dimensions(&ctx).unwrap();
        self.renderer.viewport.centre =
            Vector::new(canvas.width / 2.0, canvas.height / 2.0);
        let render_context = WebRenderContext::new(ctx, &window);

        log::trace!("Redrawing the canvas with dimensions {:?}", canvas);

        let mut system = self.renderer.system(render_context, canvas);

        RunNow::setup(&mut system, &mut self.world);
        RunNow::run_now(&mut system, &self.world);
    }

    fn add_point(&mut self, location: kurbo::Point) {
        let location = self.to_drawing_coordinates(location);
        let point = self
            .world
            .create_entity()
            .with(DrawingObject {
                geometry: Geometry::Point(Point::new(location)),
                layer: self.layer_id,
            })
            .with(PointStyle {
                colour: Color::WHITE,
                radius: Dimension::Pixels(5.0),
            })
            .build();

        log::info!("Added a point at {:?} (entity: {:?})", location, point);
    }

    fn to_drawing_coordinates(&self, location: kurbo::Point) -> Vector {
        // FIXME: actually translate this properly
        Vector::new(location.x / 2.0, location.y / 2.0)
    }
}

fn create_world_and_default_layer() -> (World, Entity) {
    log::debug!("Initializing the world");
    let mut world = World::new();

    arcs::components::register(&mut world);

    let layer_id = Layer::create(
        world.create_entity(),
        Name::new("base"),
        Layer {
            visible: true,
            z_level: 0,
        },
    );

    (world, layer_id)
}

fn canvas_dimensions(ctx: &CanvasRenderingContext2d) -> Option<Size> {
    let element = ctx.canvas()?;
    let width = element.width();
    let height = element.height();

    Some(Size::new(width.into(), height.into()))
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let on_canvas_loaded = link.send_back(|_| Msg::CanvasLoaded);
        on_ready(move || on_canvas_loaded.emit(()));

        let viewport = Viewport {
            centre: Vector::zero(),
            pixels_per_drawing_unit: 1.0,
        };
        let background = Color::BLACK;

        let (world, layer_id) = create_world_and_default_layer();

        App {
            link,
            world,
            layer_id,
            canvas: None,
            renderer: Renderer::new(viewport, background),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::trace!("Updating with {:?}", msg);

        match msg {
            Msg::CanvasLoaded => {
                self.canvas = Some(
                    html5_canvas_context_from_selector("#canvas").unwrap(),
                );
                self.redraw_canvas();
            },
            Msg::CanvasClicked(event) => {
                log::debug!("Clicked {:?}", event);
                self.add_point(event.screen);
                self.redraw_canvas();
            },
        }
        true
    }

    fn view(&self) -> Html<Self> {
        log::trace!("Updating the view");

        html! {
            <div>
                <nav class="navbar">
                    <ul>
                        <li class="brand" title={ env!("CARGO_PKG_DESCRIPTION") }>
                            <a href="#">{ "arcs WebAssembly Demo" }</a>
                        </li>
                        <li>
                            <a href="https://github.com/Michael-F-Bryan/">{ "Repo" }</a>
                        </li>
                    </ul>
                </nav>

                <main>
                    <canvas id="canvas" onclick=|e| Msg::CanvasClicked(e.into()) />
                </main>
            </div>
        }
    }
}

fn window() -> Result<Window, Error> {
    web_sys::window().context("Unable to get the Window")
}

fn html5_canvas_from_selector(
    selector: &str,
) -> Result<HtmlCanvasElement, Error> {
    document()?
        .query_selector(selector)
        .ok()
        .context("The selector was malformed")?
        .context("Can't find the element")?
        .dyn_into::<HtmlCanvasElement>()
        .ok()
        .context("The element wasn't actually a <canvas>")
}

fn html5_canvas_context_from_selector(
    selector: &str,
) -> Result<CanvasRenderingContext2d, Error> {
    html5_canvas_from_selector(selector)?
        .get_context("2d")
        .ok()
        .context("The call to #canvas.get_context(\"2d\") failed")?
        .context("There is no 2d canvas context")?
        .dyn_into::<CanvasRenderingContext2d>()
        .ok()
        .context("The 2d canvas context wasn't a CanvasRenderingContext2d")
}

fn document() -> Result<Document, Error> {
    window()?.document().context("Unable to get the Document")
}

/// An equivalent of the `$.ready()` function from jQuery.
fn on_ready<F>(cb: F)
where
    F: FnOnce() + 'static,
{
    let document = document().unwrap();
    let ready_state = document.ready_state();
    let js_callback = Closure::once_into_js(cb).dyn_into::<Function>().unwrap();

    match ready_state.as_str() {
        "complete" | "interactive" => {
            web_sys::window()
                .expect("Unable to get the Window")
                .set_timeout_with_callback(&js_callback)
                .unwrap();
        },
        _ => {
            document
                .add_event_listener_with_callback(
                    "DOMContentLoaded",
                    &js_callback,
                )
                .unwrap();
        },
    }
}
