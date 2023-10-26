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

    fn install_packages(&self, pkgs: Box<dyn Iterator<Item = crate::prelude::Pkg>>) {
        // TODO: convert package name to packager specific name.
        let _names: Vec<String> = pkgs.map(|p| p.name).collect();
        todo!()
    }

    fn remove_packages(&self, pkgs: Box<dyn Iterator<Item = crate::prelude::Pkg>>) {
        // TODO: convert package name to packager specific name.
        let _names: Vec<String> = pkgs.map(|p| p.name).collect();
        todo!()
    }

    fn install(&self, name: &str) {
        todo!()
    }
}
