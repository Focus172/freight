use std::env;

use super::{Packager, Pkg};

use crate::prelude::*;

#[derive(Default)]
pub struct PkgBuilder {
    names: Vec<String>,
    allowed_hostnames: Option<Vec<String>>,
    allowed_arches: Option<Vec<String>>,
    allowed_oss: Option<Vec<String>>,
    packager: Option<Packager>,
}

impl PkgBuilder {
    pub(crate) fn build(self) -> Option<Vec<Pkg>> {
        // HACK: error handling here is a real goof
        let hostname = nix::unistd::gethostname().ok()?.into_string().ok()?;
        let arch = env::consts::ARCH.to_string();
        let os = env::consts::OS.to_string();

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

        if self.allowed_oss.is_some_and(|oss| !oss.contains(&os)) {
            return None;
        }

        let packager = self.packager.unwrap_or_default();

        Some(
            self.names
                .iter()
                .map(|name| Pkg {
                    name: name.clone(),
                    packager: packager.clone(),
                })
                .collect(),
        )
    }

    /// Configures the package to only be built in if the current os matches
    /// the given name. Although you can put any text in this function it will
    /// only have an affect if you use one of the following:
    ///   linux, macos, ios, freebsd, dragonfly, netbsd, openbsd, solaris,
    ///   android, windows
    pub fn on_os(mut self, os: impl Into<String>) -> Self {
        match self.allowed_oss.as_mut() {
            Some(h) => h.push(os.into()),
            None => self.allowed_oss = Some(vec![os.into()]),
        }
        self
    }

