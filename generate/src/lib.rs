extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::ToTokens;

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