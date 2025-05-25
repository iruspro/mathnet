use gloo_net::http::Request;
use serde::{Serialize,Deserialize};
use serde_json::{Value, json};
use axum::{response::{Response}, Json};

#[derive(Serialize,Deserialize)]
pub struct RpcLogin{
    username : String,
    pwd : String
}


pub async fn rpc_login(user_name : String, user_password : String) -> Result<Value, String> {
    let login_data = RpcLogin {
        username: user_name,
        pwd: user_password,
    };


    let response = Request::post("/api/login")
        .json(&login_data)
        .map_err(|e| format!("Request failed: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Send error: {e}"))?;

    response
        .json::<Value>()
        .await
        .map_err(|e| format!("Deserialize error: {e}"))
}
