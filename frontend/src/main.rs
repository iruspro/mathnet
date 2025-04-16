/*
This module includes router (currently 
the project is relatively small.)
*/

use crate::pages::home;
use crate::pages::login;


use sauron::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::window::*;

#[derive(Debug, Clone)]
enum Msg {
    NavigateTo(String),
    UlrChanged,

}

#[derive(Debug,Clone,PartialEq)]
enum Page {
Home,
Login,
UserProfile,
Settings,
Register,
Chat,
Docs,
ListOfExercises,
Exercise
}

struct App {
    page: Page,
}



impl App{
    fn parse_location() -> Page {
        let path = window().unwrap().location().pathname().unwrap_or_default();
        match path.as_str() {
            "/" => Page::Home,
            "/login" => Page::Login,
            "/user_profile" => Page::UserProfile,
            "/settings" => Page::Settings,
            "/register" => Page::Register,
            "/chat" => Page::Chat,
            "/docs" => Page:Docs,
            "/list_of_exercises" => Page::ListOfExercises,
            "/exercise" => Page::Exercise,
        }
    }
}
    impl Application<Msg> for App {
        fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
            match msg {
                Msg::NavigateTo(path) => {
                    let _ = window().unwrap().history().unwrap().push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&path));
                    self.page = Self::parse_location();
                }
                Msg::UrlChanged => {
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
                    a([href("#"), on_click(|_| Msg::NavigateTo("/login".to_string()))], [text("Login")]),
                    a([href("#"), on_click(|_| Msg::NavigateTo("/user_profile".to_string()))], [text("User profile")]),
                    a([href("#"), on_click(|_| Msg::NavigateTo("/register".to_string()))], [text("User profile")]),
                    a([href("#"), on_click(|_| Msg::NavigateTo("/settings".to_string()))], [text("Settings")]),
                    a([href("#"), on_click(|_| Msg::NavigateTo("/chat".to_string()))], [text("Chat")]),
                    a([href("#"), on_click(|_| Msg::NavigateTo("/docs".to_string()))], [text("Documentation")]),
                    a([href("#"), on_click(|_| Msg::NavigateTo("/list_of_exercises".to_string()))], [text("List of exercises")]),
                    a([href("#"), on_click(|_| Msg::NavigateTo("/exercise".to_string()))], [text("Exercise")]),
                ],
            );
    
            let content = match self.page {
                Page::Home => home::view(),
                Page::Login => login::view(),
                Page::UserProfile => user_profile::view(),
                Page::Settings => settings::view(),
                Page::Register => register::view(),
                Page::Chat => chat::view(),
                Page::Docs => docs::view(),
                Page::ListOfExercise => list_of_exercises::view(),
                Page::Exercise => exercise::view(),
            };
    
            div([], [nav, hr([]), content])
        }
    
        fn init(&mut self) -> Cmd<Self, Msg> {
            self.page = Self::parse_location();
            sauron::dom::add_event_listener_with_event("popstate", |_| Msg::UrlChanged);
            Cmd::none()
        }
    }
    
    #[wasm_bindgen(start)]
    pub fn main() {
        console_error_panic_hook::set_once();
        Program::mount_to_body(App {
            page: Page::Home,
        });
    }
