use crate::args::SwaggerArgs;
use syn::{ItemFn, Stmt, FnArg};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use openapiv3::{OpenAPI, PathItem};


pub fn expand(mut input: ItemFn, args: SwaggerArgs, data: &OpenAPI, path: &PathItem) -> TokenStream {
    let (last_stmt_start_span, last_stmt_end_span) = {
        let mut last_stmt = input
            .block
            .stmts
            .last()
            .map(ToTokens::into_token_stream)
            .unwrap_or_default()
            .into_iter();
        let start = last_stmt.next().map_or_else(Span::call_site, |t| t.span());
        let end = last_stmt.last().map_or(start, |t| t.span());
        (start, end)
    };

    quote! {
        use ::actix_web::get;

        #[get("/")]
        #input
    }
}
