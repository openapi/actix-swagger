use super::methods::ImplApiMethods;
use super::structure::ApiStruct;
use crate::printer::Printable;
use quote::quote;

#[derive(Default)]
pub struct ApiModule {
    pub api: ApiStruct,
    pub methods: ImplApiMethods,
}

impl Printable for ApiModule {
    fn print(&self) -> proc_macro2::TokenStream {
        let api_struct_impl = self.api.print();
        let methods_impl = self.methods.print();
        quote! {
            pub mod api {
                #api_struct_impl

                #methods_impl
            }
        }
    }
}
