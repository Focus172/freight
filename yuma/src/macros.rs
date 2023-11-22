/// Macro for creating package sets in a easy way
/// valid forms:
/// ```no_compile
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
    // base cases for recursion
    () => {};
    // ( $p:ident ) => {};

    // Collect a set of names
    (PKG [ $($es:ident),+ ] $( $rest:tt )* ) => {
        let mut p = Pkgs::from_names([ $( stringify!($es) ),+ ]);
        y! { @ p $( $rest )* }
    };

    // one name
    (PKG $name:ident $( $rest:tt )* ) => {
        let mut p = Pkgs::from(stringify!($name));
        y! { @ p $( $rest )* }
    };



    (@ $p:ident FROM $pkgr:ident $( $rest:tt )* ) => {
        let __pkgr = y!(@ PKGR $pkgr);
        $p.backend(__pkgr);
        y! { @ $p $( $rest )* }
    };

    // Terminating statements. These explicitally insert a line ending to cause
    // a compile error if anything is after them
    (@ $p:ident IN $ctx:ident $( $rest:tt )*) => {
        $ctx.with($p);
        y! { ; $( $rest )* }
    };

    (@ $p:ident AS $name:ident $( $rest:tt )* ) => {
        let mut $name = $p;
        y! { ; $( $rest )* }
    };


    // internal function to get names of pkgrs
    (@ PKGR brew) => {
        $crate::deriv::packager::PackagerType::Brew
    };
    (@ PKGR paru) => {
        $crate::deriv::packager::PackagerType::Paru
    };
    (@ PKGR fake) => {
        $crate::deriv::packager::PackagerType::Fake
    };
    // (INSTALL $name:ident) => {};

    // line endings
    ( ; $($rest:tt)* ) => { y! { $( $rest )* } };
    ( @ $p:ident $($rest:tt)* ) => { y! { $( $rest )* } };
}

#[macro_export]
macro_rules! gaurd {
    ($cond:expr, $( $msg:expr ),+ ) => {
        if !$cond {
            $crate::log::info!( $( $msg ),+ );
            return Default::default();
        }
    };
}
