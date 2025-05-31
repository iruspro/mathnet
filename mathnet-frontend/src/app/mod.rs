mod messages;
mod router;

// region:    --- Modules
use sauron::{
    Application, Cmd, Node,
    web_sys::{Location, console, window},
};

use crate::pages::auth;
use messages::Msg;
use router::Page;
// endregion: --- Modules

pub struct App {
    current_page: Page,
}

impl Application for App {
    type MSG = Msg;

    fn view(&self) -> Node<Msg> {
        self.set_location_hash();

        match &self.current_page {
            Page::Auth(model) => model.view().map_msg(Msg::Auth),
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match &mut self.current_page {
            // Pages
            Page::Auth(model) => match msg {
                Msg::Auth(msg) => model.update(msg).map_msg(Msg::Auth),
            },
        }
    }
}

impl App {
    pub fn new() -> Self {
        App {
            current_page: Page::Auth(auth::Model::default()),
        }
    }

    fn set_location_hash(&self) {
        if let Some(win) = window() {
            let location: Location = win.location();
            if let Err(err) = location.set_hash(&self.current_page.to_uri()) {
                console::log_1(&format!("Failed to set hash: {:?}", err).into());
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}
