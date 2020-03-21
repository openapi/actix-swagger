pub use module::*;

pub mod module {
    use crate::printer::Printable;
    use quote::quote;
    pub struct RequestBodiesModule {}

    impl Printable for RequestBodiesModule {
        fn print(&self) -> proc_macro2::TokenStream {
            quote! {
                pub mod request_bodies {}
            }
        }
    }
}
