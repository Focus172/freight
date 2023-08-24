use std::{
    env, fs,
    process::{Command, Stdio},
};

fn main() {
    // let file = env::args()
    //     .nth(1)
    //     .expect("pass the file in as the first argument");

    // let content = fs::read_to_string(&file).unwrap();

    // let ast = syn::parse_file(&content).expect("you have a syntax error");
    // println!("{:#?}", ast);
}

// fn old_main() {
//     let mut args = env::args().skip(1);
//     let main = args.next().unwrap();
//     let import = args.next().unwrap();
//
//     let import = fs::read_to_string(&import).unwrap();
//     let import: PkgSet = yaml::from_str(&import).unwrap();
//
//     let main = fs::read_to_string(&main).unwrap();
//     let main: PkgSet = yaml::from_str(&main).unwrap();
//
//     let child = Command::new("pacman")
//         .arg("-Qqt")
//         .stdout(Stdio::piped())
//         .spawn()
//         .unwrap();
//     let out = child.wait_with_output().unwrap().stdout;
//     let out = String::from_utf8(out).unwrap();
//     let installed: Vec<String> = out.lines().map(|l| l.to_string()).collect();
//
//     let child = Command::new("pacman")
//         .arg("-Qq")
//         .stdout(Stdio::piped())
//         .spawn()
//         .unwrap();
//     let out = child.wait_with_output().unwrap().stdout;
//     let out = String::from_utf8(out).unwrap();
//     let allinstalled: Vec<String> = out.lines().map(|l| l.to_string()).collect();
//
//     let to_remove: Vec<String> = installed
//         .iter()
//         .map(|pkg| {
//             if main.contains(pkg) || import.contains(pkg) {
//                 None
//             } else {
//                 Some(pkg)
//             }
//         })
//         .flatten()
//         .cloned()
//         .collect();
//
//     let mut pac = main.pac.clone();
//     pac.extend(import.pac.clone());
//     let pac: Vec<String> = pac
//         .iter()
//         .filter(|pkg| !allinstalled.contains(pkg))
//         .cloned()
//         .collect();
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
//     if !to_remove.is_empty() {
//         println!("removeing: {:?}", to_remove);
//
//         Command::new("sudo")
//             .stdin(Stdio::inherit())
//             .arg("pacman")
//             .arg("-Rns")
//             .args(to_remove)
//             .spawn()
//             .unwrap()
//             .wait()
//             .unwrap();
//     }
//     if !pac.is_empty() {
//         println!("installing: {:?}", pac);
//
//         Command::new("sudo")
//             .stdin(Stdio::inherit())
//             .arg("pacman")
//             .arg("-S")
//             .arg("--needed")
//             .args(pac)
//             // .args(main.pac)
//             // .args(import.pac)
//             .spawn()
//             .unwrap()
//             .wait()
//             .unwrap();
//     }
//
//     if !aur.is_empty() {
//         println!("aur installing: {:?}", aur);
//         Command::new("paru")
//             .arg("-S")
//             .arg("--needed")
//             .args(aur)
//             // .args(main.aur)
//             // .args(import.aur)
//             .spawn()
//             .unwrap()
//             .wait()
//             .unwrap();
//     }
// }

// #[derive(Debug)]
// struct Yuma {
// all: PkgSet,
// import: PkgSet,
// }

// #[derive(Debug, serde::Serialize, serde::Deserialize)]
struct PkgSet {
    pac: Vec<String>,
    aur: Vec<String>,
}

impl PkgSet {
    pub fn contains(&self, name: &String) -> bool {
        self.pac.contains(name) || self.aur.contains(name)
    }
}

// struct Package {
// deps: Vec<Package>,
// build: Vec<Package>,
// install_type: Backend,
// }

// enum Backend {
// PkgBuild,
// Justfile,
// CargoToml,
// Portage,
// Frieght,
// }

trait PackageBackend {
    fn list_installed_packages(&self) -> Vec<String>;

    fn install_package(&mut self, name: String);

    // fn install_packages(&mut self, names: Vec<String>) {
    //     for name in names {
    //         self.install_package(name.clone());
    //     }
    // }

    fn remove_package(&mut self, name: String);
}

// The package backend for pacman and and the aur
// struct PkgBuild {}

// The rust build system that can do most the same as PkgBuild
// struct Justfile {}

// The packaging systems for rust code build from source
// struct CargoToml {}

// The package backend for gentoos source based builds
// struct Portage {}

// The native package build implementations
// struct Freight {}
