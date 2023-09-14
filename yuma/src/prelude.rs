pub use crate::deriv::{Pkg, PkgBuilder};
pub use crate::error::{YumaError, YumaResult};
pub use crate::YumaCtx;
// pub use crate::service::Service;
//
pub use crate::deriv::builder::AsBuilder;

pub const fn ctx() -> YumaCtx {
    YumaCtx::new()
}

pub fn default<T: Default>() -> T {
    Default::default()
}
