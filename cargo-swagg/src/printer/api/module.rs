use super::methods::ImplApi;
use super::structure::ApiStruct;
use crate::printer::Printable;
use quote::quote;

#[derive(Default)]
pub struct ApiModule {
    pub api: ApiStruct,
    pub methods: ImplApi,
}

impl Printable for ApiModule {
    fn print(&self) -> proc_macro2::TokenStream {
        let api_struct = self.api.print();
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
    use insta::assert_yaml_snapshot;

    #[test]
    fn default_api_module() {
        assert_yaml_snapshot!(shot(ApiModule::default()), @r###"
        ---
        - "  pub mod api {                                                                                                           "
        - "      pub struct Api {                                                                                                    "
        - "          api: actix_swagger::Api,                                                                                        "
        - "      }                                                                                                                   "
        - "      impl Api {                                                                                                          "
        - "          pub fn new() -> Self {                                                                                          "
        - "              Self {                                                                                                      "
        - "                  api: actix_swagger::Api::new(),                                                                         "
        - "              }                                                                                                           "
        - "          }                                                                                                               "
        - "      }                                                                                                                   "
        - "      impl Default for Api {                                                                                              "
        - "          fn default() -> Self {                                                                                          "
        - "              let api = Self::new();                                                                                      "
        - "              api                                                                                                         "
        - "          }                                                                                                               "
        - "      }                                                                                                                   "
        - "      impl actix_web::dev::HttpServiceFactory for Api {                                                                   "
        - "          fn register(mut self, config: &mut actix_web::dev::AppService) {                                                "
        - "              self.api.register(config);                                                                                  "
        - "          }                                                                                                               "
        - "      }                                                                                                                   "
        - "      use super::paths;                                                                                                   "
        - "      use actix_swagger::{Answer, Method};                                                                                "
        - "      use actix_web::{dev::Factory, FromRequest};                                                                         "
        - "      use std::future::Future;                                                                                            "
        - "      impl Api {}                                                                                                         "
        - "  }                                                                                                                       "
        - "                                                                                                                          "
        "###);
    }
}
