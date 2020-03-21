use super::Path;
use crate::printer::Printable;
use quote::quote;

pub struct PathsModule {
    pub paths: Vec<Path>,
}

impl Printable for PathsModule {
    fn print(&self) -> proc_macro2::TokenStream {
        let mut tokens = quote! {};

        for path in &self.paths {
            let printed = path.print();

            tokens = quote! {
                #tokens
                #printed
            };
        }

        quote! {
            pub mod paths {
                #tokens
            }
        }
    }
}
