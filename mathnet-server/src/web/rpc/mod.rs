// region:    --- Modules

mod chat_rpc;
mod mather_rpc;
mod message_rpc;
mod params;
mod thought_rpc;
mod user_group_rpc;
mod user_rpc;

use super::{Error, Result};
use crate::ctx::Ctx;
use crate::model::ModelManager;
use chat_rpc::{create_chat, delete_chat, get_chat, list_chats, update_chat};
use mather_rpc::{create_mather, delete_mather, get_mather, list_mathers, update_mather};
use message_rpc::{create_message, delete_message, get_message, list_messages, update_message};
use params::*;
use thought_rpc::{create_thought, delete_thought, get_thought, list_thoughts, update_thought};
use user_group_rpc::{
    create_user_group, delete_user_group, get_user_group, list_user_groups, update_user_group,
};
use user_rpc::{create_user, delete_user, get_user, list_users, update_user};

use axum::{
    Json, Router,
    extract::State,
    response::{IntoResponse, Response},
    routing::post,
};
use serde::Deserialize;
use serde_json::{Value, from_value, json, to_value};
use std::sync::Arc;
use tracing::debug;

// endregion: --- Modules

// region:    --- RPC Types
/// The raw JSON-RPC request object, serving as the foundation for RPC routing.
#[derive(Deserialize)]
struct RpcRequest {
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

/// RPC basic information containing the rpc request id
/// and method for additional logging purposes.
#[derive(Debug)]
pub struct RpcInfo {
    pub id: Option<Value>,
    pub method: String,
}
// endregion: --- RPC Types

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/rpc", post(rpc_handler))
        .with_state(mm)
}

async fn rpc_handler(
    State(mm): State<ModelManager>,
    ctx: Ctx,
    Json(rpc_req): Json<RpcRequest>,
) -> Response {
    // -- Create the RPC Info to be set to the response.extensions.
    let rpc_info = RpcInfo {
        id: rpc_req.id.clone(),
        method: rpc_req.method.clone(),
    };

    // -- Exec & Store RpcInfo in response.
    let mut res = _rpc_handler(ctx, mm, rpc_req).await.into_response();
    res.extensions_mut().insert(Arc::new(rpc_info));

    res
}

macro_rules! exec_rpc_fn {
    // With Params
    ($rpc_fn:expr, $ctx:expr, $mm:expr, $rpc_params:expr) => {{
        let rpc_fn_name = stringify!($rpc_fn);
        let params = $rpc_params.ok_or(Error::RpcMissingParams {
            rpc_method: rpc_fn_name.to_string(),
        })?;
        let params = from_value(params).map_err(|_| Error::RpcFailJsonParams {
            rpc_method: rpc_fn_name.to_string(),
        })?;

        $rpc_fn($ctx, $mm, params).await.map(to_value)??
    }};

    // Without Params
    ($rpc_fn:expr, $ctx:expr, $mm:expr) => {
        $rpc_fn($ctx, $mm).await.map(to_value)??
    };
}

async fn _rpc_handler(ctx: Ctx, mm: ModelManager, rpc_req: RpcRequest) -> Result<Json<Value>> {
    let RpcRequest {
        id: rpc_id,
        method: rpc_method,
        params: rpc_params,
    } = rpc_req;

    debug!("{:<12} - _rpc_handler - method: {rpc_method}", "HANDLER");

    let result_json: Value = match rpc_method.as_str() {
        // -- Chat methods
        "create_chat" => exec_rpc_fn!(create_chat, ctx, mm, rpc_params),
        "get_chat" => exec_rpc_fn!(get_chat, ctx, mm, rpc_params),
        "list_chats" => exec_rpc_fn!(list_chats, ctx, mm, rpc_params),
        "update_chat" => exec_rpc_fn!(update_chat, ctx, mm, rpc_params),
        "delete_chat" => exec_rpc_fn!(delete_chat, ctx, mm, rpc_params),

        // -- Mather RPC methods
        "create_mather" => exec_rpc_fn!(create_mather, ctx, mm, rpc_params),
        "get_mather" => exec_rpc_fn!(get_mather, ctx, mm, rpc_params),
        "list_mathers" => exec_rpc_fn!(list_mathers, ctx, mm, rpc_params),
        "update_mather" => exec_rpc_fn!(update_mather, ctx, mm, rpc_params),
        "delete_mather" => exec_rpc_fn!(delete_mather, ctx, mm, rpc_params),

        // -- Message RPC methods
        "create_message" => exec_rpc_fn!(create_message, ctx, mm, rpc_params),
        "get_message" => exec_rpc_fn!(get_message, ctx, mm, rpc_params),
        "list_messages" => exec_rpc_fn!(list_messages, ctx, mm, rpc_params),
        "update_message" => exec_rpc_fn!(update_message, ctx, mm, rpc_params),
        "delete_message" => exec_rpc_fn!(delete_message, ctx, mm, rpc_params),

        // -- Thought RPC methods
        "create_thought" => exec_rpc_fn!(create_thought, ctx, mm, rpc_params),
        "get_thought" => exec_rpc_fn!(get_thought, ctx, mm, rpc_params),
        "list_thoughts" => exec_rpc_fn!(list_thoughts, ctx, mm, rpc_params),
        "update_thought" => exec_rpc_fn!(update_thought, ctx, mm, rpc_params),
        "delete_thought" => exec_rpc_fn!(delete_thought, ctx, mm, rpc_params),

        // -- UserGroup RPC methods
        "create_user_group" => exec_rpc_fn!(create_user_group, ctx, mm, rpc_params),
        "get_user_group" => exec_rpc_fn!(get_user_group, ctx, mm, rpc_params),
        "list_user_groups" => exec_rpc_fn!(list_user_groups, ctx, mm, rpc_params),
        "update_user_group" => exec_rpc_fn!(update_user_group, ctx, mm, rpc_params),
        "delete_user_group" => exec_rpc_fn!(delete_user_group, ctx, mm, rpc_params),

        // -- User RPC methods
        "create_user" => exec_rpc_fn!(create_user, ctx, mm, rpc_params),
        "get_user" => exec_rpc_fn!(get_user, ctx, mm, rpc_params),
        "list_users" => exec_rpc_fn!(list_users, ctx, mm, rpc_params),
        "update_user" => exec_rpc_fn!(update_user, ctx, mm, rpc_params),
        "delete_user" => exec_rpc_fn!(delete_user, ctx, mm, rpc_params),

        // -- Fallback as Err
        _ => return Err(Error::RpcMethodUnknown(rpc_method)),
    };

    let body_response = json!({
        "id": rpc_id,
        "result": result_json
    });

    Ok(Json(body_response))
}
