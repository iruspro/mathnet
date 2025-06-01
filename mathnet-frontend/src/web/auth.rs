// region:    --- Modules

use super::serializers::auth::{AuthResult, UserForLogin};
use crate::api_client::do_post;

// endregion: --- Modules

pub async fn login(body: UserForLogin) {
    let _response: AuthResult = do_post("/api/login", body)
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    // let response = do_post("api/login", body).await;
}
