use super::PackageBackend;
use crate::prelude::*;

#[derive(Default, Debug)]
pub struct FakePackager;

impl PackageBackend for FakePackager {
    fn list_installed(&self) -> Vec<String> {
        vec![]
    }

    fn list_leaves(&self) -> Vec<String> {
        vec![]
    }

    fn install(&self, pkgs: Vec<super::SpecficName>) -> Result<()> {
        crate::log::info!("Would have installed: {:?}", pkgs);
        Ok(())
    }

    fn remove(&self, pkgs: Vec<super::SpecficName>) -> Result<()> {
        crate::log::info!("Would have removed: {:?}", pkgs);
        Ok(())
    }

    fn resolve_name(&self, name: super::GenericName) -> super::SpecficName {
        name.0
    }
}
