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
