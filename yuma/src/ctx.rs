use crate::callbacks::{Callbacks, YumaCallbackSig};
use crate::deriv::pkg::builder::AsPkgBuilderList;
use crate::deriv::pkg::list::Packages;
use crate::deriv::srv::Services;
use crate::prelude::*;
use requestty::Question;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct YumaCtx {
    packages: Packages,
    services: Services,
    #[serde(skip)]
    callbacks: Callbacks,
    #[serde(skip)]
    /// An internal variable used when testing that allows for not cluttering
    /// file system
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

    /// Interface for adding to the pkglist. See [`PkgBuilder`] for info
    /// on customization.
    ///
    /// Examples include
    /// ```rust
    /// use yuma::prelude::*;
    /// let mut ctx = ctx();
    /// # ctx.skip_cache();
    ///
    /// ctx.add("str");
    /// ctx.add("pkg".b().on_host("hostname"));
    /// ctx.add(["data!"]);
    /// ctx.add(["static", "array"].b());
    /// ctx.add(["build array".b()].b());
    /// ```
    pub fn add<A>(&mut self, pkgs: A)
    where
        A: AsPkgBuilderList,
    {
        self.packages.add(pkgs.list().into_iter())
    }

    /// Alias for [`add`] for a potential name change
    pub fn with(&mut self, pkgs: impl AsPkgBuilderList) {
        self.add(pkgs)
    }

    /// Adds a function to a list of callbacks to be ran after the next call to
    /// update
    pub fn schedule<F>(&mut self, name: impl Into<String>, f: F)
    where
        F: YumaCallbackSig,
    {
        self.callbacks.add(name, f)
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

    // pub fn update_single(&mut self) {
    //     // TODO: find a way to use the bundled packager for each thing
    //     let packager = crate::deriv::Packager::guess();
    //
    //     for pkg in self.packages.enabled.iter_mut() {
    //         log::info!("install {} ?", &pkg.name);
    //         // pkg.packager.install(&pkg.name);
    //         // packager.install(&pkg.name);
    //         packager.install_packages(&[&pkg.name]);
    //     }
    // }

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
    pub fn update(&mut self) -> Result<()> {
        // TODO: unwind the changes when an error occurs
        let packager = crate::deriv::packager::Packager::guess();

        log::info!("Begining install");

        let pkgnames: Vec<String> = self
            .packages
            .enabled
            .iter()
            .map(|p| p.name.to_owned())
            .collect();

        // packager.install_packages(&pkgnames);

        let installed = packager.list_leaves();
        let allinstalled = packager.list_installed();

        let to_remove: Vec<String> = installed
            .iter()
            .filter(|pkg| !pkgnames.contains(pkg))
            .map(|s| s.to_owned())
            .collect();

        let to_install: Vec<String> = pkgnames
            .iter()
            .filter(|pkg| !allinstalled.contains(&pkg.to_string()))
            .cloned()
            .collect();

        if !to_remove.is_empty() {
            let q = Question::confirm("remove")
                .message(format!("Remove {:?} ?", to_remove))
                .default(false)
                .build();

            if requestty::prompt_one(q).unwrap().as_bool().unwrap() {
                packager.remove_packages(Box::new(to_remove.into_iter().map(Into::into)));
            }
        }

        if !to_install.is_empty() {
            let q = Question::confirm("install")
                .message(format!("Install {:?} ?", to_install))
                .default(false)
                .build();

            if requestty::prompt_one(q).unwrap().as_bool().unwrap() {
                packager.install_packages(Box::new(to_install.into_iter().map(|s| s.into())));
            }
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
        self.run_callbacks()?;

        Ok(())
    }

    fn run_callbacks(&mut self) -> Result<()> {
        for (name, fun) in self.callbacks.queued.drain(..) {
            log::info!("Running callback: {name}");
            fun.call()?;
        }
        Ok(())
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
        self.run_callbacks().unwrap();

        if self.write_on_drop {
            let w = fs::File::create("./.yumacache.json").unwrap();
            json::to_writer_pretty(w, self).unwrap();
        }
    }
}
