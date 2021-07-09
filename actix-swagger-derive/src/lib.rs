mod args;
mod expand;
mod input;
mod meta;

extern crate proc_macro;

use crate::meta::METADATA;
use args::SwaggerArgs;
use convert_case::{Case, Casing};
use once_cell::sync::Lazy;
use openapiv3::{OpenAPI, PathItem, Paths, ReferenceOr};
use proc_macro::TokenStream;
use proc_macro2::Span;
use std::io::BufReader;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn swagger(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as SwaggerArgs);
    let input = syn::parse_macro_input!(input as ItemFn);

    match parse_schema() {
        Ok(data) => {
            let operation_id = args.operation_id.as_ref().map(|i| i.value()).unwrap_or_else(|| {
                let ident = input.sig.ident.to_string();
                ident.to_case(Case::Camel)
            });
            if let Some(path) = find_by_operation_id(&data.paths, &operation_id) {
                expand::expand(input, args, &data, path).into()
            } else {
                syn::Error::new(
                    Span::mixed_site(),
                    format!("Could not find operationId {} in schema", operation_id),
                )
                .to_compile_error()
                .into()
            }
        }
        Err(e) => syn::Error::new(Span::mixed_site(), e)
            .to_compile_error()
            .into(),
    }
}

fn find_by_operation_id<'a>(paths: &'a Paths, operation_id: &'a str) -> Option<&'a PathItem> {
    if !paths.is_empty() {
        for path in paths.values() {
            use openapiv3::ReferenceOr::*;
            let path: &ReferenceOr<PathItem> = path;

            match path {
                Item(path) => {
                    let found = path
                        .iter()
                        .flat_map(|r| r.operation_id.as_ref())
                        .enumerate()
                        .find(|(i, r)| *r == operation_id);

                    if found.is_none() {
                        continue;
                    }

                    return Some(path);
                }
                _ => {}
            }
        }
    }
    None
}
