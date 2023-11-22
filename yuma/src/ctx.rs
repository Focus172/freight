use crate::callbacks::{Callbacks, YumaCallbackSig};
use crate::deriv::pkg::list::{AsPkgList, Packages};
use crate::deriv::srv::Services;
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use stub::Stub;

#[derive(Debug, Serialize, Deserialize, Stub)]
pub struct YumaCtx {
    packages: Packages,
    services: Services,
    #[serde(skip)]
    callbacks: Callbacks,
    /// Determins if this context should resolve dynamically and to help with
    /// testing.
    #[serde(skip)]
    is_interactive: bool,
}

impl Default for YumaCtx {
    fn default() -> Self {
        let is_interactive = atty::is(atty::Stream::Stdout);
        Self {
            packages: Default::default(),
            services: Default::default(),
            callbacks: Default::default(),
            is_interactive,
        }
    }
}

impl YumaCtx {
    pub fn new() -> Self {
        Self::default()
    }

    /// Interface for adding to the pkglist. See [`PkgBuilder`] for info
    /// on customization.
    ///
    /// Examples include
    /// ```rust
    /// use yuma::prelude::*;
    /// let mut ctx = ctx();
    /// # ctx.dry_run();
    ///
    /// ctx.add("str");
    /// ctx.add("pkg".b().on_host("hostname"));
    /// ctx.add(["data!"]);
    /// ctx.add(["static", "array"].b());
    /// ctx.add(["build array".b()].b());
    /// ```
    pub fn add<P>(&mut self, pkgs: P)
    where
        P: AsPkgList,
    {
        self.packages.add(pkgs.list());
    }

    /// Alias for [`YumaCtx::add`] for a potential name change
    pub fn with<P>(&mut self, pkgs: P)
    where
        P: AsPkgList,
    {
        self.add(pkgs)
    }

    /// Adds a function to a list of callbacks to be ran after the next call to
    /// update
    pub fn schedule<S, F>(&mut self, name: S, f: F)
    where
        S: ToString,
        F: YumaCallbackSig,
    {
        self.callbacks.add(name.to_string(), f)
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
        log::debug!("Waiting for last cycles callbacks to end.");
        self.callbacks.wait()?;

        // TODO: unwind the changes when an error occurs
        log::info!("Starting Update.");

        self.packages.install()?;

        // ----------> self.services.install()
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

        self.callbacks.run()?;

        Ok(())
    }

    /// Sets an internal variable that singals to not cache the output of this
    /// derivation. This can allow for building a revertable version of your
    /// system or for running unit test on your config if you are ill
    pub fn dry_run(&mut self) {
        self.is_interactive = false;
    }
}

impl Drop for YumaCtx {
    fn drop(&mut self) {
        self.callbacks.run().unwrap();
        self.callbacks.wait().unwrap();

        gaurd!(self.is_interactive, "Skipping cache and pruning");

        let w = fs::File::create("./.yumacache.json").unwrap();
        json::to_writer_pretty(w, self).unwrap();

        self.packages.prune().unwrap();
    }
}
