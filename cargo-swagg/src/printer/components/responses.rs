pub use module::*;

pub mod module {
    use super::super::Component;
    use crate::printer::Printable;
    use quote::quote;

    pub struct ResponsesModule {
        pub components: Vec<Component>,
    }

    impl Printable for ResponsesModule {
        fn print(&self) -> proc_macro2::TokenStream {
            quote! {
                pub mod responses {}
            }
        }
    }
}
