extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::{ToTokens,quote};

#[proc_macro_attribute]
pub fn print_ast(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree (DeriveInput)
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct
    let struct_name = &input.ident;

    // Print the AST
    eprintln!("AST for struct '{}':\n\n{:#?}", struct_name, input);


    input.into_token_stream().into()
}

#[proc_macro_derive(HelloMacro)]
pub fn derive_impl(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

// fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
//     let name = &ast.ident;
//     let gen = quote! {
//         impl HelloMacro for #name {
//             fn hello_macro() {
//                 println!("Hello, Macro! My name is {}!", stringify!(#name));
//             }
//         }
//     };
//     gen.into()
// }

fn impl_hello_macro<'a>(ast: &'a syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl<'a> HelloMacro for #name<'a> {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}