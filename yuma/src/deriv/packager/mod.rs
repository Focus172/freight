mod brew;
mod cargo;
mod paru;

use std::fmt;
use std::sync::Arc;

use crate::prelude::*;

pub use self::brew::BrewPackager;
pub use self::cargo::CargoPackager;
pub use self::paru::ParuPackager;

type ParuRc = Arc<self::ParuPackager>;
type BrewRc = Arc<self::BrewPackager>;
type CargRc = Arc<self::CargoPackager>;

lazy_static::lazy_static!(
    pub static ref PARU_PACKAGER: ParuRc = Default::default();
    pub static ref BREW_PACKAGER: BrewRc = Default::default();
    pub static ref CARG_PACKAGER: CargRc = Default::default();
);

pub trait PackageBackend {
    fn list_installed(&self) -> Vec<String>;

    fn list_leaves(&self) -> Vec<String>;

    fn install_packages(&self, pkgs: Box<dyn Iterator<Item = Pkg>>);

    fn remove_packages(&self, pkgs: Box<dyn Iterator<Item = Pkg>>);

    fn install(&self, name: &str);
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Packager {
    _packager_type: PackagerType,
    #[serde(skip)]
    backend: SyncPackagerBackend,
}

impl PartialEq for Packager {
    fn eq(&self, other: &Self) -> bool {
        self._packager_type == other._packager_type
    }
}
impl Eq for Packager {}

#[derive(Clone)]
struct SyncPackagerBackend(Arc<dyn PackageBackend>);
impl Default for SyncPackagerBackend {
    fn default() -> Self {
        let Packager { backend, .. } = Packager::guess();
        backend
    }
}

impl fmt::Debug for Packager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Packager")
            .field("backend", &self._packager_type)
            .finish_non_exhaustive()
    }
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
        Self {
            _packager_type: PackagerType::Paru,
            backend: SyncPackagerBackend(PARU_PACKAGER.clone()),
        }
    }

    pub fn brew() -> Self {
        Self {
            _packager_type: PackagerType::Brew,
            backend: SyncPackagerBackend(BREW_PACKAGER.clone()),
        }
    }
}

impl Default for Packager {
    fn default() -> Self {
        Self::guess()
    }
}

impl std::ops::Deref for Packager {
    type Target = dyn PackageBackend;

    fn deref(&self) -> &Self::Target {
        self.backend.0.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PackagerType {
    Paru,
    Brew,
    // PkgBuild,
    // Justfile,
    // CargoToml,
    // Portage,
    // Frieght,
}
