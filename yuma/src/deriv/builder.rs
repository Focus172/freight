use std::env;

use super::{Packager, Pkg};

pub trait AsPkgBuild: Sized {
    fn builder(self) -> PkgBuilder;

    fn build(self) -> Option<Pkg> {
        self.builder().build()
    }
}

pub struct PkgBuilder {
    name: String,
    allowed_hostnames: Option<Vec<String>>,
    allowed_arches: Option<Vec<String>>,
    packager: Option<Packager>,
}

impl PkgBuilder {
    fn build(self) -> Option<Pkg> {
        // HACK: error handling here is a real goof
        let hostname = nix::unistd::gethostname().ok()?.into_string().ok()?;
        let arch = env::consts::ARCH.to_string();

        if self
            .allowed_hostnames
            .is_some_and(|hosts| !hosts.contains(&hostname))
        {
            return None;
        }

        if self
            .allowed_arches
            .is_some_and(|arches| !arches.contains(&arch))
        {
            return None;
        }

        let packager = self.packager.unwrap_or_default();

        Some(Pkg {
            name: self.name,
            packager,
        })
    }

    pub fn on_hosts(mut self, hosts: &[&str]) -> Self {
        match self.allowed_hostnames.as_mut() {
            Some(h) => h.extend(hosts.iter().map(ToString::to_string)),
            None => self.allowed_hostnames = Some(hosts.iter().map(ToString::to_string).collect()),
        }
        self
    }

    pub fn on_arches(mut self, arches: &[&str]) -> Self {
        match self.allowed_arches.as_mut() {
            Some(a) => a.extend(arches.iter().map(ToString::to_string)),
            None => self.allowed_hostnames = Some(arches.iter().map(ToString::to_string).collect()),
        }
        self
    }

    pub fn with_packager(mut self, packager: Packager) -> Self {
        self.packager = Some(packager);
        self
    }
}

impl AsPkgBuild for &str {
    fn builder(self) -> PkgBuilder {
        self.into()
    }
}

impl AsPkgBuild for String {
    fn builder(self) -> PkgBuilder {
        self.into()
    }
}

impl AsPkgBuild for PkgBuilder {
    fn builder(self) -> PkgBuilder {
        self
    }
}

impl From<&str> for PkgBuilder {
    fn from(value: &str) -> Self {
        PkgBuilder {
            name: value.to_string(),
            allowed_hostnames: None,
            allowed_arches: None,
            packager: None,
        }
    }
}

impl From<String> for PkgBuilder {
    fn from(value: String) -> Self {
        PkgBuilder {
            name: value,
            allowed_hostnames: None,
            allowed_arches: None,
            packager: None,
        }
    }
}
