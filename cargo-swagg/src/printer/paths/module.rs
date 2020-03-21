use super::Path;
use crate::printer::Printable;
use quote::quote;

#[derive(Default)]
pub struct PathsModule {
    pub paths: Vec<Path>,
}

impl Printable for PathsModule {
    fn print(&self) -> proc_macro2::TokenStream {
        let paths = self.paths.print();

        quote! {
            pub mod paths {
                #paths
            }
        }
    }
}
