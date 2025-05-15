use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;

use crate::ctx::Ctx;

use super::{
    ModelManager, Result,
    base::{self, DbBmc},
    user::User,
};

// region:    --- UserGroup Types
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct UserGroup {
    pub id: i64,
    pub name: String,
}

#[derive(Fields, Deserialize)]
pub struct UserGroupForCreate {
    pub name: String,
}

#[derive(Fields, Deserialize)]
pub struct UserGroupForUpdate {
    pub name: Option<String>,
}
// endregion: --- UserGroup Types

// region:    --- UserGroupBmc
pub struct UserGroupBmc;

impl DbBmc for UserGroupBmc {
    const TABLE: &'static str = "user_group";
}

impl UserGroupBmc {
    pub async fn create(
        // ctx and mm must be in all of the BMC functions
        ctx: &Ctx,
        mm: &ModelManager,
        user_group_c: UserGroupForCreate,
    ) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, user_group_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<UserGroup> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<UserGroup>> {
        base::list::<Self, _>(ctx, mm).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        user_group_u: UserGroupForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, user_group_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}

impl UserGroupBmc {
    pub async fn list_users(ctx: &Ctx, mm: &ModelManager, group_id: i64) -> Result<Vec<User>> {
        todo!()
    }

    pub async fn add_user(ctx: &Ctx, mm: &ModelManager, group_id: i64, user_id: i64) -> Result<()> {
        todo!()
    }

    pub async fn add_users(
        ctx: &Ctx,
        mm: &ModelManager,
        group_id: i64,
        user_ids: Vec<i64>,
    ) -> Result<()> {
        todo!()
    }
}
// endregion: --- UserGroupBmc
