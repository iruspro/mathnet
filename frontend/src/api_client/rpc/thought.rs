use gloo_net::http::{Method, RequestBuilder};
use crate::web_sys::console;
use serde_json::json;

pub async fn create_thoughts() {
    let request_builder = RequestBuilder::new("/api/rpc")
        .method(Method::POST);

    let request = request_builder
         .json(&json!({
                "id": 1,
                "method": "create_thought",
                "params": {
                    "data": {
                        "content": "thought AAA",
                        "mather_id": 1000,
                        "on_latex": true,
                    }
                },
            }))
        .unwrap();
    
    let resp = request.send().await.unwrap();

    let text = resp.text().await.unwrap();
    console::log_1(&text.into());
}