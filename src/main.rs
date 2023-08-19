fn main() {
    println!("Hello, world!");
}

struct Package {
    deps: Vec<Package>,
    build: Vec<Package>,
    install_type: Backend,
}

enum Backend {
    PkgBuild,
    Justfile,
    CargoToml,
    Portage,
    Frieght,
}

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

/// The package backend for pacman and and the aur
struct PkgBuild {}

/// The rust build system that can do most the same as PkgBuild
struct Justfile {}

/// The packaging systems for rust code build from source
struct CargoToml {}

/// The package backend for gentoos source based builds
struct Portage {}

/// The native package build implementations
struct Freight {}
