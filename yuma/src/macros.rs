#[macro_export]
macro_rules! pkg {
    ($name:ident) => {
        $crate::pkgs::Pkg {
            name: $name,
            packager: $crate::pkgs::Packager::guess(),
        }
    };
}
