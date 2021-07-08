mod args;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn validate_schema(args: TokenStream, input: TokenStream) -> TokenStream {

}