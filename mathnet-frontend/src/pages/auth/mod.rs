mod elements;
pub mod messages;

// region:    --- Modules
use sauron::{Cmd, Node, html::div, wasm_bindgen_futures::spawn_local, web_sys::console};

use super::elements::common;
use crate::web::{self, serializers::auth::UserForLogin};
use messages::Msg;
// endregion: --- Modules

// region:    --- Types
#[derive(Clone, Copy)]
pub enum AuthMode {
    Login,
    SignUp,
    PasswordRecovery,
}

pub struct Model {
    // Page data
    auth_mode: AuthMode,

    // Login / Sign Up data
    username: String,
    password: String,
    recovery_code: String,
    first_name: String,
    last_name: String,
}
// endregion: --- Types

// region:    --- Main page logic
impl Model {
    // TODO: Do we need to return Cmd<...>?
    pub fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match msg {
            // Page data
            Msg::UpdateAuthMode(auth_mode) => {
                self.reset_text_fields();
                self.auth_mode = auth_mode;
            }

            // Form data
            Msg::UpdateUsername(val) => self.username = val,
            Msg::UpdatePassword(val) => self.password = val,
            Msg::UpdateSecretCode(val) => self.recovery_code = val,
            Msg::UpdateFirstName(val) => self.first_name = val,
            Msg::UpdateLastName(val) => self.last_name = val,
            Msg::Submit => match self.auth_mode {
                AuthMode::Login => {
                    let body = self.get_login_body();
                    spawn_local(async { web::auth::login(body).await });
                }
                AuthMode::SignUp => console::log_1(&"Form send to server".into()),
                AuthMode::PasswordRecovery => console::log_1(&"Form send to server".into()),
            },

            // Other
            Msg::NoOp(_) => {}
        }

        Cmd::none()
    }

    pub fn view(&self) -> Node<Msg> {
        div(
            [],
            [
                self.header(),
                self.form_view(),
                self.navigation_view(),
                common::info_footer().map_msg(Msg::NoOp),
            ],
        )
    }
}
// endregion: --- Main page logic

// region:    --- Model Controller
impl Model {
    pub fn new() -> Self {
        Model {
            // Page data
            auth_mode: AuthMode::Login,

            // Form data
            username: "".to_string(),
            password: "".to_string(),
            recovery_code: "".to_string(),
            first_name: "".to_string(),
            last_name: "".to_string(),
        }
    }

    fn reset_text_fields(&mut self) {
        self.username.clear();
        self.password.clear();
        self.recovery_code.clear();
        self.first_name.clear();
        self.last_name.clear();
    }

    pub fn get_auth_mode(&self) -> AuthMode {
        self.auth_mode
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}
// endregion: --- Model Controller

// region:    --- Web
impl Model {
    fn get_login_body(&self) -> UserForLogin {
        // TODO: Should we encrypt data?
        UserForLogin {
            username: self.username.clone(),
            password: self.password.clone(),
        }
    }

    // fn get_sign_up_body(&self) {
    //     todo!()
    // }
}
// endregion: --- Web
