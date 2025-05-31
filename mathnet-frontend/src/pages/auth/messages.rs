// region:    --- Modules
use super::AuthMode;
// endregion: --- Modules

pub enum Msg {
    // Page data
    UpdateAuthMode(AuthMode),

    // Form data
    UpdateUsername(String),
    UpdatePassword(String),
    UpdateSecretCode(String),
    UpdateFirstName(String),
    UpdateLastName(String),
    Submit,

    // Other
    NoOp(()),
}
