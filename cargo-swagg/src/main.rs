use inflections::Inflect;
use openapiv3::OpenAPI;
use quote::{format_ident, quote};
use regex::Regex;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let file_path = "/Users/sergeysova/Projects/authmenow/backend/swagger.yaml";
    let content = fs::read_to_string(&file_path)?;
    let api: OpenAPI = serde_yaml::from_str(&content)?;
    // println!("{:#?}", api);

    // let tokens = quote! {
    //     struct #name {
    //         #prop: #atype
    //     }

    //     fn main() {
    //         let _res = <#name>::#prop();
    //     }
    // };

    let api: ApiStruct = api.info.into();

    let m1 = BindApiMethod {
        method: HttpMethod::Get,
        name: "sessionGet".to_owned(),
        path: "/session".to_owned(),
        response_type: "sessionGetResponse".to_owned(),
    };

    let m2 = BindApiMethod {
        method: HttpMethod::Post,
        name: "sessionCreate".to_owned(),
        path: "/session".to_owned(),
        response_type: "sessionCreateResponse".to_owned(),
    };

    let methods = ImplApiMethods {
        api_name: api.api_name.clone(),
        methods: vec![m1, m2],
    };

    let api_module = ApiModule { api, methods };

    println!("{}", api_module.print());

    Ok(())
}

trait Printable {
    fn print(&self) -> proc_macro2::TokenStream;
}

/// Create PascalName from string
fn to_struct_name(string: String) -> String {
    let re_name = Regex::new(r"[^\w_\-\d]+").expect("re_name invalid regex");

    re_name
        .replace_all(string.to_pascal_case().as_ref(), "")
        .to_string()
}

/// Object describing main api structure and useful impls
pub struct ApiStruct {
    pub(crate) api_name: String,
    pub(crate) terms_of_service: Option<String>,
    pub(crate) description: Option<String>,
}

impl ApiStruct {
    pub fn new(api_name: String) -> Self {
        Self {
            api_name,
            terms_of_service: None,
            description: None,
        }
    }
}

impl From<openapiv3::Info> for ApiStruct {
    fn from(info: openapiv3::Info) -> Self {
        Self {
            api_name: to_struct_name(info.title),
            description: info.description,
            terms_of_service: info.terms_of_service,
        }
    }
}

impl Printable for ApiStruct {
    fn print(&self) -> proc_macro2::TokenStream {
        let api_name = format_ident!("{}", to_struct_name(self.api_name.to_owned()));
        let terms = self
            .terms_of_service
            .to_owned()
            .map_or(String::default(), |terms| format!("@see {}", terms));
        let description = self.description.to_owned().unwrap_or_default();

        let doc_comment = format!("{}\n{}", description, terms);
        let doc = doc_comment.trim();

        quote! {
            #[doc = #doc]
            pub struct #api_name {
                api: actix_swagger::Api,
            }

            impl #api_name {
                pub fn new() -> Self {
                    Self {
                        api: actix_swagger::Api::new()
                    }
                }
            }

            impl Default for #api_name {
                fn default() -> Self {
                    let api = Self::new();
                    api
                }
            }

            impl actix_web::dev::HttpServiceFactory for #api_name {
                fn register(mut self, config: &mut actix_web::dev::AppService) {
                    self.api.register(config);
                }
            }
        }
    }
}

enum HttpMethod {
    Get,
    Post,
    Patch,
    Put,
    Delete,
}

impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Delete => "DELETE",
        }
        .to_owned()
    }
}

struct BindApiMethod {
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

struct ImplApiMethods {
    api_name: String,
    methods: Vec<BindApiMethod>,
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

struct ApiModule {
    api: ApiStruct,
    methods: ImplApiMethods,
}

impl Printable for ApiModule {
    fn print(&self) -> proc_macro2::TokenStream {
        let struct_tokens = self.api.print();
        let methods_tokens = self.methods.print();
        quote! {
            pub mod api {
                #struct_tokens

                #methods_tokens
            }
        }
    }
}
