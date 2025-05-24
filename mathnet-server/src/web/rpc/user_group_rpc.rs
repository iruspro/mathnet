use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::user_group::{
    UserGroup, UserGroupBmc, UserGroupFilter, UserGroupForCreate, UserGroupForUpdate,
};
use crate::web::Result;

use super::params::ParamsList;
use super::{ParamsForCreate, ParamsForUpdate, ParamsIded};

pub async fn create_user_group(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<UserGroupForCreate>,
) -> Result<UserGroup> {
    let ParamsForCreate { data } = params;

    let id = UserGroupBmc::create(&ctx, &mm, data).await?;
    let user_group = UserGroupBmc::get(&ctx, &mm, id).await?;

    Ok(user_group)
}

pub async fn get_user_group(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<UserGroup> {
    let ParamsIded { id } = params;

    let user_group = UserGroupBmc::get(&ctx, &mm, id).await?;

    Ok(user_group)
}

pub async fn list_user_groups(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<UserGroupFilter>,
) -> Result<Vec<UserGroup>> {
    let user_groups = UserGroupBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

    Ok(user_groups)
}

pub async fn update_user_group(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<UserGroupForUpdate>,
) -> Result<UserGroup> {
    let ParamsForUpdate { id, data } = params;

    UserGroupBmc::update(&ctx, &mm, id, data).await?;

    let user_group = UserGroupBmc::get(&ctx, &mm, id).await?;

    Ok(user_group)
}

pub async fn delete_user_group(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded,
) -> Result<UserGroup> {
    let ParamsIded { id } = params;

    let user_group = UserGroupBmc::get(&ctx, &mm, id).await?;
    UserGroupBmc::delete(&ctx, &mm, id).await?;

    Ok(user_group)
}
