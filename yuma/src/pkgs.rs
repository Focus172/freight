use std::process::Command;

// pub struct Pkg {
//     name: String,
//     packager: Arc<Mutex<dyn PackageBackend>>,
// }

pub trait PackageBackend {
    fn list_installed(&self) -> Vec<String>;

    fn list_leaves(&self) -> Vec<String>;

    fn install_packages(&mut self, names: &[&str]);

    fn remove_packages(&mut self, names: &[&str]);
}

pub struct Packager {
    _packager_type: PackagerType,
    backend: Box<dyn PackageBackend>,
}

impl Packager {
    /// Based on the operating system and installed packages make the best guess
    /// for the package backend to use
    pub fn guess() -> Self {
        Self {
            _packager_type: PackagerType::Paru,
            backend: Box::new(ParuPackager),
        }
    }
}

impl std::ops::Deref for Packager {
    type Target = Box<dyn PackageBackend>;

    fn deref(&self) -> &Self::Target {
        &self.backend
    }
}

impl std::ops::DerefMut for Packager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.backend
    }
}

pub enum PackagerType {
    Paru,
    // Brew(BrewPackager),
    //
    // PkgBuild,
    // Justfile,
    // CargoToml,
    // Portage,
    // Frieght,
}

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

    fn install_packages(&mut self, names: &[&str]) {
        Command::new("paru")
            .arg("-S")
            .arg("--needed")
            .args(names)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    fn remove_packages(&mut self, names: &[&str]) {
        Command::new("paru")
            .arg("-Rns")
            .args(names)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
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
