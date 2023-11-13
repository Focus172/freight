//! A set of usefule imports for most use cases.

pub use crate::ctx::YumaCtx;
pub use crate::deriv::packager::Packager;
pub use crate::deriv::pkg::builder::AsBuilder;
pub use crate::deriv::pkg::builder::PkgBuilder;
pub use crate::deriv::pkg::Pkg;
pub use crate::deriv::srv::Service;
pub use crate::error::{Result, YumaError};
pub use crate::y;

pub use serde::{Deserialize, Serialize};

/// Recomended way to construct a CTX as it should be stable across other some
/// basic breaking changes to the ctx constructor api.
///
/// In the the future will make good guesses about your use case to add so
/// goodness to your process.
pub fn ctx() -> YumaCtx {
    resu::install().expect("Don't try to create a context twice");
    YumaCtx::new()
}

/// An alias to make large structs easier to declare:
/// ```rust
/// # use yuma::prelude::*;
/// # #[allow(unused)]
/// let ssh = Service {
///     name: "sshd".into(),
///     ..default()
/// };
/// ```
pub fn default<T: Default>() -> T {
    T::default()
}
