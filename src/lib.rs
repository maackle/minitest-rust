use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, TokenStream, TokenTree};
use proc_macro_error::proc_macro_error;
use quote::{spanned::Spanned, ToTokens};
use syn::{parse_macro_input, Attribute, Item, ItemFn};

#[proc_macro_error]
#[proc_macro_attribute]
pub fn minitest(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut it = attrs.into_iter();
    let mut special: Option<Ident> = None;
    let mut test_type: Option<Ident> = None;
    let mut real_type: Option<Ident> = None;

    if let Some(TokenTree::Ident(i)) = it.next() {
        special = Some(i);
    } else {
        panic!("First attribute argument must be the name of the replacement fn")
    }
    let _comma = it.next().expect("minitest must have two arguments");

    if let Some(TokenTree::Ident(i)) = it.next() {
        real_type = Some(i);
    } else {
        panic!("Second attribute argument must be the name of the real struct")
    }

    let special = special
        .expect("minitest must have two arguments")
        .to_string();
    let real_type = real_type.expect("minitest must have two arguments");

    fn replace(ts: TokenStream, special: String, replacement: TokenTree) -> TokenStream {
        ts.into_iter()
            .map(|tt| match tt {
                TokenTree::Ident(ref i) if i.to_string() == special => replacement.clone(),
                TokenTree::Group(g) => TokenTree::Group(Group::new(
                    g.delimiter(),
                    replace(g.stream(), special.clone(), replacement.clone()),
                )),
                other => other,
            })
            .collect()
    }

    let input = parse_macro_input!(input as Item);

    if let Item::Fn(f) = input {
        let mut fa = f.clone();
        let mut fb = f.clone();
        let id = quote::quote!(std::convert::identity).into();
        let test_body = replace(
            f.block.clone().into_token_stream().into(),
            special.clone(),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, id)),
        );
        let newname = Group::new(
            Delimiter::Parenthesis,
            [
                TokenTree::Ident(real_type.clone()),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Ident(Ident::new("from", real_type.span())),
            ]
            .into_iter()
            .collect(),
        );
        let real_body = replace(
            f.block.into_token_stream().into(),
            special,
            TokenTree::Group(newname),
        );
        let ident = f.sig.ident;
        fa.sig.ident = syn::Ident::new(&format!("{}_test", ident.to_string()), ident.span());
        fb.sig.ident = syn::Ident::new(&format!("{}_real", ident.to_string()), ident.span());

        fa.block = Box::new(syn::parse2(test_body.into()).expect("could not parse test body"));
        fb.block = Box::new(syn::parse2(real_body.into()).expect("could not parse real body"));
        let out = quote::quote! {
            #[test]
            #fa

            #[test]
            #fb
        };

        dbg!(out.to_string());
        out.into()
    } else {
        panic!("mulitest must be applied to a function");
    }
}
