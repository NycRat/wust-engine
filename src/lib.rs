use wasm_bindgen::prelude::*;

pub mod application;
pub mod mouse;
pub mod object;
pub mod objs;
pub mod state;
pub mod transformations;
pub mod utils;
pub mod vec3;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    application::Application::init()
}
