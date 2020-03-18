use inflections::Inflect;
use openapiv3::OpenAPI;
use quote::{format_ident, quote};
use regex::Regex;
use std::fs;

use response_status::ResponseStatus;

mod response_status;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let file_path = "/Users/sergeysova/Projects/authmenow/backend/swagger.yaml";
    let content = fs::read_to_string(&file_path)?;
    let api: OpenAPI = serde_yaml::from_str(&content)?;

    // println!("{}", "OAuthAuthorizeRequest".to_snake_case());

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

    let m3 = BindApiMethod {
        method: HttpMethod::Post,
        name: "registerConfirmation".to_owned(),
        path: "/register/confirmation".to_owned(),
        response_type: "registerConfirmationResponse".to_owned(),
    };

    let methods = ImplApiMethods {
        api_name: api.api_name.clone(),
        methods: vec![m1, m2, m3],
    };

    let api_module = ApiModule { api, methods };

    let components_module = ComponentsModule {
        responses: ResponsesModule {},
        request_bodies: RequestBodiesModule {},
    };

    let p1 = Path {
        name: "registerConfirmation".to_owned(),
        response: ResponseEnum {
            responses: vec![
                StatusVariant {
                    status: ResponseStatus::Created,
                    response_type_name: None,
                    description: None,
                    content_type: None,
                    x_variant_name: None,
                },
                StatusVariant {
                    status: ResponseStatus::BadRequest,
                    response_type_name: Some("RegisterConfirmationFailed".to_owned()),
                    description: None,
                    content_type: Some(ContentType::Json),
                    x_variant_name: None,
                },
                StatusVariant {
                    status: ResponseStatus::InternalServerError,
                    response_type_name: None,
                    description: None,
                    content_type: Some(ContentType::Json),
                    x_variant_name: Some("Unexpected".to_owned()),
                },
            ],
        },
    };

    let p2 = Path {
        name: "sessionCreate".to_owned(),
        response: ResponseEnum {
            responses: vec![
                StatusVariant {
                    status: ResponseStatus::Created,
                    response_type_name: None,
                    description: Some("User logined, cookies writed".to_owned()),
                    content_type: None,
                    x_variant_name: None,
                },
                StatusVariant {
                    status: ResponseStatus::BadRequest,
                    response_type_name: Some("sessionCreateFailed".to_owned()),
                    description: None,
                    content_type: Some(ContentType::Json),
                    x_variant_name: None,
                },
                StatusVariant {
                    status: ResponseStatus::InternalServerError,
                    response_type_name: None,
                    description: None,
                    content_type: Some(ContentType::Json),
                    x_variant_name: Some("Unexpected".to_owned()),
                },
            ],
        },
    };

    let paths_module = PathsModule {
        paths: vec![p1, p2],
    };

    let generated_module = GeneratedModule {
        api_module,
        components_module,
        paths_module,
    };

    println!("{}", generated_module.print());

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

struct ComponentsModule {
    pub responses: ResponsesModule,
    pub request_bodies: RequestBodiesModule,
}

impl Printable for ComponentsModule {
    fn print(&self) -> proc_macro2::TokenStream {
        let responses = self.responses.print();
        let request_bodies = self.request_bodies.print();

        quote! {
            pub mod components {
                #request_bodies
                #responses
            }
        }
    }
}

struct ResponsesModule {}

impl Printable for ResponsesModule {
    fn print(&self) -> proc_macro2::TokenStream {
        quote! {
            pub mod responses {}
        }
    }
}

struct RequestBodiesModule {}

impl Printable for RequestBodiesModule {
    fn print(&self) -> proc_macro2::TokenStream {
        quote! {
            pub mod request_bodies {}
        }
    }
}

struct PathsModule {
    paths: Vec<Path>,
}

impl Printable for PathsModule {
    fn print(&self) -> proc_macro2::TokenStream {
        let mut tokens = quote! {};

        for path in &self.paths {
            let printed = path.print();

            tokens = quote! {
                #tokens
                #printed
            };
        }

        quote! {
            pub mod paths {
                #tokens
            }
        }
    }
}

struct Path {
    name: String,
    response: ResponseEnum,
}

impl Path {
    fn print_enum_variants(&self) -> proc_macro2::TokenStream {
        let mut tokens = quote! {};

        for status in &self.response.responses {
            let variant = status.print_enum_variant();

            tokens = quote! {
                #tokens
                #variant,
            };
        }

        tokens
    }

