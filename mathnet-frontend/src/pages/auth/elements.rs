// region:    --- RegionName
use sauron::{
    Node,
    html::{attributes::*, events::*, *},
};

use super::messages::Msg;
use super::{AuthMode, Model};
// endregion: --- RegionName

// TODO: Add styles
impl Model {
    pub(in crate::pages::auth) fn header(&self) -> Node<Msg> {
        header(
            [],
            [
                h1([], [text("MathNet")]),
                p(
                    [],
                    [text(
                        "MathNet is a social network designed specifically for mathematicians. 
                        It provides a platform for communication, collaboration, and knowledge sharing, 
                        with built-in support for LaTeX-based formula rendering.",
                    )],
                ),
            ],
        )
    }

    pub(in crate::pages::auth) fn form_view(&self) -> Node<Msg> {
        let header = self.get_form_header();
        let inputs = self.get_form_inputs();
        let btn_name = self.get_form_btn_name();

        section(
            [],
            [
                h2([], [text(header)]),
                form([], inputs),
                button([on_click(|_| Msg::Submit)], [text(btn_name)]),
            ],
        )
    }

    pub(in crate::pages::auth) fn navigation_view(&self) -> Node<Msg> {
        let links = self.get_navigation_links();

        div([], links)
    }

    fn get_form_header(&self) -> &str {
        match self.auth_mode {
            super::AuthMode::Login => "Login",
            super::AuthMode::SignUp => "Sign Up",
            super::AuthMode::PasswordRecovery => "Password recovery",
        }
    }

    fn get_form_inputs(&self) -> Vec<Node<Msg>> {
        let username = input(
            [
                placeholder("username"),
                value(self.username.clone()),
                on_input(|val: InputEvent| Msg::UpdateUsername(val.value())),
            ],
            [],
        );
        let password = input(
            [
                r#type("password"),
                placeholder("password"),
                value(self.password.clone()),
                on_input(|val: InputEvent| Msg::UpdatePassword(val.value())),
            ],
            [],
        );
        let first_name = input(
            [
                placeholder("first name"),
                value(self.first_name.clone()),
                on_input(|val: InputEvent| Msg::UpdateFirstName(val.value())),
            ],
            [],
        );
        let last_name = input(
            [
                placeholder("last_name"),
                value(self.last_name.clone()),
                on_input(|val: InputEvent| Msg::UpdateLastName(val.value())),
            ],
            [],
        );
        let recovery_code = input(
            [
                placeholder("recovery code"),
                value(self.recovery_code.clone()),
                on_input(|val: InputEvent| Msg::UpdateSecretCode(val.value())),
            ],
            [],
        );

        match self.auth_mode {
            AuthMode::Login => vec![username, password],
            AuthMode::SignUp => vec![username, password, first_name, last_name],
            AuthMode::PasswordRecovery => vec![username, recovery_code],
        }
    }

    fn get_form_btn_name(&self) -> &str {
        match self.auth_mode {
            super::AuthMode::Login => "Login",
            super::AuthMode::SignUp => "Sign Up",
            super::AuthMode::PasswordRecovery => "Recovery",
        }
    }

    fn get_navigation_links(&self) -> Vec<Node<Msg>> {
        let login_link = p(
            [],
            [a(
                [on_click(|_| Msg::UpdateAuthMode(AuthMode::Login))],
                [text("Login")],
            )],
        );
        let sign_up_link = p(
            [],
            [a(
                [on_click(|_| Msg::UpdateAuthMode(AuthMode::SignUp))],
                [text("Sign Up")],
            )],
        );
        let pwd_recovery_link = p(
            [],
            [a(
                [on_click(|_| {
                    Msg::UpdateAuthMode(AuthMode::PasswordRecovery)
                })],
                [text("Password recovery")],
            )],
        );

        match self.auth_mode {
            AuthMode::Login => vec![pwd_recovery_link, sign_up_link],
            AuthMode::SignUp => vec![login_link],
            AuthMode::PasswordRecovery => vec![login_link, sign_up_link],
        }
    }
}
