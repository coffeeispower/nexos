use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, ItemFn, Visibility};

#[proc_macro_attribute]
pub fn test(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the function definition
    let input = parse_macro_input!(input as ItemFn);
    let input_block = input.block.clone();

    // Extract function name
    let test_name = &input.sig.ident;
    if !matches!(input.vis, Visibility::Inherited) {
        panic!("test functions should be private");
    }
    let output = ItemFn {
        block: Box::new(parse_quote! {
            {
    
                {
                    let module_path = module_path!();
                    if module_path.is_empty(){
                        print!("test {}::{}...", module_path, stringify!(#test_name));
                    } else {
                        print!("test {}...", stringify!(#test_name));
                    }
                }
                #input_block
                println!("\x1B[1;32mok\x1B[1;0m");
            }
        }),
        attrs: {
            let mut attrs = input.attrs.clone();
            attrs.push(parse_quote!(#[test_case]));
            attrs
        },
        ..input.clone()
    };
    TokenStream::from(quote!(#output))
}
