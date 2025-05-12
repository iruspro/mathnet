#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // hc.do_get("/index.html").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "admin",
            "pwd": "password"
        }),
    );
    req_login.await?.print().await?;

    let req_create_thought = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "create_thought",
            "params": {
                "data": {
                    "content": "thought AAAB"
                }
            },
        }),
    );
    req_create_thought.await?.print().await?;

    let req_update_thought = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "update_thought",
            "params": {
                "id": 1000, // Hardcode the task id.
                "data": {
                    "content": "content BB"
                }
            },
        }),
    );
    req_update_thought.await?.print().await?;

    let req_get_thought = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "get_thought",
            "params": {
                "id": 1000,
            },
        }),
    );
    req_get_thought.await?.print().await?;

    let req_delete_thought = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "delete_thought",
            "params": {
                "id": 1001, // Hardcode the task id
            }
        }),
    );
    req_delete_thought.await?.print().await?;

    let req_list_thoughts = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "list_thoughts"
        }),
    );
    req_list_thoughts.await?.print().await?;

    Ok(())
}
