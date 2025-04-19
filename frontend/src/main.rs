/*
This module includes router (currently 
the project is relatively small.)
*/

mod app;
use crate::app as appp;
use sauron::prelude::*;
use wasm_bindgen::prelude::*;
pub mod messages;
pub mod router;
pub mod user;
pub mod pages;
pub mod list_of_pages;
use crate::list_of_pages::Page;
use tokio;
mod theserver;
//use web_sys::window::*;


//#[tokio::main]
//async fn main() {
//    theserver::start_server().await;
//}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    Program::mount_to_body(appp::App {
        page: Page::Home,
    });
}
