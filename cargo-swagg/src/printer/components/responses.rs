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

#[cfg(test)]
mod tests {
    use super::super::Component;
    use super::*;
    use crate::test::shot;
    use insta::assert_snapshot;

    #[test]
    fn responses_with_some_components() {
        assert_snapshot!(shot(ResponsesModule {
            list: vec![
                Component::Enum { name: "Example".to_owned(), description: None, variants: vec![] },
                Component::Object { name: "Test".to_owned(), description: None, fields: vec![] },
            ]
        }), @r###"
        pub mod responses {
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Serialize, Deserialize)]
            pub enum Example {}
            #[derive(Debug, Serialize, Deserialize)]
            pub struct Test {}
        }
        "###);
    }
}
