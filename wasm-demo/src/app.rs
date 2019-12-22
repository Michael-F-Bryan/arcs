use anyhow::{Context, Error};
use crate::Msg;
use arcs_render::html5_canvas::Html5Canvas;
use wasm_bindgen::{closure::Closure, JsCast};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use web_sys::Document;
use js_sys::Function;

#[derive(Debug)]
pub struct App {
    link: ComponentLink<Self>,
    canvas: Option<Html5Canvas>,
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
                self.canvas = Some(html5_canvas_from_selector("#canvas").unwrap());
            }
            Msg::CanvasClicked(_event) => unimplemented!(),
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

fn html5_canvas_from_selector(selector: &str) -> Result<Html5Canvas, Error> {
    let element = document()?
        .query_selector(selector)
        .ok()
        .context("The selector was malformed")?
        .context("Can't find the element")?
        .dyn_into()
        .ok()
        .context("The element wasn't actually a <canvas>")?;

    Html5Canvas::for_element(element)
}

fn document() -> Result<Document, Error> {
    Ok(web_sys::window()
        .context("Unable to get the Window")?
        .document()
        .context("Unable to get the Document")?)
}

fn on_ready<F>(cb: F)
where F: FnOnce() + 'static
{
    let document = document().unwrap();
    let ready_state = document.ready_state();
    let js_callback = Closure::once_into_js(cb)
        .dyn_into::<Function>()
        .unwrap();

    match ready_state.as_str() {
        "complete" | "interactive" => {
            web_sys::window()
                .expect("Unable to get the Window")
                .set_timeout_with_callback(&js_callback)
                .unwrap();
        }
        _ => {
            document.add_event_listener_with_callback("DOMContentLoaded", &js_callback)
                .unwrap();
        }
    }
}