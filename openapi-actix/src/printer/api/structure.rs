use crate::printer::Printable;
use inflections::Inflect;
use quote::{format_ident, quote};
use regex::Regex;

/// Create PascalName from string
pub fn to_struct_name(string: String) -> String {
    let re_name = Regex::new(r"[^\w_\-\d]+").expect("re_name invalid regex");

    re_name
        .replace_all(string.to_pascal_case().as_ref(), "")
        .to_string()
}

/// Object describing main api structure and useful impls
pub struct ApiStruct {
    pub api_name: String,
    pub terms_of_service: Option<String>,
    pub description: Option<String>,
    // pub license: Option<String>,
    // pub contact: Option<String>,
    // pub version: semver::Version,
}

impl Default for ApiStruct {
    fn default() -> Self {
        Self {
            api_name: "Api".to_owned(),
            terms_of_service: None,
            description: None,
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

        let doc_stream = match doc.len() > 0 {
            true => quote! { #[doc = #doc] },
            false => quote! {},
        };

        quote! {
            #doc_stream
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
                fn register(self, config: &mut actix_web::dev::AppService) {
                    self.api.register(config);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::shot;
    use insta::assert_snapshot;

    #[test]
    fn default_struct() {
        assert_snapshot!(shot(ApiStruct::default()), @r###"
        pub struct Api {
            api: actix_swagger::Api,
        }
        impl Api {
            pub fn new() -> Self {
                Self {
                    api: actix_swagger::Api::new(),
                }
            }
        }
        impl Default for Api {
            fn default() -> Self {
                let api = Self::new();
                api
            }
        }
        impl actix_web::dev::HttpServiceFactory for Api {
            fn register(self, config: &mut actix_web::dev::AppService) {
                self.api.register(config);
            }
        }
        "###);
    }

    #[test]
    fn with_terms() {
        assert_snapshot!(shot(ApiStruct {
            api_name: "test_api".to_owned(),
            description: None,
            terms_of_service: Some("https://example.com/terms".to_owned())
        }), @r###"
        #[doc = "@see https://example.com/terms"]
        pub struct TestApi {
            api: actix_swagger::Api,
        }
        impl TestApi {
            pub fn new() -> Self {
                Self {
                    api: actix_swagger::Api::new(),
                }
            }
        }
        impl Default for TestApi {
            fn default() -> Self {
                let api = Self::new();
                api
            }
        }
        impl actix_web::dev::HttpServiceFactory for TestApi {
            fn register(self, config: &mut actix_web::dev::AppService) {
                self.api.register(config);
            }
        }
        "###);
    }

    #[test]
    fn with_description() {
        assert_snapshot!(shot(ApiStruct {
            api_name: "test_api".to_owned(),
            description: Some("My super simple description.\nAnother back".to_owned()),
            terms_of_service: None,
        }), @r###"
        #[doc = "My super simple description.\nAnother back"]
        pub struct TestApi {
            api: actix_swagger::Api,
        }
        impl TestApi {
            pub fn new() -> Self {
                Self {
                    api: actix_swagger::Api::new(),
                }
            }
        }
        impl Default for TestApi {
            fn default() -> Self {
                let api = Self::new();
                api
            }
        }
        impl actix_web::dev::HttpServiceFactory for TestApi {
            fn register(self, config: &mut actix_web::dev::AppService) {
                self.api.register(config);
            }
        }
        "###);
    }

    #[test]
    fn with_description_and_terms() {
        assert_snapshot!(shot(ApiStruct {
            api_name: "test_api".to_owned(),
            description: Some("My super simple description.\nAnother back".to_owned()),
            terms_of_service: Some("https://example.com/terms".to_owned()),
        }), @r###"
        #[doc = "My super simple description.\nAnother back\n@see https://example.com/terms"]
        pub struct TestApi {
            api: actix_swagger::Api,
        }
        impl TestApi {
            pub fn new() -> Self {
                Self {
                    api: actix_swagger::Api::new(),
                }
            }
        }
        impl Default for TestApi {
            fn default() -> Self {
                let api = Self::new();
                api
            }
        }
        impl actix_web::dev::HttpServiceFactory for TestApi {
            fn register(self, config: &mut actix_web::dev::AppService) {
                self.api.register(config);
            }
        }
        "###);
    }
}
