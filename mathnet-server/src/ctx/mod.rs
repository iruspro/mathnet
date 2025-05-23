// region:    --- Modules
mod error;

pub use self::error::{Error, Result};
// endregion: --- Modules

#[derive(Clone, Debug)]
pub struct Ctx {
    mather_id: i64,
}

// Constructor.
impl Ctx {
    pub fn root_ctx() -> Self {
        Ctx { mather_id: 0 }
    }

    pub fn new(mather_id: i64) -> Result<Self> {
        if mather_id == 0 {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self { mather_id })
        }
    }
}

// Property Accessors.
impl Ctx {
    pub fn mather_id(&self) -> i64 {
        self.mather_id
    }
}
