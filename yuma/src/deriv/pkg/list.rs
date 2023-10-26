use serde::{Deserialize, Serialize};

use super::{builder::PkgBuilder, Pkg};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Packages {
    pub enabled: Vec<Pkg>,
}

impl Packages {
    pub const fn new() -> Self {
        Self {
            enabled: Vec::new(),
        }
    }

    pub fn add(&mut self, pkgs: impl Iterator<Item = PkgBuilder>) {
        self.enabled.extend(pkgs.flat_map(|p| p.build()).flatten())
    }
}
