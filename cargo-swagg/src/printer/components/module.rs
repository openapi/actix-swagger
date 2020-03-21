use super::request_bodies::RequestBodiesModule;
use super::responses::ResponsesModule;
use crate::printer::Printable;
use quote::quote;

pub struct ComponentsModule {
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
