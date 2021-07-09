use openapiv3::{OpenAPI, ReferenceOr};

mod highway;
mod printer;

#[cfg(test)]
pub mod test;

use printer::Printable;

/// Format for OpenAPI3 specification
pub enum Format {
    Yaml,
    Json,
}

/// Describes convertation error
#[derive(Debug)]
pub enum Error {
    InvalidSource,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSource => write!(f, "OpenAPI structure cannot be parsed"),
        }
    }
}

impl std::error::Error for Error {}

/// Convert source of OpenAPI3 specification to rust code in string representation
pub fn to_string(source: &str, format: Format) -> Result<String, Error> {
    let api: OpenAPI = match format {
        Format::Yaml => serde_yaml::from_str(&source).map_err(|_| Error::InvalidSource)?,
        Format::Json => serde_json::from_str(&source).map_err(|_| Error::InvalidSource)?,
    };

    // eprintln!("{:#?}", api.components);

    let mut highway_components = highway::Components::new();

    if let Some(components) = api.components {
        for (name, body) in components.request_bodies.iter() {
            match body {
                ReferenceOr::Item(body) => {
                    highway_components.parse_request_body(&name, &body);
                }
                ReferenceOr::Reference { reference } => {
                    log::info!("skipping request body reference {}", reference);
                }
            }
        }

        for (name, schema) in components.schemas.iter() {
            if let Err(reason) = highway_components.parse_schema(&name, &schema) {
                eprintln!("Failed {} {:#?}", name, reason);
            }
        }
    }

    println!("{:#?}", highway_components);

    let mut generated: printer::GeneratedModule = highway_components.into();

    generated.api.set_name(api.info.title);
    generated.api.set_description(api.info.description);
    generated
        .api
        .set_terms_of_service(api.info.terms_of_service);

    Ok(format!("{}", generated.print()))
}

#[cfg(test)]
mod tests {
    use super::{to_string, Format};
    use crate::test::pretty;
    use insta::assert_snapshot;

    #[test]
    fn yaml_schema_prints() {
        let schema = r###"
openapi: 3.0.1
info:
  title: Demo API.
  version: 0.1.0
  description: Test api
paths:
  "/stub":
    get:
      operationId: stub
      responses:
        303:
          description: "Stub"

components:
  schemas:
    SessionUser:
      description: Current user in a session
      type: object
      required:
        - firstName
        - lastName
      properties:
        firstName:
          type: string
        lastName:
          type: string
        inner:
          type: object
          properties:
            foo:
              type: number
            bar:
              type: integer
            baz:
              type: object
              properties:
                demo:
                  type: string
          required:
            - baz
            - bar
        "###;

        assert_snapshot!(pretty(to_string(&schema, Format::Yaml).unwrap()), @r###"
        #![allow(dead_code, unused_imports)]
        pub mod api {
            #[doc = "Test api"]
            pub struct DemoApi {
                api: actix_swagger::Api,
            }
            impl DemoApi {
                pub fn new() -> Self {
                    Self {
                        api: actix_swagger::Api::new(),
                    }
                }
            }
            impl Default for DemoApi {
                fn default() -> Self {
                    let api = Self::new();
                    api
                }
            }
            impl actix_web::dev::HttpServiceFactory for DemoApi {
                fn register(self, config: &mut actix_web::dev::AppService) {
                    self.api.register(config);
                }
            }
            use super::paths;
            use actix_swagger::{Answer, Method};
            use actix_web::FromRequest;
            use std::future::Future;
            impl DemoApi {}
        }
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
                #[doc = "Current user in a session"]
                #[derive(Debug, Serialize, Deserialize)]
                pub struct SessionUser {
                    #[serde(rename = "firstName")]
                    pub first_name: String,
                    #[serde(rename = "lastName")]
                    pub last_name: String,
                    pub inner: Option<SessionUserInner>,
                }
                #[derive(Debug, Serialize, Deserialize)]
                pub struct SessionUserInner {
                    pub foo: Option<f32>,
                    pub bar: i32,
                    pub baz: SessionUserInnerBaz,
                }
                #[derive(Debug, Serialize, Deserialize)]
                pub struct SessionUserInnerBaz {
                    pub demo: Option<String>,
                }
            }
        }
        pub mod paths {
            use super::components::{parameters, responses};
        }
        "###);
    }
}
