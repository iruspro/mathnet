// region:    --- Modules

use super::{
    ModelManager, Result,
    base::{self, DbBmc},
};
use crate::ctx::Ctx;

use chrono::{DateTime, Utc};
use modql::{
    field::Fields,
    filter::{FilterNodes, ListOptions, OpValsInt64},
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// endregion: --- Modules

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
    // pub chat_type: ChatType,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub last_message_id: Option<i64>,
}

#[derive(Fields, Deserialize)]
pub struct ChatForCreate {}

#[derive(Fields, Deserialize)]
pub struct ChatForUpdate {
    pub last_message_id: i64,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ChatFilter {
    id: Option<OpValsInt64>,

    // created_at: Option<_>,
    // updated_at: Option<_>,
    last_message_id: Option<OpValsInt64>,
    // status: Option<_>,
}
// endregion: --- Chat Types

// region:    --- ChatBmc
pub struct ChatBmc;

impl DbBmc for ChatBmc {
    const TABLE: &'static str = "chat";
}

impl ChatBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, chat_c: ChatForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, chat_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Chat> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ChatFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Chat>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

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

    // pub async fn list_user_chats(ctx: &Ctx, mm: &ModelManager, chat_id: i64) -> Result<Vec<Chat>> {
    //     todo!()
    // }
}
// endregion: --- ChatBmc
