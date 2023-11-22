use serde::{Deserialize, Serialize};
use stub::Stub;

use super::packager::{Packager, PackagerType, SpecficName};

pub mod builder;
pub mod list;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Stub)]
pub struct Pkgs {
    pub names: Vec<SpecficName>,
    pub packager: Packager,
}

impl Pkgs {
    pub fn new<I>(backend: PackagerType, names: I) -> Self
    where
        I: Iterator<Item = String>,
    {
        let packager = backend.into();

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
        self.packager = backend.into();
    }

    pub fn from_names<const N: usize>(names: [&str; N]) -> Self {
        Self {
            names: names.into_iter().map(ToOwned::to_owned).collect(),
            packager: Packager::guess(),
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
