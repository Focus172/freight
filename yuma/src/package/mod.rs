mod brew;
mod paru;

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Packages {
    pub enabled: Vec<Pkg>,
}

#[derive(Serialize, Deserialize)]
pub struct Pkg {
    pub name: String,
    #[serde(skip)]
    _packager: Packager,
}

impl From<&str> for Pkg {
    fn from(value: &str) -> Self {
        Pkg {
            name: value.to_string(),
            _packager: Packager::guess(),
        }
    }
}

pub trait PackageBackend {
    fn list_installed(&self) -> Vec<String>;

    fn list_leaves(&self) -> Vec<String>;

    fn install_packages(&mut self, names: &[&str]);

    fn remove_packages(&mut self, names: &[&str]);
}

pub struct Packager {
    _packager_type: PackagerType,
    backend: Box<dyn PackageBackend>,
}

impl Packager {
    /// Based on the operating system and installed packages make the best guess
    /// for the package backend to use
    #[cfg(target_arch = "x86_64")]
    pub fn guess() -> Self {
        use self::paru::ParuPackager;

        Self {
            _packager_type: PackagerType::Paru,
            backend: Box::new(ParuPackager),
        }
    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn guess() -> Self {
        use self::brew::BrewPackager;

        Self {
            _packager_type: PackagerType::Brew,
            backend: Box::new(BrewPackager),
        }
    }

    pub fn paru() -> Self {
        use self::paru::ParuPackager;

        Self {
            _packager_type: PackagerType::Paru,
            backend: Box::new(ParuPackager),
        }
    }

    pub fn brew() -> Self {
        use self::brew::BrewPackager;

        Self {
            _packager_type: PackagerType::Brew,
            backend: Box::new(BrewPackager),
        }
    }
}

impl Default for Packager {
    fn default() -> Self {
        Self::guess()
    }
}

impl std::ops::Deref for Packager {
    type Target = Box<dyn PackageBackend>;

    fn deref(&self) -> &Self::Target {
        &self.backend
    }
}

impl std::ops::DerefMut for Packager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.backend
    }
}

pub enum PackagerType {
    Paru,
    Brew, //
          // PkgBuild,
          // Justfile,
          // CargoToml,
          // Portage,
          // Frieght,
}
