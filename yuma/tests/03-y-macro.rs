use yuma::prelude::*;

#[test]
fn pkg_list_y_macros() {
    y! {
        PKG text AS p1;
        PKG name FROM fake AS p2;
        PKG [ a, b, c ] FROM fake AS p3;
        // PKG [ r, a ] FROM fake AS p4;
    };

    let q1 = Pkgs {
        names: vec![String::from("text")],
        packager: Packager::guess(),
    };
    assert_eq!(p1, q1);

    let q2 = Pkgs {
        names: vec![String::from("name")],
        packager: Packager::fake(),
    };
    assert_eq!(p2, q2);

    let q3 = Pkgs {
        names: <[_]>::into_vec(Box::new([
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ])),
        packager: Packager::fake(),
    };

    assert_eq!(p3, q3);
}

// #[test]
// fn compiles() {
//     let _0 = y! { PKG a, b, c FROM brew };
//     let _a = y!(PKG test);
//     let _b = y! { PKG asdf FROM paru };
// }
