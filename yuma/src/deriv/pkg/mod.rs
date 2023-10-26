use serde::{Deserialize, Serialize};

use super::packager::Packager;

pub mod builder;
pub mod list;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pkg {
    pub name: String,
    pub packager: Packager,
}

impl From<&str> for Pkg {
    fn from(value: &str) -> Self {
        value.to_owned().into()
    }
}

impl From<String> for Pkg {
    fn from(value: String) -> Self {
        Pkg {
            name: value,
            packager: Packager::guess(),
        }
    }
}
