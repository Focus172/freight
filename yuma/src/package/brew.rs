use std::process::Command;
use super::PackageBackend;

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

    fn install_packages(&mut self, names: &[&str]) {
        Command::new("brew")
            .arg("install")
            .args(names)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    fn remove_packages(&mut self, names: &[&str]) {
        Command::new("brew")
            .arg("remove")
            .args(names)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}
