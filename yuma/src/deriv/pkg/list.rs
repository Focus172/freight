use serde::{Deserialize, Serialize};

use super::{
    builder::{AsPkgBuilderList, PkgBuilder},
    Pkgs,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Packages {
    pub enabled: Vec<Pkgs>,
}

impl Packages {
    pub const fn new() -> Self {
        Self {
            enabled: Vec::new(),
        }
    }

    pub fn add<PkgList>(&mut self, pkgs: PkgList)
    where
        PkgList: IntoIterator<Item = Pkgs>,
    {
        self.enabled.extend(pkgs);
    }
}

pub trait AsPkgList {
    fn list(self) -> Vec<Pkgs>;
}

impl<B: AsPkgBuilderList> AsPkgList for B {
    fn list(self) -> Vec<Pkgs> {
        self.list()
            .into_iter()
            .flat_map(PkgBuilder::build)
            .collect()
    }
}
