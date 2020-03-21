pub use module::*;

pub mod module {
    use super::super::Component;
    use crate::printer::Printable;
    use quote::quote;

    pub struct ResponsesModule {
        pub list: Vec<Component>,
    }

    impl Printable for ResponsesModule {
        fn print(&self) -> proc_macro2::TokenStream {
            let mut components = quote! {};

            for component in &self.list {
                let printed = component.print();

                components = quote! {
                    #components

                    #printed
                }
            }

            quote! {
                pub mod responses {
                    use serde::{Serialize, Deserialize};

                    #components
                }
            }
        }
    }
}
