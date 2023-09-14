use std::env;

use super::{Packager, Pkg};

pub trait AsPkgBuild: Sized {
    fn builder(self) -> PkgBuilder;

    fn build(self) -> Option<Vec<Pkg>> {
        self.builder().build()
    }
}

pub struct PkgBuilder {
    names: Vec<String>,
    allowed_hostnames: Option<Vec<String>>,
    allowed_arches: Option<Vec<String>>,
    packager: Option<Packager>,
}

impl PkgBuilder {
    fn build(self) -> Option<Vec<Pkg>> {
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

impl AsPkgBuild for &[&str] {
    fn builder(self) -> PkgBuilder {
        PkgBuilder {
            names: self.iter().map(|s| s.to_string()).collect(),
            allowed_hostnames: None,
            allowed_arches: None,
            packager: None,
        }
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
            names: vec![value.to_string()],
            allowed_hostnames: None,
            allowed_arches: None,
            packager: None,
        }
    }
}

impl From<String> for PkgBuilder {
    fn from(value: String) -> Self {
        PkgBuilder {
            names: vec![value],
            allowed_hostnames: None,
            allowed_arches: None,
            packager: None,
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
