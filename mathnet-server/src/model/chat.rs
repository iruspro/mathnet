use chrono::{DateTime, Utc};
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
// pub enum ChatType {
//     Personal,
//     Group,
// }

// region:    --- Chat Types
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Chat {
    pub id: i64,
    // TODO
    // pub chat_type: ChatType,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub last_message_id: Option<i64>,
}

#[derive(Fields, Deserialize)]
pub struct ChatForCreate {
    pub chat_id: i64,
    pub content: String,
}

#[derive(Fields, Deserialize)]
pub struct ChatForUpdate {
    pub last_message_id: i64,
}
// endregion: --- Chat Types

// region:    --- ChatBmc
pub struct ChatBmc;

impl DbBmc for ChatBmc {
    const TABLE: &'static str = "chat";
}

impl ChatBmc {
    pub async fn create(
        // ctx and mm must be in all of the BMC functions
        ctx: &Ctx,
        mm: &ModelManager,
        chat_c: ChatForCreate,
    ) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, chat_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Chat> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    // pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Message>> {
    //     base::list::<Self, _>(ctx, mm).await
    // }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        chat_u: ChatForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, chat_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}

impl ChatBmc {
    pub async fn list_user_chats(ctx: &Ctx, mm: &ModelManager, chat_id: i64) -> Result<Vec<Chat>> {
        todo!()
    }
}
// endregion: --- ChatBmc
