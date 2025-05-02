use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;

use super::base::{self, DbBmc};

// region:    --- Thought Types
// ---->
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Thought {
    pub id: i64,
    pub content: String,
}

// <----
#[derive(Fields, Deserialize)]
pub struct ThoughtForCreate {
    pub content: String,
}

// <----
#[derive(Fields, Deserialize)]
pub struct ThoughtForUpdate {
    pub content: Option<String>,
}
// endregion: --- Thought Types

// region:    --- ThoughtBmc
pub struct ThoughtBmc;

impl DbBmc for ThoughtBmc {
    const TABLE: &'static str = "thought";
}

impl ThoughtBmc {
    pub async fn create(
        // _ctx and mm must be in all of the BMC functions
        ctx: &Ctx,
        mm: &ModelManager,
        // Function specific arguments
        thought_c: ThoughtForCreate,
    ) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, thought_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Thought> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Thought>> {
        base::list::<Self, _>(ctx, mm).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        thought_u: ThoughtForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, thought_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
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
        let thought = ThoughtBmc::get(&ctx, &mm, id).await?;
        assert_eq!(thought.content, fx_content);

        // -- Clean
        ThoughtBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_err_not_found() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        // -- Exec
        let res = ThoughtBmc::get(&ctx, &mm, fx_id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "thought",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_contents = &["test_list_ok-thought 01", "test_list_ok-thought 02"];
        _dev_utils::seed_thoughts(&ctx, &mm, fx_contents).await?;

        // -- Exec
        let thoughts = ThoughtBmc::list(&ctx, &mm).await?;

        // -- Check
        // TODO: Add filters, limits, and other constraints
        let thoughts: Vec<Thought> = thoughts
            .into_iter()
            .filter(|t| t.content.starts_with("test_list_ok-thought"))
            .collect();
        assert_eq!(thoughts.len(), 2, "number of seeded tasks");

        // -- Clean
        for thought in thoughts.iter() {
            ThoughtBmc::delete(&ctx, &mm, thought.id).await?;
        }

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_content = "test_update ok - thought 01";
        let fx_content_new = "test_update_ok - thought 01 - new";
        let fx_thought = _dev_utils::seed_thoughts(&ctx, &mm, &[fx_content])
            .await?
            .remove(0);

        // -- Exec
        ThoughtBmc::update(
            &ctx,
            &mm,
            fx_thought.id,
            ThoughtForUpdate {
                content: Some(fx_content_new.to_string()),
            },
        )
        .await?;

        // -- Check
        let thought = ThoughtBmc::get(&ctx, &mm, fx_thought.id).await?;
        assert_eq!(thought.content, fx_content_new);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_err_not_found() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        // -- Exec
        let res = ThoughtBmc::delete(&ctx, &mm, fx_id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "thought",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
// endregion: --- Tests