    pub fn on_host(mut self, host: impl Into<String>) -> Self {
        match self.allowed_hostnames.as_mut() {
            Some(h) => h.push(host.into()),
            None => self.allowed_hostnames = Some(vec![host.into()]),
        }
        self
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

pub trait AsBuilder: Into<PkgBuilder> {
    fn b(self) -> PkgBuilder {
        self.into()
    }
}

impl AsBuilder for &str {}
impl AsBuilder for String {}
impl AsBuilder for PkgBuilder {}
// impl AsBuilder for &dyn AsBuilder {}

impl<const N: usize> AsBuilder for [&str; N] {}
impl<const N: usize> AsBuilder for [String; N] {}
impl<const N: usize> AsBuilder for [PkgBuilder; N] {}
// impl<const N: usize> AsBuilder for [&dyn AsBuilder; N] {}

impl AsBuilder for &[String] {}
impl AsBuilder for &[&str] {}
impl AsBuilder for &[PkgBuilder] {}
// impl AsBuilder for &[&dyn AsBuilder] {}

impl From<&str> for PkgBuilder {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for PkgBuilder {
    fn from(value: String) -> Self {
        PkgBuilder {
            names: vec![value],
            ..default()
        }
    }
}

impl<const N: usize> From<[&str; N]> for PkgBuilder {
    fn from(value: [&str; N]) -> Self {
        // PERF: Even though we own the data there is nothing to be gained from
        // taking it as we need to clone it to make it as string
        value.as_slice().into()
    }
}

impl<const N: usize> From<[String; N]> for PkgBuilder {
    fn from(value: [String; N]) -> Self {
        // PERF: we own the data so we dont need to copy
        PkgBuilder {
            names: Vec::from(value),
            ..default()
        }
    }
}

impl<const N: usize> From<[PkgBuilder; N]> for PkgBuilder {
    fn from(value: [PkgBuilder; N]) -> Self {
        let mut names = Vec::new();
        let mut allowed_hostnames = None;
        let mut allowed_arches = None;
        let mut allowed_oss = None;
        let mut packager = None;
        for pkg in value {
            names.extend(pkg.names);

            if let Some(hosts) = pkg.allowed_hostnames {
                allowed_hostnames.get_or_insert(Vec::new()).extend(hosts);
            }

            if let Some(arches) = pkg.allowed_arches {
                allowed_arches.get_or_insert(Vec::new()).extend(arches);
            }

            if let Some(oss) = pkg.allowed_oss {
                allowed_oss.get_or_insert(Vec::new()).extend(oss);
            }

            if let Some(pkgr) = pkg.packager {
                packager.get_or_insert(pkgr);
            }
        }

        PkgBuilder {
            names,
            allowed_hostnames,
            allowed_arches,
            allowed_oss,
            packager,
        }
    }
}

impl From<&[PkgBuilder]> for PkgBuilder {
    fn from(value: &[PkgBuilder]) -> Self {
        (*value).into()
    }
}

impl From<&[&str]> for PkgBuilder {
    fn from(value: &[&str]) -> Self {
        PkgBuilder {
            names: value.iter().map(|s| s.to_string()).collect(),
            ..default()
        }
    }
}

impl From<&[String]> for PkgBuilder {
    fn from(value: &[String]) -> Self {
        PkgBuilder {
            names: value.to_vec(),
            ..default()
        }
    }
}

// ----------------------------------------------------------------------------
// ============================================================================
// ----------------------------------------------------------------------------
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>> DONT LOOK ANY MORE <<<<<<<<<<<<<<<<<<<<<<<<<<
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> THIS IS A FAVOR <<<<<<<<<<<<<<<<<<<<<<<<<<<
// ----------------------------------------------------------------------------
// ============================================================================
// ----------------------------------------------------------------------------

/*
impl AsPkgBuild for (&str, &str) {
    fn builder(self) -> PkgBuilder {
        PkgBuilder {
            names: vec![self.0.to_string(), self.1.to_string()],
            allowed_hostnames: None,
            allowed_arches: None,
            packager: None,
        }
    }
}

impl AsPkgBuild for (&str, &str, &str) {
    fn builder(self) -> PkgBuilder {
        PkgBuilder {
            names: vec![self.0.to_string(), self.1.to_string(), self.2.to_string()],
            allowed_hostnames: None,
            allowed_arches: None,
            packager: None,
        }
    }
}

impl AsPkgBuild for (&str, &str, &str, &str) {
    fn builder(self) -> PkgBuilder {
        PkgBuilder {
            names: vec![
                self.0.to_string(),
                self.1.to_string(),
                self.2.to_string(),
                self.3.to_string(),
            ],
            allowed_hostnames: None,
            allowed_arches: None,
            packager: None,
        }
    }
}

impl AsPkgBuild for (&str, &str, &str, &str, &str) {
    fn builder(self) -> PkgBuilder {
        PkgBuilder {
            names: vec![
                self.0.to_string(),
                self.1.to_string(),
                self.2.to_string(),
                self.3.to_string(),
                self.4.to_string(),
            ],
            allowed_hostnames: None,
            allowed_arches: None,
            packager: None,
        }
    }
}

impl AsPkgBuild for (&str, &str, &str, &str, &str, &str) {
    fn builder(self) -> PkgBuilder {
        PkgBuilder {
            names: vec![
                self.0.to_string(),
                self.1.to_string(),
                self.2.to_string(),
                self.3.to_string(),
                self.4.to_string(),
                self.5.to_string(),
            ],
            allowed_hostnames: None,
            allowed_arches: None,
            packager: None,
        }
    }
}

impl AsPkgBuild for (&str, &str, &str, &str, &str, &str, &str) {
    fn builder(self) -> PkgBuilder {
        PkgBuilder {
            names: vec![
                self.0.to_string(),
                self.1.to_string(),
                self.2.to_string(),
                self.3.to_string(),
                self.4.to_string(),
                self.5.to_string(),
                self.6.to_string(),
            ],
            allowed_hostnames: None,
            allowed_arches: None,
            packager: None,
        }
    }
}

impl AsPkgBuild for (&str, &str, &str, &str, &str, &str, &str, &str) {
    fn builder(self) -> PkgBuilder {
        PkgBuilder {
            names: vec![
                self.0.to_string(),
                self.1.to_string(),
                self.2.to_string(),
                self.3.to_string(),
                self.4.to_string(),
                self.5.to_string(),
                self.6.to_string(),
                self.7.to_string(),
            ],
            allowed_hostnames: None,
            allowed_arches: None,
            packager: None,
        }
    }
}
*/
