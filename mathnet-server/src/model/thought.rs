use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region:    --- Thought Types
// ---->
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Thought {
    pub id: i64,
    pub content: String,
}

// <----
#[derive(Deserialize)]
pub struct ThoughtForCreate {
    pub content: String,
}

// <----
#[derive(Deserialize)]
pub struct ThoughtForUpdate {
    pub content: Option<String>,
}
// endregion: --- Thought Types

// region:    --- ThoughtBmc
pub struct ThoughtBmc;

impl ThoughtBmc {
    pub async fn create(
        // _ctx and mm must be in all of the BMC functions
        _ctx: &Ctx,
        mm: &ModelManager,
        // Function specific arguments
        thought_c: ThoughtForCreate,
    ) -> Result<i64> {
        let db = mm.db();

        let (id,) =
            sqlx::query_as::<_, (i64,)>("INSERT INTO thought (content) values ($1) returning id")
                .bind(thought_c.content)
                .fetch_one(db)
                .await?;

        Ok(id)
    }

    pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Thought> {
        todo!()
    }

    pub async fn list(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Thought>> {
        todo!()
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        todo!()
    }
}
// endregion: --- ThoughtBmc

// region:    --- Tests
#[cfg(test)]
mod tests {
    use crate::_dev_utils;

    use super::*;
    use anyhow::{Ok, Result};
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_content = "test_create_ok content";

        // -- Exec
        let thought_c = ThoughtForCreate {
            content: fx_content.to_string(),
        };
        let id = ThoughtBmc::create(&ctx, &mm, thought_c).await?;

        // -- Check
        let (content,): (String,) = sqlx::query_as("SELECT content from thought where id = $1")
            .bind(id)
            .fetch_one(mm.db())
            .await?;
        assert_eq!(content, fx_content);

        // -- Clean
        let count = sqlx::query("DELETE FROM thought WHERE id = $1")
            .bind(id)
            .execute(mm.db())
            .await?
            .rows_affected();
        assert_eq!(count, 1, "Did not delete 1 row?");

        Ok(())
    }

    #[tokio::test]
    async fn test_create_err() -> Result<()> {
        Ok(())
    }
}
// endregion: --- Tests
