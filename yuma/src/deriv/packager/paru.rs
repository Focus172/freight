use crate::prelude::*;

use std::{collections::HashMap, fs, process::Command, sync::OnceLock};

use super::PackageBackend;

pub static PARU_NAME_MAP: OnceLock<HashMap<super::GenericName, super::SpecficName>> =
    OnceLock::new();

#[derive(Debug, Default)]
pub struct ParuPackager;

impl PackageBackend for ParuPackager {
    fn list_leaves(&self) -> Vec<String> {
        let stdout = Command::new("paru").arg("-Qqt").output().unwrap().stdout;
        String::from_utf8(stdout)
            .unwrap()
            .lines()
            .map(ToString::to_string)
            .collect()
    }

    fn list_installed(&self) -> Vec<String> {
        let stdout = Command::new("paru").arg("-Qq").output().unwrap().stdout;
        String::from_utf8(stdout)
            .unwrap()
            .lines()
            .map(ToString::to_string)
            .collect()
    }

    fn install(&self, pkgs: Vec<String>) -> Result<()> {
        Command::new("paru")
            .arg("-S")
            .arg("--needed")
            // .arg("--yes")
            .args(pkgs)
            .spawn()?
            .wait()?;
        Ok(())
    }

    fn remove_packages(&self, pkgs: Vec<String>) {
        Command::new("paru")
            .arg("-Rns")
            // .arg("--yes")
            .args(pkgs)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    fn resolve_name(&self, name: super::GenericName) -> super::SpecficName {
        PARU_NAME_MAP
            .get_or_init(|| {
                let path: &str = "/home/focus/dox/code/freight/shipyard/index.json";
                let f = fs::File::open(path).unwrap();
                json::from_reader(f).unwrap()
            })
            .get(&name)
            .unwrap()
            .clone()
    }
}

//     let mut pac = main.pac.clone();
//
//     let mut aur = main.aur.clone();
//     aur.extend(import.aur.clone());
//     let aur: Vec<String> = aur
//         .iter()
//         .filter(|pkg| !allinstalled.contains(pkg))
//         .cloned()
//         .collect();
//     // println!("{:?}", to_remove);
//
