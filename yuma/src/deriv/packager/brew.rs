use crate::prelude::*;

use std::process::Command;

use super::PackageBackend;

#[derive(Debug, Default)]
pub struct BrewPackager;

impl PackageBackend for BrewPackager {
    fn list_leaves(&self) -> Vec<String> {
        let stdout = Command::new("brew").arg("leaves").output().unwrap().stdout;
        String::from_utf8(stdout)
            .unwrap()
            .lines()
            .map(ToString::to_string)
            .collect()
    }

    fn list_installed(&self) -> Vec<String> {
        let stdout = Command::new("brew").arg("list").output().unwrap().stdout;
        String::from_utf8(stdout)
            .unwrap()
            .lines()
            .map(ToString::to_string)
            .collect()
    }

    fn install(&self, pkgs: Vec<super::SpecficName>) -> Result<()> {
        Command::new("brew")
            .arg("install")
            .args(pkgs)
            .spawn()?
            .wait()?;

        Ok(())
    }

    fn remove(&self, pkgs: Vec<super::SpecficName>) -> Result<()> {
        Command::new("brew")
            .arg("remove")
            .args(pkgs)
            .spawn()?
            .wait()?;
        Ok(())
    }

    fn resolve_name(&self, _name: super::GenericName) -> super::SpecficName {
        todo!()
    }
}
