use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::thought::{self, Thought, ThoughtBmc, ThoughtForCreate, ThoughtForUpdate};
use crate::web::Result;

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

pub async fn list_thoughts(ctx: Ctx, mm: ModelManager) -> Result<Vec<Thought>> {
    // TODO: Add filters, limits, and other constraints
    let thoughts = ThoughtBmc::list(&ctx, &mm).await?;

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
