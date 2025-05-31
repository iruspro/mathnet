// region:    --- Modules
use crate::pages::auth;
// endregion: --- Modules

pub enum Page {
    Auth(auth::Model),
}

impl Page {
    pub fn to_uri(&self) -> String {
        match self {
            Page::Auth(model) => match model.get_auth_mode() {
                auth::AuthMode::Login => "#/login".to_string(),
                auth::AuthMode::SignUp => "#/sign_up".to_string(),
                auth::AuthMode::PasswordRecovery => "#/password_recovery".to_string(),
            },
        }
    }
}
