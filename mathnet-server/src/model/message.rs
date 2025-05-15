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

// region:    --- Message Types
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Message {
    pub id: i64,
    pub chat_id: i64,
    pub content: String,
    pub mather_id: i64,
    pub created_at: DateTime<Utc>,
    pub is_read: bool,
    pub is_edited: bool,
    pub edited_at: Option<DateTime<Utc>>,
}

#[derive(Fields, Deserialize)]
pub struct MessageForCreate {
    pub chat_id: i64,
    pub content: String,
}

#[derive(Fields, Deserialize)]
pub struct MessageForUpdate {
    pub content: Option<String>,
}
// endregion: --- Message Types

// region:    --- MessageBmc
pub struct MessageBmc;

impl DbBmc for MessageBmc {
    const TABLE: &'static str = "message";
}

impl MessageBmc {
    pub async fn create(
        // ctx and mm must be in all of the BMC functions
        ctx: &Ctx,
        mm: &ModelManager,
        message_c: MessageForCreate,
    ) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, message_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Message> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    // pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Message>> {
    //     base::list::<Self, _>(ctx, mm).await
    // }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        message_u: MessageForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, message_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}

impl MessageBmc {
    pub async fn list_chat_messages(
        ctx: &Ctx,
        mm: &ModelManager,
        chat_id: i64,
    ) -> Result<Vec<Message>> {
        todo!()
    }
}
// endregion: --- MessageBmc
