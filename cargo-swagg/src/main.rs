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

    // let name = format_ident!("{}", "hello-quote".to_pascal_case());
    // let prop = format_ident!("{}", "my-super-method".to_snake_case());
    // let sub = format_ident!("String");
    // let atype = quote! { Option<#sub> };

    // let tokens = quote! {
    //     struct #name {
    //         #prop: #atype
    //     }

    //     fn main() {
    //         let _res = <#name>::#prop();
    //     }
    // };

    let tokens = api.info.print();

    println!("{}", tokens);

    Ok(())
}

trait Printable {
    fn print(&self) -> proc_macro2::TokenStream;
}

impl Printable for openapiv3::Info {
    fn print(&self) -> proc_macro2::TokenStream {
        let api_name = format_ident!("{}", to_struct_name(self.title.to_owned()));
        let terms = self
            .terms_of_service
            .to_owned()
            .map_or(String::default(), |terms| format!("@see {}", terms));
        let description = self.description.to_owned().unwrap_or_default();

        let doc_comment = format!("{}\n{}", description, terms);

        quote! {
            #[doc = #doc_comment]
            pub struct #api_name {
                root: actix_web::Scope,
                routes: std::collections::HashMap<String, Scope>
            }

            impl #api_name {
                pub fn new() -> Self {
                    Self {
                        root: actix_web::Scope:: new("/"),
                        routes: std::collections::HashMap::new(),
                    }
                }
            }

            impl actix_web::dev::HttpServiceFactory for #api_name {
                fn register(mut self, config: &mut actix_web::dev::AppService) {
                    let keys: Vec<String> = self.routes.keys().cloned().collect();

                    for key in keys.iter() {
                        if let Some(route) = self.routes.remove(key) {
                            self.root = self.root.service(resource);
                        }
                    }

                    self.root.register(config);
                }
            }
        }
    }
}

impl Printable for OpenAPI {
    fn print(&self) -> proc_macro2::TokenStream {
        let api_struct = self.info.print();
        // let api_name = format_ident!("{}", to_struct_name(self.info.title.to_owned()));

        quote! {
            #api_struct
        }
    }
}

/// Create PascalName from string
fn to_struct_name(string: String) -> String {
    let re_name = Regex::new(r"[^\w_\-\d]+").expect("re_name invalid regex");

    re_name
        .replace_all(string.to_pascal_case().as_ref(), "")
        .to_string()
}
