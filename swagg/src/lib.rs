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
        // for (name, body) in components.request_bodies.iter() {
        //     match body {
        //         ReferenceOr::Item(body) => {
        //             highway_components.parse_request_body(&name, &body);
        //         }
        //         ReferenceOr::Reference { reference } => {
        //             log::info!("skipping request body reference {}", reference);
        //         }
        //     }
        // }

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
