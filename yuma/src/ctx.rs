use crate::callbacks::{Callbacks, YumaCallbackSig};
use crate::deriv::pkg::list::{AsPkgList, Packages};
use crate::deriv::srv::Services;
use crate::prelude::*;
use requestty::Question;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
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
        log::info!("Starting Update.");

        // TODO: unwind the changes when an error occurs

        let packager = Packager::guess();
        let installed: HashSet<String> = HashSet::from_iter(packager.list_installed());

        let mut prunable: HashSet<String> = HashSet::from_iter(packager.list_leaves());

        for mut pkgs in self.packages.enabled.drain(..) {
            let q = Question::confirm("install")
                .message(format!("Install {:?} ?", &pkgs.names))
                .default(false)
                .build();

            let requestty::Answer::Bool(yes) = requestty::prompt_one(q)? else {
                unreachable!()
            };

            if yes {
                // remove packages about to be installed from prunable list
                pkgs.names.iter().for_each(|name| {
                    prunable.remove(name);
                });
                // only install packages that are not already installed
                pkgs.names.retain(|name| !installed.contains(name));
                pkgs.packager.install(pkgs.names)?;
            }
        }

        if !prunable.is_empty() {
            let packager = Packager::guess();

            let q = Question::confirm("remove")
                .message(format!("Remove {:?} ?", prunable))
                .default(false)
                .build();

            if requestty::prompt_one(q).unwrap().as_bool().unwrap() {
                packager.remove_packages(prunable.into_iter().map(Into::into).collect());
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
            crate::log::info!("<red>Running callback</>: {name}");
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
