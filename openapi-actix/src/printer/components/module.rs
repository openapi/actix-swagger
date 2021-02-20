use super::parameters::ParametersModule;
use super::request_bodies::RequestBodiesModule;
use super::responses::ResponsesModule;
use super::schemas::SchemasModule;
use crate::printer::Printable;
use quote::quote;

#[derive(Default)]
pub struct ComponentsModule {
    pub parameters: ParametersModule,
    pub request_bodies: RequestBodiesModule,
    pub responses: ResponsesModule,
    pub schemas: SchemasModule,
}

impl Printable for ComponentsModule {
    fn print(&self) -> proc_macro2::TokenStream {
        let parameters = self.parameters.print();
        let request_bodies = self.request_bodies.print();
        let responses = self.responses.print();
        let schemas = self.schemas.print();

        quote! {
            pub mod components {
                #parameters
                #request_bodies
                #responses
                #schemas
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
        assert_snapshot!(shot(ComponentsModule::default()), @r###"
        pub mod components {
            pub mod parameters {
                use serde::{Deserialize, Serialize};
            }
            pub mod request_bodies {
                use serde::{Deserialize, Serialize};
            }
            pub mod responses {
                use serde::{Deserialize, Serialize};
            }
            pub mod schemas {
                use serde::{Deserialize, Serialize};
            }
        }
        "###);
    }
}
