use proc_macro::{Group, Ident, TokenStream, TokenTree};
use syn::{parse_macro_input, Item};

// #[proc_macro_error]
#[proc_macro_attribute]
pub fn multitest(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut it = attrs.into_iter();
    let mut special: Option<Ident> = None;
    let mut test_type: Option<Ident> = None;
    let mut real_type: Option<Ident> = None;

    if let Some(TokenTree::Ident(i)) = it.next() {
        special = Some(i);
    } else {
        panic!("First attribute argument must be the name of the replacement fn")
    }
    let _comma = it.next().unwrap();

    if let Some(TokenTree::Ident(i)) = it.next() {
        test_type = Some(i);
    } else {
        panic!("Second attribute argument must be the name of the test struct")
    }
    let _comma = it.next().unwrap();

    if let Some(TokenTree::Ident(i)) = it.next() {
        real_type = Some(i);
    } else {
        panic!("Third attribute argument must be the name of the real struct")
    }

    let special = special.unwrap();

    fn replace(ts: TokenStream, special: Ident) -> TokenStream {
        ts.into_iter()
            .flat_map(|tt| match tt {
                TokenTree::Ident(ref i) if i.to_string() == special.to_string() => None,
                TokenTree::Group(g) => Some(TokenTree::Group(Group::new(
                    g.delimiter(),
                    replace(g.stream(), special.clone()),
                ))),
                other => Some(other),
            })
            .collect()
    }

    replace(input, special)
    // let input = parse_macro_input!(input as Item);

    // if let Item::Fn(mut f) = input {
    //     quote::quote! {
    //         f.block
    //         #f
    //     }
    // } else {
    //     panic!("mulitest must be applied to a function");
    // }
    // .into()
    // let (ident, variants) = match &input {
    //     Item::Enum(ItemEnum {
    //         ident, variants, ..
    //     }) => (ident, variants),
    //     _ => abort!(input, "hdk_entry_def_conversions can only be used on Enums"),
    // };

    // let inner: proc_macro2::TokenStream = variants
    //     .into_iter()
    //     .map(
    //         |syn::Variant {
    //              ident: v_ident,
    //              fields,
    //              ..
    //          }| {
    //             get_single_tuple_variant(v_ident, fields);
    //             quote::quote! {#ident::#v_ident (v) => SerializedBytes::try_from(v),}
    //         },
    //     )
    //     .collect();
    // let try_from_sb: proc_macro2::TokenStream = quote::quote! {
    //     let result = match t {
    //         #inner
    //     };
    // };

    // let output = quote::quote! {
    //     #input

    //     impl TryFrom<&#ident> for AppEntryBytes {
    //         type Error = WasmError;
    //         fn try_from(t: &#ident) -> Result<Self, Self::Error> {
    //             #try_from_sb
    //             AppEntryBytes::try_from(result.map_err(|e| wasm_error!(e.into()))?).map_err(|entry_error| match entry_error {
    //                 EntryError::SerializedBytes(serialized_bytes_error) => {
    //                     wasm_error!(WasmErrorInner::Serialize(serialized_bytes_error))
    //                 }
    //                 EntryError::EntryTooLarge(_) => {
    //                     wasm_error!(WasmErrorInner::Guest(entry_error.to_string()))
    //                 }
    //             })
    //         }
    //     }
    //     impl TryFrom<#ident> for AppEntryBytes {
    //         type Error = WasmError;
    //         fn try_from(t: #ident) -> Result<Self, Self::Error> {
    //             Self::try_from(&t)
    //         }
    //     }

    //     impl TryFrom<&#ident> for Entry {
    //         type Error = WasmError;
    //         fn try_from(t: &#ident) -> Result<Self, Self::Error> {
    //             Ok(Self::App(AppEntryBytes::try_from(t)?))
    //         }
    //     }

    //     impl TryFrom<#ident> for Entry {
    //         type Error = WasmError;
    //         fn try_from(t: #ident) -> Result<Self, Self::Error> {
    //             Self::try_from(&t)
    //         }
    //     }

    // };
    // output.into()
}
