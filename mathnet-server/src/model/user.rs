// region:    --- Modules

use super::{
    ModelManager, Result,
    base::{self, DbBmc},
};
use crate::ctx::Ctx;

use modql::{
    field::Fields,
    filter::{FilterNodes, ListOptions, OpValsBool, OpValsInt64, OpValsString},
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// endregion: --- Modules

// region:    --- User Types
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub is_active: bool,
    pub is_staff: bool,
}

#[derive(Fields, Deserialize)]
pub struct UserForCreate {
    pub email: String,
    // hash
    pub password: String,
}

#[derive(Fields, Deserialize)]
pub struct UserForUpdate {
    pub email: Option<String>,
    // hash
    pub password: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct UserFilter {
    id: Option<OpValsInt64>,

    email: Option<OpValsString>,
    is_active: Option<OpValsBool>,
    is_staff: Option<OpValsBool>,
}
// endregion: --- User Types

// region:    --- UserBmc
pub struct UserBmc;

impl DbBmc for UserBmc {
    const TABLE: &'static str = "user";
}

impl UserBmc {
    pub async fn create(
        // ctx and mm must be in all of the BMC functions
        ctx: &Ctx,
        mm: &ModelManager,
        user_c: UserForCreate,
    ) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, user_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<User> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<UserFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<User>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        user_u: UserForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, user_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }

    // TODO: Implement this
    // pub async fn list_user_groups(
    //     ctx: &Ctx,
    //     mm: &ModelManager,
    //     user_id: i64,
    // ) -> Result<Vec<UserGroup>> {
    //     todo!()
    // }

    // pub async fn deactivate(ctx: &Ctx, mm: &ModelManager, user_id: i64) -> Result<()> {
    //     todo!()
    // }
}
// endregion: --- UserBmc
