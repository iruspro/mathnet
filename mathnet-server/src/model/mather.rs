use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;

use crate::ctx::Ctx;

use super::{
    ModelManager, Result,
    base::{self, DbBmc},
    thought::Thought,
    user::User,
};

// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
// #[sqlx(type_name = "status", rename_all = "lowercase")]
// pub enum Status {
//     Online,
//     Offline,
// }

// region:    --- Mather Types
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Mather {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub user_id: i64,
    // TODO: Add status, with custom sqlx Type.
    // pub status: Status,
}

#[derive(Fields, Deserialize)]
pub struct MatherForCreate {
    pub first_name: String,
    pub last_name: String,
    pub user_id: i64,
}

#[derive(Fields, Deserialize)]
pub struct MatherForUpdate {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
// endregion: --- Mather Types

// region:    --- MatherBmc
pub struct MatherBmc;

impl DbBmc for MatherBmc {
    const TABLE: &'static str = "mather";
}

impl MatherBmc {
    pub async fn create(
        // ctx and mm must be in all of the BMC functions
        ctx: &Ctx,
        mm: &ModelManager,
        mather_c: MatherForCreate,
    ) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, mather_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Mather> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Mather>> {
        base::list::<Self, _>(ctx, mm).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        mather_u: MatherForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, mather_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}

impl MatherBmc {
    pub async fn list_mather_friends(
        ctx: &Ctx,
        mm: &ModelManager,
        mather_id: i64,
    ) -> Result<Vec<Mather>> {
        todo!()
    }

    pub async fn list_mather_thoughts(
        ctx: &Ctx,
        mm: &ModelManager,
        mather_id: i64,
    ) -> Result<Vec<Thought>> {
        todo!()
    }

    // pub async fn list_mather_posts(
    //     ctx: &Ctx,
    //     mm: &ModelManager,
    //     mather_id: i64,
    // ) -> Result<Vec<Post>> {
    //     todo!()
    // }

    // pub async fn list_mather_tasks(
    //     ctx: &Ctx,
    //     mm: &ModelManager,
    //     mather_id: i64,
    // ) -> Result<Vec<Task>> {
    //     todo!()
    // }

    // pub async fn list_mather_chats(
    //     ctx: &Ctx,
    //     mm: &ModelManager,
    //     mather_id: i64,
    // ) -> Result<Vec<Chat>> {
    //     todo!()
    // }

    // pub async fn list_friends_activity(
    //     ctx: &Ctx,
    //     mm: &ModelManager,
    //     mather_id: i64,
    // ) -> Result<Vec<Thought>> {
    //     todo!()
    // }

    pub async fn add_new_friend(
        ctx: &Ctx,
        mm: &ModelManager,
        mather_id: i64,
        friend_id: i64,
    ) -> Result<()> {
        todo!()
    }
}
// endregion: --- MatherBmc
