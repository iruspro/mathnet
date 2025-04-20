use sauron::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::window;
use crate::messages::Msg;
use crate::list_of_pages::{self, Page};
use crate::pages::{home, chat, docs,exercise,list_of_exercises,login,register,settings,user_profile};
use sauron::dom::Program;


pub struct App {
    pub current_page: Page,
}

impl App {
    pub fn new() -> Self {
        App {
            current_page : Page::Home,
        }
    }
    //fn parse_location() -> Page {
    //    let path = window().unwrap().location().pathname().unwrap_or_default();
    //    match path.as_str() {
    //        "/" => Page::Home,
    //        "/docs" => Page::Docs,
    //        "/chat" => Page::Chat,
    //        "/exercise" => Page::Exercise,
    //        "/list_of_exercises" => Page::ListOfExercise,
    //        "/login" => Page::Login,
    //        "/register" => Page::Register,
    //        "/settings" => Page::Settings,
    //        "/user_profile" => Page::UserProfile,
    //        _ => Page::Home,
    //    }
    //}
}//

impl Application for App {
    type MSG = Msg;
    fn update(&mut self, msg: Self::MSG) -> Cmd<Msg> {
        match msg {
            Msg::GoToHomePage => self.current_page = Page::Home,
            Msg::GoToDocsPage => self.current_page = Page::Docs,
        }
        Cmd::none()
    }

    fn view(&self) -> Node<Self::MSG> {
        match self.current_page {
            Page::Home => home::view(),
            Page::Chat => chat::view(),
            Page::Docs => docs::view(),
            Page::Exercise => exercise::view(),
            Page::ListOfExercise => list_of_exercises::view(),
            Page::Login => login::view(),
            Page::Register => register::view(),
            Page::Settings => settings::view(),
            Page::UserProfile => user_profile::view(),
        }
    }

    }

    #[wasm_bindgen(start)]
    pub fn start() {
        console_log::init_with_level(log::Level::Trace).unwrap();
        console_error_panic_hook::set_once();
        Program::mount_to_body(App::new());
    }
