pub use module::*;

pub mod module {
    use super::super::Component;
    use crate::printer::Printable;
    use quote::quote;

    #[derive(Default)]
    pub struct ParametersModule {
        pub list: Vec<Component>,
    }

    impl Printable for ParametersModule {
        fn print(&self) -> proc_macro2::TokenStream {
            let components = self.list.print();

            quote! {
                pub mod parameters {
                    use serde::{Serialize, Deserialize};

                    #components
                }
            }
        }
    }
}
