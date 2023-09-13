pub mod callbacks;
pub mod deriv;
pub mod error;
pub mod prelude;
pub mod service;

use crate::prelude::*;

use crate::callbacks::Callbacks;

// re-export of inline documentation functions
pub use yumadoc::inline_doc as yumadoc;

use std::{fs, process};

use deriv::{builder::PkgBuilder, Packages};
use service::Services;

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[must_use]
pub struct YumaCtx {
    enabled_packages: Vec<String>,
    packages: Packages,
    services: Services,
    #[serde(skip)]
    callbacks: Callbacks,
}

impl YumaCtx {
    pub const fn new() -> Self {
        Self {
            enabled_packages: Vec::new(),
            packages: Packages::new(),
            services: Services::new(),
            callbacks: Callbacks::new(),
        }
    }

    pub fn add(&mut self, pkgs: &[&str]) {
        for pkg in pkgs {
            let pkg = pkg.to_string();
            if self.enabled_packages.contains(&pkg) {
                println!("Duplicate Package skipped: {}", pkg);
            } else {
                self.enabled_packages.push(pkg);
            }
        }
    }

    /// The fueture interface for adding to the pkglist
    pub fn add2(&mut self, pkgs: impl IntoIterator<Item = impl Into<PkgBuilder>>) {
        self.packages.enabled.extend(
            pkgs.into_iter()
                .map(Into::into)
                .filter_map(PkgBuilder::build),
        );
    }

    /// Adds a function to a list of callbacks to be ran after the next call to
    /// update
    pub fn schedule(&mut self, f: impl FnOnce(Box<dyn AsMut<YumaCtx>>) -> YumaResult + 'static) {
        self.callbacks.queued.push(Box::new(f));
    }

    /// adds the pkgs to the configuration if the given hostname matches the current hostname
    pub fn add_if_host(&mut self, host: &str, pkgs: &[&str]) {
        let _h =
            String::from_utf8(process::Command::new("hostname").output().unwrap().stdout).unwrap();
        let hostname = _h.trim();
        if hostname == host {
            self.add(pkgs)
        }
    }

    /// adds the packages to the config if any of the given hosts matches the current
    pub fn add_if_hosts(&mut self, hosts: &[&str], pkgs: &[&str]) {
        let _h =
            String::from_utf8(process::Command::new("hostname").output().unwrap().stdout).unwrap();
        let hostname = _h.trim();
        if hosts.contains(&hostname) {
            self.add(pkgs)
        }
    }

    /// # Safety
    /// this meathod is very untested and could brick you device
    pub unsafe fn enable(&mut self, services: &[&str]) {
        for ser in services {
            let ser = ser.to_string();
            if self.enabled_packages.contains(&ser) {
                println!("Duplicate Service skipped: {}", ser);
            } else {
                self.enabled_packages.push(ser);
            }
        }
    }

    /// Installs and enables the packages and services that have been added since
    /// the last update.
    ///
    /// FUTURE INCOMPAT: Packages are not removed until [`finalize`] is called.
    /// right now this prunes old packages but wont in the future
    ///
    /// # Panics
    /// Panics when the installation of anything fails. It is best practice to
    /// have multipule updates throughout your build so if a later stage fails
    /// you still have the core packages (like your kernal and drivers) working.
    ///
    pub fn update(&mut self) {
        let mut packager = crate::deriv::Packager::guess();

        let installed = packager.list_leaves();
        let allinstalled = packager.list_installed();

        let to_remove: Vec<&str> = installed
            .iter()
            .filter(|pkg| !&self.enabled_packages.contains(pkg))
            .map(|s| s.as_str())
            .collect();

        let to_install: Vec<&str> = self
            .enabled_packages
            .iter()
            .filter(|pkg| !allinstalled.contains(pkg))
            .map(|s| s.as_str())
            .collect();

        if !to_remove.is_empty() {
            println!("Removing: {:?}", to_remove);
            packager.remove_packages(&to_remove);
        }

        if !to_install.is_empty() {
            println!("Installing: {:?}", to_install);
            packager.install_packages(&to_install);
        }

        // let mut servicer = Services::guess();
        // let enabled = servicer.list_leaves_enabled();
        //
        // let to_enable: Vec<&str> = self
        //     .enabled_services
        //     .iter()
        //     .filter(|ser| enabled.contains(ser))
        //     .map(|s| s.as_str())
        //     .collect();
        // if !to_enable.is_empty() {
        //     println!("Enabling services: {:?}", to_enable);
        //     servicer.enable(&to_enable)
        // }
    }

    pub fn update2(&mut self) {
        for pkg in self.packages.enabled.iter_mut() {
            pkg.packager.install(&pkg.name);
        }
    }
}

impl Drop for YumaCtx {
    fn drop(&mut self) {
        let w = fs::File::create("./.yumacache.json").unwrap();
        serde_json::to_writer_pretty(w, self).unwrap();
    }
}
