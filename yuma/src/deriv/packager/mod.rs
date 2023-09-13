mod brew;
mod paru;

pub use self::brew::BrewPackager;
pub use self::paru::ParuPackager;

pub trait PackageBackend {
    fn list_installed(&self) -> Vec<String>;

    fn list_leaves(&self) -> Vec<String>;

    fn install_packages(&mut self, names: &[&str]);

    fn remove_packages(&mut self, names: &[&str]);

    fn install(&mut self, name: &str);
}
