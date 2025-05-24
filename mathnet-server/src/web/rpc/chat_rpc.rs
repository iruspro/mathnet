// region:    --- Modules

use super::Result;
use super::{ParamsForCreate, ParamsForUpdate, ParamsIded, params::ParamsList};
use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::chat::{Chat, ChatBmc, ChatFilter, ChatForCreate, ChatForUpdate};

// endregion: --- Modules

pub async fn create_chat(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<ChatForCreate>,
) -> Result<Chat> {
    let ParamsForCreate { data } = params;

    let id = ChatBmc::create(&ctx, &mm, data).await?;
    let chat = ChatBmc::get(&ctx, &mm, id).await?;

    Ok(chat)
}

pub async fn get_chat(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Chat> {
    let ParamsIded { id } = params;

    let chat = ChatBmc::get(&ctx, &mm, id).await?;

    Ok(chat)
}

pub async fn list_chats(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<ChatFilter>,
) -> Result<Vec<Chat>> {
    let chats = ChatBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

    Ok(chats)
}

pub async fn update_chat(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<ChatForUpdate>,
) -> Result<Chat> {
    let ParamsForUpdate { id, data } = params;

    ChatBmc::update(&ctx, &mm, id, data).await?;

    let chat = ChatBmc::get(&ctx, &mm, id).await?;

    Ok(chat)
}

pub async fn delete_chat(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Chat> {
    let ParamsIded { id } = params;

    let chat = ChatBmc::get(&ctx, &mm, id).await?;
    ChatBmc::delete(&ctx, &mm, id).await?;

    Ok(chat)
}
