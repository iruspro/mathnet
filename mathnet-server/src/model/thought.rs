// region:    --- Modules

use super::base::{self, DbBmc};
use crate::ctx::Ctx;
use crate::model::{ModelManager, Result};

use chrono::{DateTime, Utc};
use modql::{
    field::Fields,
    filter::{FilterNodes, ListOptions, OpValsBool, OpValsInt64, OpValsString},
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// endregion: --- Modules

// region:    --- Thought Types
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Thought {
    pub id: i64,
    pub content: String,
    pub mather_id: i64,
    pub created_at: DateTime<Utc>,
    pub on_latex: bool,
}

#[derive(Deserialize, Fields)]
pub struct ThoughtForCreate {
    pub content: String,
    pub mather_id: i64,
    pub on_latex: bool,
}

#[derive(Deserialize, Fields)]
pub struct ThoughtForUpdate {
    pub content: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ThoughtFilter {
    id: Option<OpValsInt64>,

    content: Option<OpValsString>,
    mather_id: Option<OpValsInt64>,
    // created_at: Option<OpValsDateTime<Utc>>,
    on_latex: Option<OpValsBool>,
}
// endregion: --- Thought Types

// region:    --- ThoughtBmc
pub struct ThoughtBmc;

impl DbBmc for ThoughtBmc {
    const TABLE: &'static str = "thought";
}

impl ThoughtBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, thought_c: ThoughtForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, thought_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Thought> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ThoughtFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Thought>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
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

    // TODO: Implement this
    // pub async fn list_thought_reactions(
    //     ctx: &Ctx,
    //     mm: &ModelManager,
    //     mather_id: i64,
    // ) -> Result<Vec<Reaction>> {
    //     todo!()
    // }
}
// endregion: --- ThoughtBmc

// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use crate::model::Error;
    use anyhow::{Ok, Result};
    use serde_json::json;
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
            mather_id: 1000,
            on_latex: true,
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
    async fn test_list_all_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_contents = &["test_list_all_ok-thought 01", "test_list_all_ok-thought 02"];
        _dev_utils::seed_thoughts(&ctx, &mm, fx_contents).await?;

        // -- Exec
        let thoughts = ThoughtBmc::list(&ctx, &mm, None, None).await?;

        // -- Check
        let thoughts: Vec<Thought> = thoughts
            .into_iter()
            .filter(|t| t.content.starts_with("test_list_all_ok-thought"))
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
    async fn test_list_by_filter_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_contents = &[
            "test_list_by_filter_ok-thought 01.a",
            "test_list_by_filter_ok-thought 01.b",
            "test_list_by_filter_ok-thought 02.a",
            "test_list_by_filter_ok-thought 02.b",
            "test_list_by_filter_ok-thought 03",
        ];
        _dev_utils::seed_thoughts(&ctx, &mm, fx_contents).await?;

        // -- Exec
        let filters: Vec<ThoughtFilter> = serde_json::from_value(json!([
            {
                "content": {
                    "$endsWith": ".a",
                    "$containsAny": ["01", "02"],
                },

            },
            {
                "content": {
                    "$contains": "03"
                }
            }
        ]))?;

        let list_options = serde_json::from_value(json!({
            "order_bys": "!id"
        }))?;

        let thoughts = ThoughtBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;

        // -- Check
        assert_eq!(thoughts.len(), 3);
        assert!(thoughts[0].content.ends_with("03"));
        assert!(thoughts[1].content.ends_with("02.a"));
        assert!(thoughts[2].content.ends_with("01.a"));

        // -- Clean
        let thoughts = ThoughtBmc::list(
            &ctx,
            &mm,
            Some(serde_json::from_value(json!([{
                "content": {"$startsWith": "test_list_by_filter_ok"}
            }]))?),
            None,
        )
        .await?;
        assert_eq!(thoughts.len(), 5);
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
