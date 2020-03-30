use super::methods::ImplApi;
use super::structure::ApiStruct;
use crate::printer::Printable;
use quote::quote;

#[derive(Default)]
pub struct ApiModule {
    pub structure: ApiStruct,
    pub methods: ImplApi,
}

impl ApiModule {
    pub fn set_name(&mut self, name: String) {
        self.structure.api_name = name.clone();
        self.methods.api_name = name;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.structure.description = description;
    }

    pub fn set_terms_of_service(&mut self, terms: Option<String>) {
        self.structure.terms_of_service = terms;
    }
}

impl Printable for ApiModule {
    fn print(&self) -> proc_macro2::TokenStream {
        let api_struct = self.structure.print();
        let methods_impl = self.methods.print();
        quote! {
            pub mod api {
                #api_struct

                #methods_impl
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
    fn default_api_module() {
        assert_snapshot!(shot(ApiModule::default()), @r###"
        pub mod api {
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
            use super::paths;
            use actix_swagger::{Answer, Method};
            use actix_web::{dev::Factory, FromRequest};
            use std::future::Future;
            impl Api {}
        }
        "###);
    }
}
