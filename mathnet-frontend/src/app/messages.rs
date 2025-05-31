// region:    --- Modules
use crate::pages::auth;
// endregion: --- Modules

pub enum Msg {
    // Pages
    Auth(auth::messages::Msg),
}
