use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[derive(FromMeta, Default)]
struct TestArgs {
    #[darling(default)]
    name: Option<String>,
    #[darling(default)]
    ignore: bool,
}

#[proc_macro_attribute]
pub fn test(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(Error::from(e).write_errors()); }
    };
    let test_args = match TestArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };
    let test_body = input_fn.block;

    let fn_name = &input_fn.sig.ident;
    let test_name = test_args.name.clone().map_or_else(|| quote!{ stringify!(#fn_name) }, |name| quote! { #name });
    let is_ignored = test_args.ignore;
    let expanded = if is_ignored {
        quote! {
            #[test_case]
            fn #fn_name() {
                println!("\x1B[1;33mIGNORED\x1B[1;0m TEST {} > {}", file!(), #test_name);
            }
        }
    } else {
        quote! {
            #[test_case]
            fn #fn_name() {
                println!("\n\x1B[1;34mSTART\x1B[1;0m TEST {} > {}", file!(), #test_name);
                #test_body
                println!("\x1B[1;32mPASS\x1B[1;0m TEST {} > {}", file!(), #test_name);
            }
        }
    };

    TokenStream::from(expanded)
}
