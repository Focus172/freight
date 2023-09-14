pub mod callbacks;
pub mod deriv;
pub mod error;
pub mod prelude;
pub mod service;

use crate::prelude::*;

use deriv::list::AsPkgBuilderList;
// re-export of inline documentation functions
pub use yumadoc::inline_doc as yumadoc;

use serde::{Deserialize, Serialize};
use std::fs;

use crate::{callbacks::Callbacks, deriv::Packages, service::Services};

#[derive(Default, Serialize, Deserialize)]
#[must_use]
pub struct YumaCtx {
    packages: Packages,
    services: Services,
    #[serde(skip)]
    callbacks: Callbacks,
    #[serde(skip)]
    write_on_drop: bool,
}

impl YumaCtx {
    pub const fn new() -> Self {
        Self {
            packages: Packages::new(),
            services: Services::new(),
            callbacks: Callbacks::new(),
            write_on_drop: true,
        }
    }

    /// Interface for adding to the pkglist. See [`PkgBuilder`] for info on customization
    pub fn add(&mut self, pkgs: impl AsPkgBuilderList) {
        self.packages.enabled.extend(
            pkgs.list()
                .into_iter()
                .flat_map(PkgBuilder::build)
                .flatten(),
        );
    }

    /// Adds a function to a list of callbacks to be ran after the next call to
    /// update
    pub fn schedule(&mut self, f: impl FnOnce() -> YumaResult + 'static) {
        self.callbacks.queued.push(Box::new(f));
    }

    // # Safety
    // this meathod is very untested and could brick you device
    // pub unsafe fn enable(&mut self, services: &[&str]) {
    //     for ser in services {
    //         let ser = ser.to_string();
    //         if self.services.enabled.contains(&ser) {
    //             println!("Duplicate Service skipped: {}", ser);
    //         } else {
    //             self.services.enabled.push(ser);
    //         }
    //     }
    // }

    pub fn update_single(&mut self) {
        // TODO: find a way to use the bundled packager for each thing
        let packager = crate::deriv::Packager::guess();

        for pkg in self.packages.enabled.iter_mut() {
            log::info!("installing: {}", &pkg.name);
            // pkg.packager.install(&pkg.name);
            // packager.install(&pkg.name);
            packager.install_packages(&[&pkg.name]);
        }
    }

    /// Installs and enables the packages and services that have been added since
    /// the last update.
    ///
    /// # Future Compat
    /// This meathod is the most likely to change as time goes on. Here are some
    /// of the things that may change as they go on.
    /// - Packages are not removed until [`finalize`] is called.
    /// - Enable and disable services
    ///
    /// ## Panics
    /// Panics when the installation of anything fails. It is best practice to
    /// have multipule updates throughout your build so if a later stage fails
    /// you still have the core packages (like your kernal and drivers) working.
    pub fn update(&mut self) {
        let packager = crate::deriv::Packager::guess();

        log::info!("installing all thing things");

        let pkgnames: Vec<&str> = self
            .packages
            .enabled
            .iter()
            .map(|p| p.name.as_str())
            .collect();

        // packager.install_packages(&pkgnames);

        let installed = packager.list_leaves();
        let allinstalled = packager.list_installed();

        let to_remove: Vec<&str> = installed
            .iter()
            .map(|s| s.as_str())
            .filter(|pkg| !pkgnames.contains(pkg))
            .collect();

        let to_install: Vec<&str> = pkgnames
            .iter()
            .filter(|pkg| !allinstalled.contains(&pkg.to_string()))
            .cloned()
            .collect();

        if !to_remove.is_empty() {
            log::info!("Removing: {:?}", to_remove);
            packager.remove_packages(&to_remove);
        }

        if !to_install.is_empty() {
            log::info!("Installing: {:?}", to_install);
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

        for fun in self.callbacks.queued.drain(..) {
            fun().unwrap();
        }
    }

    /// Sets an internal variable that singals to not cache the output of this
    /// derivation. This can allow for building a revertable version of your
    /// system or for running unit test on your config if you are ill
    pub fn skip_cache(&mut self) {
        self.write_on_drop = false;
    }
}

impl Drop for YumaCtx {
    fn drop(&mut self) {
        if self.write_on_drop {
            let w = fs::File::create("./.yumacache.json").unwrap();
            serde_json::to_writer_pretty(w, self).unwrap();
        }
    }
}

pub fn init_logger() -> YumaResult {
    init_logger_with_level(log::LevelFilter::Trace)
}

pub fn init_logger_with_level(level: log::LevelFilter) -> YumaResult {
    simplelog::TermLogger::init(
        level,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn allowed_addables() {
        let mut ctx = ctx();

        ctx.skip_cache();

        ctx.add("test");
        ctx.add("test".b().on_hosts(&["test"]));
        ctx.add(["test"]);
        ctx.add(["test", "test"].b());
        // ctx.add(("test", "test").builder());
        ctx.add(["test".b()].b());
    }
}
