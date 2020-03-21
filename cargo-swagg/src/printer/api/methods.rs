use super::structure::to_struct_name;
use crate::printer::Printable;
use inflections::Inflect;
use quote::{format_ident, quote};

pub enum HttpMethod {
    Delete,
    Get,
    Patch,
    Post,
    Put,
}

impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        match self {
            HttpMethod::Delete => "DELETE",
            HttpMethod::Get => "GET",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
        }
        .to_owned()
    }
}

pub struct BindApiMethod {
    pub method: HttpMethod,
    pub path: String,
    pub name: String,
}

impl Printable for BindApiMethod {
    fn print(&self) -> proc_macro2::TokenStream {
        let request_path = self.path.clone();
        let http_method = format_ident!("{}", self.method.to_string());
        let path_name = format_ident!("{}", self.name.to_snake_case());
        let bind_method_name = format_ident!("bind_{}", self.name.to_snake_case());

        quote! {
            pub fn #bind_method_name<F, T, R>(mut self, handler: F) -> Self
            where
                F: Factory<T, R, Answer<'static, paths::#path_name::Response>>,
                T: FromRequest + 'static,
                R: Future<Output = Answer<'static, paths::#path_name::Response>> + 'static,
            {
                self.api = self.api.bind(#request_path.to_owned(), #http_method, handler);
                self
            }
        }
    }
}

#[derive(Default)]
pub struct ImplApiMethods {
    pub api_name: String,
    pub methods: Vec<BindApiMethod>,
}

impl Printable for ImplApiMethods {
    fn print(&self) -> proc_macro2::TokenStream {
        let api_name = format_ident!("{}", to_struct_name(self.api_name.to_owned()));
        let methods = self.methods.print();

        quote! {
            use actix_web::{FromRequest, dev::Factory};
            use actix_swagger::{Answer, Method};
            use std::future::Future;
            use super::paths;

            impl #api_name {
                #methods
            }
        }
    }
}
