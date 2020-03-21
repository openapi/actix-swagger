pub use module::*;

pub mod module {
    use super::super::Component;
    use crate::printer::Printable;
    use quote::quote;
    pub struct RequestBodiesModule {
        pub list: Vec<Component>,
    }

    impl Printable for RequestBodiesModule {
        fn print(&self) -> proc_macro2::TokenStream {
            let components = self.list.print();

            quote! {
                pub mod request_bodies {
                    use serde::{Serialize, Deserialize};

                    #components
                }
            }
        }
    }
}
