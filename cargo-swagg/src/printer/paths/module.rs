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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::shot;
    use insta::assert_snapshot;

    #[test]
    fn components_module_default() {
        assert_snapshot!(shot(PathsModule::default()), @"pub mod paths {}");
    }
}
