// region:    --- Modules

use super::{
    ModelManager, Result,
    base::{self, DbBmc},
};
use crate::ctx::Ctx;

use chrono::{DateTime, Utc};
use modql::{
    field::Fields,
    filter::{FilterNodes, ListOptions, OpValsBool, OpValsInt64, OpValsString},
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// endregion: --- Modules

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

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct MessageFilter {
    id: Option<OpValsInt64>,

    chat_id: Option<OpValsInt64>,
    content: Option<OpValsString>,
    mather_id: Option<OpValsInt64>,
    // created_at: Option<_>,
    is_read: Option<OpValsBool>,
    is_edited: Option<OpValsBool>,
    // edited_at: Option<_>,
}
// endregion: --- Message Types

// region:    --- MessageBmc
pub struct MessageBmc;

impl DbBmc for MessageBmc {
    const TABLE: &'static str = "message";
}

impl MessageBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, message_c: MessageForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, message_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Message> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<MessageFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Message>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

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

    // pub async fn list_chat_messages(
    //     ctx: &Ctx,
    //     mm: &ModelManager,
    //     chat_id: i64,
    // ) -> Result<Vec<Message>> {
    //     todo!()
    // }
}
// endregion: --- MessageBmc
