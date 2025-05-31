pub mod api_client;
pub mod app;
pub mod pages;

// region:    --- Modules
use sauron::{Program, wasm_bindgen};

use app::App;
// endregion: --- Modules

#[wasm_bindgen(start)]
pub fn main() {
    Program::mount_to_body(App::new());
}
