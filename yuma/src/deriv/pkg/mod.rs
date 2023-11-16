use serde::{Deserialize, Serialize};

use super::packager::{Packager, PackagerType};

pub mod builder;
pub mod list;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Pkgs {
    pub names: Vec<String>,
    pub packager: Packager,
}

impl Pkgs {
    pub fn new<I>(backend: PackagerType, names: I) -> Self
    where
        I: Iterator<Item = String>,
    {
        let packager = match backend {
            super::packager::PackagerType::Paru => Packager::paru(),
            super::packager::PackagerType::Brew => Packager::brew(),
        };
        Self {
            names: names.collect(),
            packager,
        }
    }

    pub fn add<I>(&mut self, names: I)
    where
        I: Iterator<Item = String>,
    {
        self.names.extend(names)
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

impl From<&str> for Pkgs {
    fn from(value: &str) -> Self {
        value.to_owned().into()
    }
}

impl From<String> for Pkgs {
    fn from(value: String) -> Self {
        Pkgs {
            names: vec![value],
            packager: Packager::guess(),
        }
    }
}
