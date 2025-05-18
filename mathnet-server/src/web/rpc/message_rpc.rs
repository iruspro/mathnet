// region:    --- Modules

use super::Result;
use super::{ParamsForCreate, ParamsForUpdate, ParamsIded, params::ParamsList};
use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::message::{
    Message, MessageBmc, MessageFilter, MessageForCreate, MessageForUpdate,
};

// endregion: --- Modules

pub async fn create_message(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<MessageForCreate>,
) -> Result<Message> {
    let ParamsForCreate { data } = params;

    let id = MessageBmc::create(&ctx, &mm, data).await?;
    let message = MessageBmc::get(&ctx, &mm, id).await?;

    Ok(message)
}

pub async fn get_message(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Message> {
    let ParamsIded { id } = params;

    let message = MessageBmc::get(&ctx, &mm, id).await?;

    Ok(message)
}

pub async fn list_messages(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<MessageFilter>,
) -> Result<Vec<Message>> {
    let messages = MessageBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

    Ok(messages)
}

pub async fn update_message(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<MessageForUpdate>,
) -> Result<Message> {
    let ParamsForUpdate { id, data } = params;

    MessageBmc::update(&ctx, &mm, id, data).await?;

    let message = MessageBmc::get(&ctx, &mm, id).await?;

    Ok(message)
}

pub async fn delete_message(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Message> {
    let ParamsIded { id } = params;

    let message = MessageBmc::get(&ctx, &mm, id).await?;
    MessageBmc::delete(&ctx, &mm, id).await?;

    Ok(message)
}
