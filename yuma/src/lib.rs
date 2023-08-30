mod macros;
mod pkgs;
mod serve;

/// re-export of inline documentation functions
pub use yumadoc::inline_doc as doc;

use std::process;

use pkgs::Packager;
use serve::{ServiceBackend, Services};

#[derive(Debug, Default)]
#[must_use]
pub struct YumaCtx {
    enabled_packages: Vec<String>,
    enabled_services: Vec<String>,
}
impl YumaCtx {
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
    pub fn update(self) {
        let mut packager = Packager::guess();

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

        let mut servicer = Services::guess();
        let enabled = servicer.list_leaves_enabled();

        let to_enable: Vec<&str> = self
            .enabled_services
            .iter()
            .filter(|ser| enabled.contains(ser))
            .map(|s| s.as_str())
            .collect();

        if !to_enable.is_empty() {
            println!("Enabling services: {:?}", to_enable);
            servicer.enable(&to_enable)
        }
    }
}

pub fn ctx() -> YumaCtx {
    YumaCtx::default()
}
