// region:    --- Modules

use serde_derive::{Deserialize, Serialize};

// endregion: --- Modules

// region:    --- Common serializer
#[derive(Debug, Deserialize)]
pub struct AuthResultData {
    pub success: bool,
}

#[derive(Debug, Deserialize)]
pub struct AuthResult {
    pub result: AuthResultData,
}
// endregion: --- Common serializer

// region:    --- Login Serializers
#[derive(Debug, Serialize)]
pub struct UserForLogin {
    pub username: String,
    pub password: String,
}
// endregion: --- Login Serializers

// region:    --- Sign Up Serializers
#[derive(Debug, Serialize)]
pub struct UserForCreate {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}
// endregion: --- Sign Up Serializers

// region:    --- Password recovery Serializers
#[derive(Debug, Serialize)]
pub struct PasswordRecovery {
    pub username: String,
    pub recovery_code: String,
}
// endregion: --- Password recovery Serializers
