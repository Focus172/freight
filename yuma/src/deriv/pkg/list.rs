use requestty::Question;
use std::collections::HashSet;

use color_eyre::eyre::ensure;
use serde::{Deserialize, Serialize};
use stub::Stub;

use crate::deriv::packager::SpecficName;
use crate::prelude::*;

use super::{
    builder::{AsPkgBuilderList, PkgBuilder},
    Pkgs,
};

#[derive(Debug, Default, Serialize, Deserialize, Stub)]
pub(crate) struct PackagerDerivation {
    pkgr: Packager,
    enabled: Vec<SpecficName>,
    prunable: HashSet<SpecficName>,
}

impl PackagerDerivation {
    pub fn new(pkgs: Pkgs) -> Self {
        let prunable = pkgs.packager.list_leaves().into_iter().collect();
        Self {
            pkgr: pkgs.packager,
            enabled: pkgs.names,
            prunable,
        }
    }

    pub fn add(&mut self, pkgs: Pkgs) -> Result<()> {
        ensure!(
            pkgs.packager == self.pkgr,
            "Packager does not match this pakager."
        );

        self.enabled.extend(pkgs.names);
        Ok(())
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Stub)]
pub struct Packages {
    backends: Vec<PackagerDerivation>,
}

impl Packages {
    pub(crate) fn add(&mut self, pkgsets: Vec<Pkgs>) {
        // find a thing that uses the same packager
        for pkgs in pkgsets {
            if let Some(d) = self
                .backends
                .iter_mut()
                .find(|deriv| deriv.pkgr == pkgs.packager)
            {
                d.add(pkgs).unwrap();
            } else {
                self.backends.push(PackagerDerivation::new(pkgs));
            }
        }
    }

    pub(crate) fn install(&mut self) -> Result<()> {
        for deriv in self.backends.iter_mut() {
            let mut enabled: Vec<SpecficName> = deriv.enabled.drain(..).collect();

            let installed = deriv.pkgr.list_installed();
            // only install packages that are not already installed
            enabled.retain(|name| !installed.contains(name));

            let q = Question::confirm("install")
                .message(format!("Install {:?} ?", &enabled))
                .default(false)
                .build();

            let requestty::Answer::Bool(yes) = requestty::prompt_one(q)? else {
                unreachable!()
            };

            if yes {
                // remove packages about to be installed from prunable list
                for name in enabled.iter() {
                    deriv.prunable.remove(name);
                }

                deriv.pkgr.install(enabled)?;
            }
        }

        Ok(())
    }

    pub(crate) fn prune(&mut self) -> Result<()> {
        for deriv in self.backends.drain(..) {
            let q = Question::confirm("remove")
                .message(format!("Remove {:?} ?", deriv.prunable))
                .default(false)
                .build();

            let requestty::Answer::Bool(yes) = requestty::prompt_one(q)? else {
                unreachable!()
            };

            if yes {
                deriv
                    .pkgr
                    .remove(deriv.prunable.into_iter().collect())
                    .unwrap();
            }
        }
        Ok(())
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

impl AsPkgList for Vec<Pkgs> {
    fn list(self) -> Vec<Pkgs> {
        self
    }
}

impl AsPkgList for Pkgs {
    fn list(self) -> Vec<Pkgs> {
        vec![self]
    }
}
