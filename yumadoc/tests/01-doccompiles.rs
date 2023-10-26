use yumadoc::docu;

#[allow(unused)]
// #[docu("this", should, comp1le fa1l)]
fn test() {
    /// the universe is impressed by this number
    let mut magic_number = 42;

    // magic_number = 39;

    /// true magic
    assert_eq!(magic_number, 3);
}

// trait Stub {
//     fn stub() -> Self;
// }
//
// impl Stub for Model {
//     fn stub() -> Self {
//         Self {
//             documentaion: vec![],
//             item: parse_quote!(
//                 fn f() {}
//             ),
//         }
//     }
// }

// #[test]
// fn attributes_are_preserved() {
//     let model = analyze(parse_quote!(
//         #[a]
//         /// things and stuff
//         #[doc = "test"]
//         #[b]
//         fn f(x: bool) {}
//     ));
//
//     let expected: &[Attribute] = &[parse_quote!(#[a]), parse_quote!(#[b])];
//
//     assert_eq!(expected, model.item.attrs);
// }

// #[test]
// fn docs_are_extracted() {
//     let model = analyze(parse_quote!(
//         #[a]
//         #[doc = "test"]
//         #[b]
//         /// more test
//         fn f(x: bool) {}
//     ));
//
//     let expected: &[Attribute] = &[
//         parse_quote!(#[doc = "test"]),
//         parse_quote!(#[doc = r" more test"]),
//     ];
//
//     assert_eq!(expected, model.documentaion);
// }

// #[test]
// fn attr_conversion_works() {
//     let mut model = Model::stub();
//     model.documentaion.push(parse_quote!(#[doc = "test"]));
//     let ir = lower(model);
//
//     let expected: &[Comment] = &[Comment::Attr(parse_quote!(#[doc = "test"]))];
//
//     assert_eq!(expected, ir.documentaion);
// }
//
// #[test]
// fn adding_stmt_docs() {
//     let mut model = Model::stub();
//     model.documentaion.push(parse_quote!(#[doc = "still here"]));
//     model.item = parse_quote!(
//         fn f() {
//             /// the best number
//             let x = 42;
//         }
//     );
//     let ir = lower(model);
//
//     let expected: &[Comment] = &[
//         Comment::Attr(parse_quote!(#[doc = "still here"])),
//         Comment::Attr(parse_quote!(#[doc = r" the best number"])),
//         Comment::Attr(parse_quote!(#[doc = "```rust"])),
//         Comment::Code(parse_quote!(let x = 42;)),
//         Comment::Attr(parse_quote!(#[doc = "```"])),
//     ];
//
//     assert_eq!(expected, ir.documentaion);
// }
