use crate::prelude::*;

pub use super::PackageBackend;

#[derive(Debug, Default)]
pub struct CargoPackager;

impl PackageBackend for CargoPackager {
    fn list_installed(&self) -> Vec<String> {
        todo!()
    }

    fn list_leaves(&self) -> Vec<String> {
        todo!()
    }

    fn install(&self, _pkgs: Vec<super::SpecficName>) -> Result<()> {
        todo!()
    }

    fn remove_packages(&self, _pkgs: Vec<super::SpecficName>) {
        todo!()
    }

    fn resolve_name(&self, _name: super::GenericName) -> super::SpecficName {
        todo!()
    }
}
