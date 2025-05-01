use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result};
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
        let db = mm.db();

        let thought: Thought = sqlx::query_as("SELECT * FROM thought WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            // Will fail if there is a problem with the SELECT statement
            // or database
            .await?
            // By convention, `get` must always return a value
            // otherwise, it should return an EntityNotFound error
            .ok_or(Error::EntityNotFound {
                entity: "thought",
                id,
            })?;

        Ok(thought)
    }

    pub async fn list(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Thought>> {
        // TODO: Add filters, limits, and other constraints
        let db = mm.db();

        let thoughts: Vec<Thought> = sqlx::query_as("SELECT * FROM thought ORDER BY id")
            .fetch_all(db)
            .await?;

        Ok(thoughts)
    }

    //TODO: update

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        let db = mm.db();

        let count = sqlx::query("DELETE FROM thought where id = $1")
            .bind(id)
            .execute(db)
            .await?
            .rows_affected();

        // By convention, `delete` must always remove something
        // otherwise, it should return an EntityNotFound error
        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "thought",
                id: id,
            });
        }

        Ok(())
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
