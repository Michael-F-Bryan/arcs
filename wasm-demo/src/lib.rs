#![recursion_limit = "128"]

pub mod app;

pub use app::App;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() {
    // Provide better error messages in debug mode.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    yew::start_app::<App>();
}
