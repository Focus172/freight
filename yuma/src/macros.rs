/// Macro for creating package sets in a easy way
/// valid forms:
/// ```
/// use yuma::prelude::*;
/// use yuma::deriv::packager::PackagerType;
///
/// let _single: Pkg = y!(PKG test);
/// let _specific: Pkg = y!(PKG test FROM paru);
/// let _list: Vec<Pkg> = y!(PKG test, test2, test3);
/// let _lists: Vec<Pkg> = y!(PKG test, test2, test3 FROM brew);
/// let _pkgr: PackagerType = y!(PKGR brew);
///
/// ```
#[macro_export]
macro_rules! y {
    // Simple conversion case
    (PKG $name:ident) => { Into::<$crate::prelude::Pkg>::into(stringify!($name)) };

    // Full constructor case
    (PKG $name:ident FROM $pkgr:ident) => {{
        const __PKGR: $crate::deriv::packager::PackagerType = y!(PKGR $pkgr);
        const __NAME: &'static str = stringify!($name);
        $crate::prelude::Pkg::new(__PKGR, __NAME)
    }};

    // construct many with shorthand
    (PKG $($es:ident),+) => { Vec::from([ $(y! { PKG $es }),+ ]) };

    // construct many with full constructor
    (PKG $($es:ident),+ FROM $p:ident) => { Vec::from([ $(y! { PKG $es FROM $p}),+ ]) };

    (PKGR brew) => { $crate::deriv::packager::PackagerType::Brew };
    (PKGR paru) => { $crate::deriv::packager::PackagerType::Paru };

    (INSTALL $name:ident) => {};
}
