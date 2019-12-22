#![recursion_limit = "128"]

mod app;
mod msg;

pub use app::App;
pub use msg::{MouseEvent, Msg};

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() {
    // Provide better error messages in debug mode.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    web_logger::init();
    yew::start_app::<App>();
}
