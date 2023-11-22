//! A set of usefule imports for most use cases.

pub use crate::{
    ctx::YumaCtx,
    deriv::{
        packager::Packager,
        pkg::{builder::AsBuilder, Pkgs},
        srv::Service,
    },
    error::{Result, YumaError},
    gaurd,
    log::*,
    y,
};
pub use stub::Stub;

pub use serde::{Deserialize, Serialize};
pub use std::fs;

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
