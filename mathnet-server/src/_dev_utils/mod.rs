// region:    --- Modules
mod dev_db;

use tokio::sync::OnceCell;
use tracing::info;

use crate::{
    ctx::Ctx,
    model::{
        self, ModelManager,
        thought::{Thought, ThoughtBmc, ThoughtForCreate},
    },
};
// endregion: --- Modules

/// Initialize environment for local development.
/// (for early development, will be called from main()).
pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");

        dev_db::init_dev_db().await.unwrap();
    })
    .await;
}

/// Initialize test environment.
pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();

    let mm = INIT
        .get_or_init(|| async {
            init_dev().await;
            ModelManager::new().await.unwrap()
        })
        .await;

    mm.clone()
}

pub async fn seed_thoughts(
    ctx: &Ctx,
    mm: &ModelManager,
    contents: &[&str],
) -> model::Result<Vec<Thought>> {
    let mut thoughts = Vec::new();

    for content in contents {
        let id = ThoughtBmc::create(
            ctx,
            mm,
            ThoughtForCreate {
                content: content.to_string(),
                mather_id: 1000,
                on_latex: true,
            },
        )
        .await?;
        let thought = ThoughtBmc::get(ctx, mm, id).await?;

        thoughts.push(thought);
    }

    Ok(thoughts)
}
