pub mod builder;
pub mod list;
mod packager;
mod pkgs;
mod serv;

use std::rc::Rc;

use serde::{Deserialize, Serialize};

pub use self::builder::PkgBuilder;
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

    pub fn add(&mut self, pkgs: impl Iterator<Item = PkgBuilder>) {
        self.enabled.extend(pkgs.flat_map(|p| p.build()).flatten())
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

#[derive(Clone)]
pub struct Packager {
    _packager_type: PackagerType,
    backend: Rc<dyn PackageBackend>,
}

impl Packager {
    /// Based on the operating system and installed packages make the best guess
    /// for the package backend to use
    pub fn guess() -> Self {
        // HACK: this gets it working for just my system but this should be a smarter system
        #[cfg(target_arch = "x86_64")]
        return Self::paru();

        #[cfg(not(target_arch = "x86_64"))]
        return Self::brew();
    }

    pub fn paru() -> Self {
        use self::packager::ParuPackager;

        Self {
            _packager_type: PackagerType::Paru,
            backend: Rc::new(ParuPackager),
        }
    }

    pub fn brew() -> Self {
        use self::packager::BrewPackager;

        Self {
            _packager_type: PackagerType::Brew,
            backend: Rc::new(BrewPackager),
        }
    }
}

impl Default for Packager {
    fn default() -> Self {
        Self::guess()
    }
}

impl std::ops::Deref for Packager {
    type Target = Rc<dyn PackageBackend>;

    fn deref(&self) -> &Self::Target {
        &self.backend
    }
}

impl std::ops::DerefMut for Packager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.backend
    }
}

#[derive(Clone)]
pub enum PackagerType {
    Paru,
    Brew, //
          // PkgBuild,
          // Justfile,
          // CargoToml,
          // Portage,
          // Frieght,
}
