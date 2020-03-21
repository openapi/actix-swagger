pub use module::*;

pub mod module {
    use super::super::Component;
    use crate::printer::Printable;
    use quote::quote;

    #[derive(Default)]
    pub struct ResponsesModule {
        pub list: Vec<Component>,
    }

    impl Printable for ResponsesModule {
        fn print(&self) -> proc_macro2::TokenStream {
            let components = self.list.print();

            quote! {
                pub mod responses {
                    use serde::{Serialize, Deserialize};

                    #components
                }
            }
        }
    }
}
