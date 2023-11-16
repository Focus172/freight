use yuma::prelude::Pkgs;
use yuma::y;

#[test]
fn pkg_list_y_macros() {
    // y! { PKG a, b, c AS l1 };
    // let l2 = y! { PKG a, b, c };
    // let l3 = vec![Pkgs::from("a"), Pkgs::from("b"), Pkgs::from("c")];
    // assert_eq!(l1, l2);
    // assert_eq!(l2, l3);
}

#[test]
fn compiles() {
    // let _0 = y! { PKG a, b, c FROM brew };
    // let _a = y!(PKG test);
    // let _b = y! { PKG asdf FROM paru };
}
