pub mod builder;
mod packager;
mod pkgs;
mod serv;

use serde::{Deserialize, Serialize};

use self::packager::PackageBackend;

#[derive(Default, Serialize, Deserialize)]
pub struct Packages {
    pub enabled: Vec<Pkg>,
}

impl Packages {
    pub const fn new() -> Self {
        Self {
            enabled: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Pkg {
    pub name: String,
    #[serde(skip)]
    pub packager: Packager,
}

impl From<&str> for Pkg {
    fn from(value: &str) -> Self {
        Pkg {
            name: value.to_string(),
            packager: Packager::guess(),
        }
    }
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
        Self::paru()
    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn guess() -> Self {
        Self::brew()
    }

    pub fn paru() -> Self {
        use self::packager::ParuPackager;

        Self {
            _packager_type: PackagerType::Paru,
            backend: Box::new(ParuPackager),
        }
    }

    pub fn brew() -> Self {
        use self::packager::BrewPackager;

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
