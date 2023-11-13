#![deny(
    // missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unused_crate_dependencies
)]
#![feature(
    trait_alias,
    // adt_const_params,
    never_type
)]
#![forbid(unsafe_code)]

pub mod callbacks;
pub mod ctx;
pub mod deriv;
pub mod error;
pub mod log;
mod macros;
pub mod prelude;

pub extern crate color_eyre as resu;

// re-export of inline documentation functions
pub use yumadoc::docu;
