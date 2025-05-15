use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;

use crate::ctx::Ctx;

use super::{
    ModelManager, Result,
    base::{self, DbBmc},
    user_group::UserGroup,
};

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

    pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<User>> {
        base::list::<Self, _>(ctx, mm).await
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
}

impl UserBmc {
    pub async fn list_user_groups(
        ctx: &Ctx,
        mm: &ModelManager,
        user_id: i64,
    ) -> Result<Vec<UserGroup>> {
        todo!()
    }

    pub async fn deactivate(ctx: &Ctx, mm: &ModelManager, user_id: i64) -> Result<()> {
        todo!()
    }
}
// endregion: --- UserBmc
