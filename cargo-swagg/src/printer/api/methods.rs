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
    pub(crate) method: HttpMethod,
    pub(crate) path: String,
    pub(crate) name: String,
    pub(crate) response_type: String,
}

impl Printable for BindApiMethod {
    fn print(&self) -> proc_macro2::TokenStream {
        let request_path = self.path.clone();
        let http_method = format_ident!("{}", self.method.to_string());
        let response_type = format_ident!("{}", self.response_type.to_pascal_case());
        let bind_method_name = format_ident!("bind_{}", self.name.to_snake_case());

        quote! {
            pub fn #bind_method_name<F, T, R>(mut self, handler: F) -> Self
            where
                F: actix_web::dev::Factory<T, R, actix_swagger::Answer<'static, super::paths::#response_type>>,
                T: actix_web::FromRequest + 'static,
                R: std::future::Future<Output = actix_swagger::Answer<'static, super::paths::#response_type>> + 'static,
            {
                self.api = self.api.bind(#request_path.to_owned(), actix_web::http::Method::#http_method, handler);
                self
            }
        }
    }
}

pub struct ImplApiMethods {
    pub api_name: String,
    pub methods: Vec<BindApiMethod>,
}

impl Printable for ImplApiMethods {
    fn print(&self) -> proc_macro2::TokenStream {
        let api_name = format_ident!("{}", to_struct_name(self.api_name.to_owned()));
        let mut tokens = quote! {};

        for method in &self.methods {
            let method_tokens = method.print();
            tokens = quote! {
                #tokens

                #method_tokens
            };
        }
        quote! {
            impl #api_name {
                #tokens
            }
        }
    }
}
