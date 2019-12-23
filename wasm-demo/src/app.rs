use crate::Msg;
use anyhow::{Context, Error};
use js_sys::Function;
use piet_web::WebRenderContext;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{CanvasRenderingContext2d, Document, Window};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

#[derive(Debug)]
pub struct App {
    link: ComponentLink<Self>,
    canvas: Option<CanvasRenderingContext2d>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let on_canvas_loaded = link.send_back(|_| Msg::CanvasLoaded);
        on_ready(move || on_canvas_loaded.emit(()));

        App { link, canvas: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::trace!("Updating with {:?}", msg);

        match msg {
            Msg::CanvasLoaded => {
                self.canvas =
                    Some(html5_canvas_from_selector("#canvas").unwrap());
            },
            Msg::CanvasClicked(event) => {
                let window = window().unwrap();
                let ctx = self.canvas.as_mut().unwrap();
                let _render_context = WebRenderContext::new(ctx, &window);
                log::debug!("Clicked {:?}", event);
            },
        }
        true
    }

    fn view(&self) -> Html<Self> {
        log::trace!("Redrawing");

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
) -> Result<CanvasRenderingContext2d, Error> {
    document()?
        .query_selector(selector)
        .ok()
        .context("The selector was malformed")?
        .context("Can't find the element")?
        .dyn_into()
        .ok()
        .context("The element wasn't actually a <canvas>")
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
