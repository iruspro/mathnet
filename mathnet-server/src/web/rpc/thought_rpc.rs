use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::thought::{
    Thought, ThoughtBmc, ThoughtFilter, ThoughtForCreate, ThoughtForUpdate,
};
use crate::web::Result;

use super::params::ParamsList;
use super::{ParamsForCreate, ParamsForUpdate, ParamsIded};

pub async fn create_thought(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<ThoughtForCreate>,
) -> Result<Thought> {
    let ParamsForCreate { data } = params;

    let id = ThoughtBmc::create(&ctx, &mm, data).await?;
    let thought = ThoughtBmc::get(&ctx, &mm, id).await?;

    Ok(thought)
}

pub async fn get_thought(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Thought> {
    let ParamsIded { id } = params;

    let thought = ThoughtBmc::get(&ctx, &mm, id).await?;

    Ok(thought)
}

pub async fn list_thoughts(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<ThoughtFilter>,
) -> Result<Vec<Thought>> {
    let thoughts = ThoughtBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

    Ok(thoughts)
}

pub async fn update_thought(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<ThoughtForUpdate>,
) -> Result<Thought> {
    let ParamsForUpdate { id, data } = params;

    ThoughtBmc::update(&ctx, &mm, id, data).await?;

    let thought = ThoughtBmc::get(&ctx, &mm, id).await?;

    Ok(thought)
}

pub async fn delete_thought(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Thought> {
    let ParamsIded { id } = params;

    let thought = ThoughtBmc::get(&ctx, &mm, id).await?;
    ThoughtBmc::delete(&ctx, &mm, id).await?;

    Ok(thought)
}
