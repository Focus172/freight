use serde::{Deserialize, Serialize};

use super::packager::{Packager, PackagerType};

pub mod builder;
pub mod list;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Pkg {
    pub name: String,
    pub packager: Packager,
}

impl Pkg {
    pub fn new(backend: PackagerType, name: &str) -> Self {
        let packager = match backend {
            super::packager::PackagerType::Paru => Packager::paru(),
            super::packager::PackagerType::Brew => Packager::brew(),
        };
        Self {
            name: name.into(),
            packager,
        }
    }

    // pub const fn named(name: &'static str) -> Self {
    //     Pkg {
    //         name,
    //         packager: Packager::guess(),
    //     }
    // }

    pub fn backend(&mut self, backend: PackagerType) {
        match backend {
            super::packager::PackagerType::Paru => {
                self.packager = Packager::paru();
            }
            super::packager::PackagerType::Brew => {
                self.packager = Packager::brew();
            }
        }
    }
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
