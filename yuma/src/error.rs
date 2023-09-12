pub type YumaResult = std::result::Result<(), YumaError>;

use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum YumaError {
    #[error("Io failure: {0}")]
    Io(#[from] io::Error),
    #[error("Packages specified that could not be resolved: {name:?}")]
    InvalidPackage { name: String },
    #[error(transparent)]
    Static(anyhow::Error),
    #[error("Unknown error")]
    Unknown,
}
