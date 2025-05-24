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

    // -- Create Thoughts
    let mut thoughts_ids: Vec<i64> = Vec::new();
    for i in 0..=4 {
        let req_create_thought = hc.do_post(
            "/api/rpc",
            json!({
                "id": 1,
                "method": "create_thought",
                "params": {
                    "data": {
                        "content": format!("thought AAA {i}"),
                        "mather_id": 1000,
                        "on_latex": true,
                    }
                },
            }),
        );
        let result = req_create_thought.await?;
        thoughts_ids.push(result.json_value::<i64>("/result/id")?);
    }

    // -- Update first Thought
    let req_update_thought = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "update_thought",
            "params": {
                "id": thoughts_ids[0],
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

    // -- Delete second Thought
    let req_delete_thought = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "delete_thought",
            "params": {
                "id": thoughts_ids[1],
            }
        }),
    );
    req_delete_thought.await?.print().await?;

    // -- List Thoughts with filters
    let req_list_thoughts = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "list_thoughts",
            "params": {
                "filters": [{
                    "content": {"$endsWith": "BB"}
                }, {
                    "id": thoughts_ids[2],
                }],
                "list_options": {
                    "order_bys": "!id"
                },
            }
        }),
    );
    req_list_thoughts.await?.print().await?;

    Ok(())
}
