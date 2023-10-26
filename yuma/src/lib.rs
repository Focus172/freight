#![feature(trait_alias)]
#![deny(
    // missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unused_imports,
    dead_code,
    unused_crate_dependencies
)]
// #![feature(never_type)]
#![forbid(unsafe_code)]

pub mod callbacks;
pub mod ctx;
pub mod deriv;
pub mod error;
mod macros;
pub mod prelude;

pub extern crate color_eyre as petty;

// re-export of inline documentation functions
pub use yumadoc::docu;

use crate::prelude::*;

pub fn init_logger() -> Result<()> {
    init_logger_with_level(log::LevelFilter::Debug)
}

pub fn init_logger_with_level(level: log::LevelFilter) -> Result<()> {
    simplelog::TermLogger::init(
        level,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )?;
    Ok(())
}
