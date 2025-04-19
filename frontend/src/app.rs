use sauron::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::window;
use crate::messages::{Msg};
use crate::list_of_pages::Page;
use crate::pages::{home, chat, docs,exercise,list_of_exercises,login,register,settings,user_profile};
use sauron::dom::Program;


pub struct App {
    pub page: Page,
}

impl App {
    fn parse_location() -> Page {
        let path = window().unwrap().location().pathname().unwrap_or_default();
        match path.as_str() {
            "/" => Page::Home,
            "/docs" => Page::Docs,
            "/chat" => Page::Chat,
            "/exercise" => Page::Exercise,
            "/list_of_exercises" => Page::ListOfExercise,
            "/login" => Page::Login,
            "/register" => Page::Register,
            "/settings" => Page::Settings,
            "/user_profile" => Page::UserProfile,
            _ => Page::Home,
        }
    }
}

impl Application for App {
    type MSG = Msg;
    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match msg {
            Msg::NavigateTo(path) => {
                let _ = window().unwrap().history().unwrap().push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&path));
                self.page = Self::parse_location();
            }
            Msg::UrlChanged => {
                self.page = Self::parse_location();
            }
            Msg::Unchanged => {
                self.page = Self::parse_location();
            }
        }
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        let nav = nav(
            [],
            [
                a([href("#"), on_click(|_| Msg::NavigateTo("/".to_string()))], [text("Home")]),
                text(" | "),
                a([href("#"), on_click(|_| Msg::NavigateTo("/about".to_string()))], [text("About")]),
            ],
        );

        let content = match self.page {
            Page::Home => home::view(),
            Page::Chat => chat::view(),
            Page::Docs => docs::view(),
            Page::Exercise => exercise::view(),
            Page::ListOfExercise => list_of_exercises::view(),
            Page::Login => login::view(),
            Page::Register => register::view(),
            Page::Settings => settings::view(),
            Page::UserProfile => user_profile::view(),
        };

        div([], [nav, hr([],[]), content])
    }

    fn init(&mut self) -> Cmd<Msg> {
        self.page = Self::parse_location();
        Cmd::none()
    }
}

