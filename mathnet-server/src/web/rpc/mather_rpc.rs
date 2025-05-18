// region:    --- Modules

use super::Result;
use super::{ParamsForCreate, ParamsForUpdate, ParamsIded, params::ParamsList};
use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::mather::{Mather, MatherBmc, MatherFilter, MatherForCreate, MatherForUpdate};

// endregion: --- Modules

pub async fn create_mather(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<MatherForCreate>,
) -> Result<Mather> {
    let ParamsForCreate { data } = params;

    let id = MatherBmc::create(&ctx, &mm, data).await?;
    let mather = MatherBmc::get(&ctx, &mm, id).await?;

    Ok(mather)
}

pub async fn get_mather(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Mather> {
    let ParamsIded { id } = params;

    let mather = MatherBmc::get(&ctx, &mm, id).await?;

    Ok(mather)
}

pub async fn list_mathers(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<MatherFilter>,
) -> Result<Vec<Mather>> {
    let mathers = MatherBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

    Ok(mathers)
}

pub async fn update_mather(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<MatherForUpdate>,
) -> Result<Mather> {
    let ParamsForUpdate { id, data } = params;

    MatherBmc::update(&ctx, &mm, id, data).await?;

    let mather = MatherBmc::get(&ctx, &mm, id).await?;

    Ok(mather)
}

pub async fn delete_mather(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Mather> {
    let ParamsIded { id } = params;

    let mather = MatherBmc::get(&ctx, &mm, id).await?;
    MatherBmc::delete(&ctx, &mm, id).await?;

    Ok(mather)
}
