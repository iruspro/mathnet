mod elements;
pub mod messages;
mod serializers;

// region:    --- Modules
use sauron::{Cmd, Node, html::div, web_sys::console};

use super::elements::common;
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
            Msg::UpdatePassword(val) => self.set_password(val),
            Msg::UpdateSecretCode(val) => self.recovery_code = val,
            Msg::UpdateFirstName(val) => self.first_name = val,
            Msg::UpdateLastName(val) => self.last_name = val,
            Msg::Submit => {
                console::log_1(&"Form send to server".into());
            }

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

    fn get_hidden_password(&self) -> String {
        let len = self.password.len();
        "*".repeat(len)
    }

    // TODO: Should we encrypt data?
    fn set_password(&mut self, val: String) {
        self.password = val;
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
