use crate::args::SwaggerArgs;
use syn::ItemFn;
use proc_macro2::TokenStream;
use quote::quote;
use openapiv3::{OpenAPI, PathItem};

pub fn expand(input: ItemFn, args: SwaggerArgs, data: &OpenAPI, path: &PathItem) -> TokenStream {
    quote! {
        use ::actix_web::get;

        #[get("/")]
        #input
    }
}