    fn print_status_variants(&self) -> proc_macro2::TokenStream {
        let mut tokens = quote! {};

        for status in &self.response.responses {
            let variant = status.print_status_variant();

            tokens = quote! {
                #tokens
                #variant,
            };
        }

        quote! {
            match self {
                #tokens
            }
        }
    }

    fn print_content_type_variants(&self) -> proc_macro2::TokenStream {
        let mut tokens = quote! {};

        for status in &self.response.responses {
            let variant = status.print_content_type_variant();

            tokens = quote! {
                #tokens
                #variant,
            }
        }

        quote! {
            match self {
                #tokens
            }
        }
    }
}

impl Printable for Path {
    fn print(&self) -> proc_macro2::TokenStream {
        let module_name = format_ident!("{}", self.name.to_snake_case());
        let enum_variants = self.print_enum_variants();
        let status_match = self.print_status_variants();
        let content_type_match = self.print_content_type_variants();

        quote! {
            pub mod #module_name {
                use super::components::responses;
                use actix_swagger::{Answer, ContentType};
                use actix_web::http::StatusCode;
                use serde::Serialize;

                #[derive(Debug, Serialize)]
                #[serde(untagged)]
                pub enum Response {
                    #enum_variants
                }

                impl Response {
                    #[inline]
                    pub fn answer(self) -> Answer<'static, Self> {
                        let status = #status_match;
                        let content_type = #content_type_match;

                        Answer::new(self).status(status).content_type(content_type)
                    }
                }
            }
        }
    }
}

struct ResponseEnum {
    responses: Vec<StatusVariant>,
}

struct StatusVariant {
    status: ResponseStatus,

    /// Should be in `#/components/responses/`
    response_type_name: Option<String>,

    /// Comment for response status
    description: Option<String>,

    /// Now supports only one content type per response
    content_type: Option<ContentType>,

    /// Variant can be renamed with `x-variant-name`
    x_variant_name: Option<String>,
}

impl StatusVariant {
    pub fn name(&self) -> proc_macro2::Ident {
        let name = self
            .x_variant_name
            .clone()
            .unwrap_or(self.status.to_string());
        format_ident!("{}", name.to_pascal_case())
    }

    pub fn description(&self) -> proc_macro2::TokenStream {
        match &self.description {
            Some(text) => quote! { #[doc = #text] },
            None => quote! {},
        }
    }

    pub fn content_type(&self) -> proc_macro2::TokenStream {
        match self.content_type.clone() {
            Some(t) => {
                let content = t.print();
                quote! { Some(ContentType::#content) }
            }
            None => quote! { None },
        }
    }

    pub fn print_enum_variant(&self) -> proc_macro2::TokenStream {
        let description = self.description();
        let variant_name = self.name();

        if let Some(response) = self.response_type_name.clone() {
            let response_name = format_ident!("{}", response);

            quote! {
                #description
                #variant_name(responses::#response_name)
            }
        } else {
            quote! {
                #description
                #variant_name
            }
        }
    }

    pub fn print_status_variant(&self) -> proc_macro2::TokenStream {
        let variant_name = self.name();
        let status = format_ident!("{}", self.status.to_string().to_constant_case());

        if let Some(_) = self.response_type_name {
            quote! {
                Self::#variant_name(_) => StatusCode::#status
            }
        } else {
            quote! {
                Self::#variant_name => StatusCode::#status
            }
        }
    }

    pub fn print_content_type_variant(&self) -> proc_macro2::TokenStream {
        let variant_name = self.name();
        let content_type = self.content_type();

        if let Some(_) = self.response_type_name {
            quote! {
                Self::#variant_name(_) => #content_type
            }
        } else {
            quote! {
                Self::#variant_name => #content_type
            }
        }
    }
}

#[derive(Debug, Clone)]
enum ContentType {
    Json,
}

impl Printable for ContentType {
    fn print(&self) -> proc_macro2::TokenStream {
        let ident = format_ident!(
            "{}",
            match self {
                Self::Json => "Json",
            }
        );

        quote! { #ident }
    }
}

struct GeneratedModule {
    pub api_module: ApiModule,
    pub components_module: ComponentsModule,
    pub paths_module: PathsModule,
}

impl Printable for GeneratedModule {
    fn print(&self) -> proc_macro2::TokenStream {
        let api_module = self.api_module.print();
        let components_module = self.components_module.print();
        let paths_module = self.paths_module.print();

        quote! {
            #api_module
            #components_module
            #paths_module
        }
    }
}
