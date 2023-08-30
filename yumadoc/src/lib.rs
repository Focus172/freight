use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, parse_quote, Attribute, Block, Expr, Ident, ItemFn, Stmt};

type Ast = syn::ItemFn;

#[proc_macro_attribute]
pub fn inline_doc(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ast: Ast = parse_macro_input!(input as ItemFn);
    let model = analyze(ast);
    let ir = lower(model);
    codegen(ir)
}

/// Model that repersents the macro
/// none of the attributes on the function should be docs
#[derive(Debug)]
struct Model {
    documentaion: Vec<Attribute>,
    item: ItemFn,
}

fn analyze(mut ast: Ast) -> Model {
    let mut documentaion: Vec<Attribute> = Vec::new();
    let doc: Ident = parse_quote!(doc);

    let attrs = ast
        .attrs
        .iter()
        .filter(|attr| match attr.path().get_ident() {
            Some(id) => {
                if id == &doc {
                    documentaion.push((*attr).clone());
                    false
                } else {
                    true
                }
            }
            None => true,
        })
        .cloned()
        .collect();

    ast.attrs = attrs;

    Model {
        documentaion,
        item: ast as ItemFn,
    }
}

trait Commentable {
    fn is_doc_comment(&self) -> bool;
    fn as_doc_comment(&self) -> Option<Expr>;
}
impl Commentable for Attribute {
    fn is_doc_comment(&self) -> bool {
        let doc: Ident = parse_quote!(doc);
        match self.path().get_ident() {
            Some(id) => id == &doc,
            None => false,
        }
    }

    fn as_doc_comment(&self) -> Option<Expr> {
        match &self.meta {
            syn::Meta::NameValue(nm) => Some(nm.value.clone()),
            _ => None,
        }
    }
}

trait Attributable {
    fn get_attrs(&self) -> Vec<Attribute>;

    fn set_attrs(&mut self, attrs: Vec<Attribute>);
}

impl Attributable for Stmt {
    fn get_attrs(&self) -> Vec<Attribute> {
        match self {
            Stmt::Local(l) => l.attrs.clone(),
            Stmt::Macro(m) => m.attrs.clone(),
            Stmt::Item(_) => vec![],
            Stmt::Expr(..) => vec![],
        }
    }

    fn set_attrs(&mut self, attrs: Vec<Attribute>) {
        match self {
            Stmt::Local(l) => l.attrs = attrs,
            Stmt::Macro(m) => m.attrs = attrs,
            Stmt::Item(_) => {}
            Stmt::Expr(..) => {}
        }
    }
}

struct Ir {
    documentaion: Vec<Comment>,
    item: ItemFn,
}

#[derive(Debug, PartialEq, Eq)]
enum Comment {
    Attr(Attribute),
    Code(Stmt),
}

impl ToTokens for Comment {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Comment::Attr(a) => a.to_tokens(tokens),
            Comment::Code(c) => {
                let s = c.to_token_stream().to_string();
                tokens.extend(quote::quote!(#[doc = #s]));
            }
        }
    }
}

fn lower(model: Model) -> Ir {
    let mut docs: Vec<Comment> = model
        .documentaion
        .iter()
        .map(|at| Comment::Attr(at.clone()))
        .collect();

    let mut stmts = vec![];

    model.item.block.stmts.iter().for_each(|stmt| {
        let attrs = stmt.get_attrs();

        let mut stmt = stmt.clone();

        let new_docs: Vec<Attribute> = attrs
            .iter()
            .filter(|at| at.is_doc_comment())
            // .map(|at| at.as_doc_comment().unwrap())
            .cloned()
            .collect();

        if !new_docs.is_empty() {
            let new_attr: Vec<Attribute> = attrs
                .iter()
                .filter(|at| !at.is_doc_comment())
                .cloned()
                .collect();

            let mut print_stmt = stmt.clone();
            print_stmt.set_attrs(vec![]);

            stmt.set_attrs(new_attr);

            for doc in new_docs {
                docs.push(Comment::Attr(doc));
            }

            let open_code = "```rust";
            let close_code = "```";
            docs.push(Comment::Attr(parse_quote!(#[doc = #open_code])));
            docs.push(Comment::Code(print_stmt));
            docs.push(Comment::Attr(parse_quote!(#[doc = #close_code])));
        }

        stmts.push(stmt);
    });

    let sig = model.item.sig;
    let vis = model.item.vis;
    let attrs = model.item.attrs;

    let brace_token = model.item.block.brace_token;

    let block = Block { brace_token, stmts };

    let function = ItemFn {
        attrs,
        sig,
        vis,
        block: Box::new(block),
    };
    Ir {
        documentaion: docs,
        item: function,
    }
}

fn codegen(ir: Ir) -> TokenStream {
    let item = ir.item;

    let docs = ir.documentaion;

    quote::quote!(
        #(#docs)*
        #item
    )
    .into()
}

#[cfg(test)]
mod test {
    use super::*;

    trait Stub {
        fn stub() -> Self;
    }

    impl Stub for Model {
        fn stub() -> Self {
            Self {
                documentaion: vec![],
                item: parse_quote!(
                    fn f() {}
                ),
            }
        }
    }

    #[test]
    fn attributes_are_preserved() {
        let model = analyze(parse_quote!(
            #[a]
            /// things and stuff
            #[doc = "test"]
            #[b]
            fn f(x: bool) {}
        ));

        let expected: &[Attribute] = &[parse_quote!(#[a]), parse_quote!(#[b])];

        assert_eq!(expected, model.item.attrs);
    }

    #[test]
    fn docs_are_extracted() {
        let model = analyze(parse_quote!(
            #[a]
            #[doc = "test"]
            #[b]
            /// more test
            fn f(x: bool) {}
        ));

        let expected: &[Attribute] = &[
            parse_quote!(#[doc = "test"]),
            parse_quote!(#[doc = r" more test"]),
        ];

        assert_eq!(expected, model.documentaion);
    }

    #[test]
    fn attr_conversion_works() {
        let mut model = Model::stub();
        model.documentaion.push(parse_quote!(#[doc = "test"]));
        let ir = lower(model);

        let expected: &[Comment] = &[Comment::Attr(parse_quote!(#[doc = "test"]))];

        assert_eq!(expected, ir.documentaion);
    }

    #[test]
    fn adding_stmt_docs() {
        let mut model = Model::stub();
        model.documentaion.push(parse_quote!(#[doc = "still here"]));
        model.item = parse_quote!(
            fn f() {
                /// the best number
                let x = 42;
            }
        );
        let ir = lower(model);

        let expected: &[Comment] = &[
            Comment::Attr(parse_quote!(#[doc = "still here"])),
            Comment::Attr(parse_quote!(#[doc = r" the best number"])),
            Comment::Attr(parse_quote!(#[doc = "```rust"])),
            Comment::Code(parse_quote!(let x = 42;)),
            Comment::Attr(parse_quote!(#[doc = "```"])),
        ];

        assert_eq!(expected, ir.documentaion);
    }
}
