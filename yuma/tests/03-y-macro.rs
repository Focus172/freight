use yuma::prelude::Pkg;
use yuma::y;

#[test]
fn pkg_list_y_macros() {
    let l1 = y! { PKG a, b, c };
    let l2 = vec![Pkg::from("a"), Pkg::from("b"), Pkg::from("c")];
    assert_eq!(l1, l2);
}

#[test]
fn compiles() {
    let _0 = y! { PKG a, b, c FROM brew };
    let _a = y!(PKG test);
    let _b = y! { PKG asdf FROM paru };
}
