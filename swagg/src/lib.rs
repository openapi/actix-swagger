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

    let mut generated = printer::GeneratedModule::default();
    let mut highway_components = highway::Components::new();

    generated.api.set_name(api.info.title);
    generated.api.set_description(api.info.description);
    generated
        .api
        .set_terms_of_service(api.info.terms_of_service);

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
            match parse_ref_or_schema(&name, &schema) {
                Ok((field, components)) => {
                    println!("Schema: {} {:#?}", name, field);
                    println!("Parsed components {:#?}", components);
                }
                Err(reason) => {
                    eprintln!("Failed {} {:#?}", name, reason);
                }
            }
        }
    }

    println!("{:#?}", highway_components);

    Ok(format!("{}", generated.print()))
}

#[derive(Debug)]
enum ParseSchemaError {
    UnsupportedType,
    ReferenceNotSupported,
}

fn parse_ref_or_schema(
    name: &String,
    ref_or: &ReferenceOr<openapiv3::Schema>,
) -> Result<(highway::FieldType, Vec<highway::Component>), ParseSchemaError> {
    match ref_or {
        ReferenceOr::Item(schema) => parse_schema(&name, &schema),
        ReferenceOr::Reference { reference } => {
            log::info!(
                "reference for schemas is not supported yet. Skipping {} for {}...",
                reference,
                name
            );

            Err(ParseSchemaError::ReferenceNotSupported)
        }
    }
}

fn parse_schema(
    name: &String,
    schema: &openapiv3::Schema,
) -> Result<(highway::FieldType, Vec<highway::Component>), ParseSchemaError> {
    use inflections::Inflect;

    let mut list = vec![];

    use openapiv3::{SchemaKind, Type};

    match &schema.schema_kind {
        SchemaKind::Type(schema_type) => {
            let field_type = match schema_type {
                Type::Number(_number) => highway::FieldType::Number,
                Type::Integer(_integer) => highway::FieldType::Integer,
                Type::String(_string) => highway::FieldType::String,
                Type::Boolean {} => highway::FieldType::Boolean,
                Type::Object(object) => {
                    let (fields, mut created_components) = parse_schema_object(&name, object)?;

                    let component_name = name.clone().to_pascal_case();
                    let component = highway::Component {
                        name: component_name.clone(),
                        description: schema.schema_data.description.clone(),
                        kind: highway::ComponentKind::Object { fields },
                    };

                    list.push(component);
                    list.append(&mut created_components);

                    highway::FieldType::Type(component_name.clone())
                }
                Type::Array(_array) => unimplemented!(),
            };

            Ok((field_type, list))
        }
        other => {
            log::info!("this schema kind is not supported {:?}", other);
            Err(ParseSchemaError::UnsupportedType)
        }
    }
}

fn parse_schema_object(
    name: &String,
    schema_object: &openapiv3::ObjectType,
) -> Result<
    (
        indexmap::IndexMap<String, highway::ComponentField>,
        Vec<highway::Component>,
    ),
    ParseSchemaError,
> {
    use inflections::Inflect;

    let mut components = vec![];
    let mut fields = indexmap::IndexMap::new();

    for (field_name, schema) in schema_object.properties.iter() {
        let inner_name = format!("{}{}", name, field_name.clone().to_pascal_case());

        let (field_type, mut created_components) =
            parse_ref_or_schema(&inner_name, &schema.clone().unbox())?;

        components.append(&mut created_components);

        let field = highway::ComponentField {
            required: schema_object
                .required
                .iter()
                .find(|found| *found == field_name)
                .is_some(),
            description: None, // TODO: parse field description
            field_type,
        };

        fields.insert(field_name.clone(), field);
    }

    Ok((fields, components))
}
