mod brew;
mod cargo;
mod fake;
mod paru;

use std::fmt;
use std::sync::Arc;

use crate::prelude::*;

pub use self::brew::BrewPackager;
pub use self::cargo::CargoPackager;
pub use self::fake::FakePackager;
pub use self::paru::ParuPackager;

type ParuRc = Arc<self::ParuPackager>;
type BrewRc = Arc<self::BrewPackager>;
type CargRc = Arc<self::CargoPackager>;
type FakeRc = Arc<self::FakePackager>;

thread_local! {
pub static PARU_PACKAGER: ParuRc = default();
pub static BREW_PACKAGER: BrewRc = default();
pub static CARG_PACKAGER: CargRc = default();
pub static FAKE_PACKAGER: FakeRc = default();
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GenericName(String);
impl GenericName {
    pub fn new(name: String) -> Self {
        GenericName(name)
    }
}

pub type SpecficName = String;

pub trait PackageBackend {
    fn list_installed(&self) -> Vec<String>;

    fn list_leaves(&self) -> Vec<String>;

    fn install(&self, pkgs: Vec<SpecficName>) -> Result<()>;

    fn remove(&self, pkgs: Vec<SpecficName>) -> Result<()>;

    fn resolve_name(&self, name: GenericName) -> SpecficName;
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
            backend: SyncPackagerBackend(PARU_PACKAGER.with(|b| b.clone())),
        }
    }

    pub fn brew() -> Self {
        Self {
            _packager_type: PackagerType::Brew,
            backend: SyncPackagerBackend(BREW_PACKAGER.with(|b| b.clone())),
        }
    }

    pub fn fake() -> Self {
        Self {
            _packager_type: PackagerType::Fake,
            backend: SyncPackagerBackend(FAKE_PACKAGER.with(Clone::clone)),
        }
    }
}

impl Default for Packager {
    fn default() -> Self {
        Self::guess()
    }
}

impl Stub for Packager {
    fn stub() -> Self {
        Packager::fake()
    }
}

impl std::ops::Deref for Packager {
    type Target = dyn PackageBackend;

    fn deref(&self) -> &Self::Target {
        self.backend.0.as_ref()
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PackagerType {
    Paru,
    Brew,
    Fake,
    // PkgBuild,
    // Justfile,
    // CargoToml,
    // Portage,
    // Frieght,
}

impl TryFrom<&str> for PackagerType {
    type Error = resu::eyre::Report;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "Paur" | "paur" => Ok(PackagerType::Paru),
            "Brew" | "brew" => Ok(PackagerType::Brew),
            name => Err(resu::eyre::eyre!("Unkown packager: {}", name)),
        }
    }
}

impl From<PackagerType> for Packager {
    fn from(value: PackagerType) -> Self {
        match value {
            PackagerType::Paru => Packager::paru(),
            PackagerType::Brew => Packager::brew(),
            PackagerType::Fake => Packager::fake(),
        }
    }
}
