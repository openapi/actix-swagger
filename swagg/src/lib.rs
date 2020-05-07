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

        println!("{:#?}", components.schemas);

        for (name, schema) in components.schemas.iter() {
            parse_ref_or_schema(&name, &schema);
        }
    }

    println!("{:#?}", highway_components);

    Ok(format!("{}", generated.print()))
}

fn parse_ref_or_schema(
    name: &String,
    ref_or: &ReferenceOr<openapiv3::Schema>,
) -> Vec<highway::Component> {
    match ref_or {
        ReferenceOr::Item(schema) => {
            let comp = parse_schema(&name, &schema);

            println!("{:#?}", comp);

            comp
        }
        ReferenceOr::Reference { reference } => {
            log::info!(
                "reference for schemas is not supported yet. Skipping {} for {}...",
                reference,
                name
            );

            vec![]
        }
    }
}

fn parse_schema(name: &String, schema: &openapiv3::Schema) -> Vec<highway::Component> {
    use inflections::Inflect;

    let mut list = vec![];

    let mut component = highway::Component {
        name: name.clone().to_pascal_case(),
        description: schema.schema_data.description.clone(),
        ..Default::default()
    };

    use openapiv3::{SchemaKind, Type};

    match &schema.schema_kind {
        SchemaKind::Type(schema_type) => {
            let kind: highway::ComponentKind = match schema_type {
                Type::Object(object) => {
                    let mut fields = indexmap::IndexMap::new();

                    // Iterate over fields in object
                    for (field, schema) in object.properties.iter() {
                        // Parse field schema to highway components
                        let field_components = parse_ref_or_schema(&field, &schema.clone().unbox());

                        for field_component in field_components.iter() {
                            let required = object
                                .required
                                .iter()
                                .find(|found| *found == field)
                                .is_some();

                            let description = field_component.description.clone();

                            match field_component.kind {
                                highway::ComponentKind::Number => {
                                    fields.insert(
                                        field.clone(),
                                        highway::ComponentField {
                                            name: field.clone(),
                                            required,
                                            description,
                                            field_type: highway::FieldType::Number,
                                        },
                                    );
                                }
                                highway::ComponentKind::Integer => {
                                    fields.insert(
                                        field.clone(),
                                        highway::ComponentField {
                                            name: field.clone(),
                                            required,
                                            description,
                                            field_type: highway::FieldType::Integer,
                                        },
                                    );
                                }
                                highway::ComponentKind::Boolean => {
                                    fields.insert(
                                        field.clone(),
                                        highway::ComponentField {
                                            name: field.clone(),
                                            required,
                                            description,
                                            field_type: highway::FieldType::Boolean,
                                        },
                                    );
                                }
                                highway::ComponentKind::String => {
                                    fields.insert(
                                        field.clone(),
                                        highway::ComponentField {
                                            name: field.clone(),
                                            required,
                                            description,
                                            field_type: highway::FieldType::String,
                                        },
                                    );
                                }
                                highway::ComponentKind::Object { .. } => {
                                    let field_structure_component = highway::Component {
                                        // Full name of the structure for nested object
                                        name: format!(
                                            "{}{}",
                                            name.to_pascal_case(),
                                            field.to_pascal_case()
                                        ),
                                        description: field_component.description.clone(),
                                        kind: field_component.kind.clone(),
                                    };

                                    fields.insert(
                                        field.clone(),
                                        highway::ComponentField {
                                            /// name of the field
                                            name: field.clone(),
                                            required,
                                            description,
                                            field_type: highway::FieldType::Type(
                                                field_structure_component.name.clone(),
                                            ),
                                        },
                                    );

                                    list.push(field_structure_component);
                                }
                                _ => {}
                            };
                        }
                    }

                    highway::ComponentKind::Object { fields }
                }
                Type::Array(array) => highway::ComponentKind::Array {
                    items: Default::default(),
                },
                Type::Number(_number) => highway::ComponentKind::Number,
                Type::Integer(_integer) => highway::ComponentKind::Integer,
                Type::String(_string) => highway::ComponentKind::String,
                Type::Boolean {} => highway::ComponentKind::Boolean,
            };

            component.kind = kind;
        }
        other => log::info!("this schema kind is not supported {:?}", other),
    }

    list.push(component);

    list
}
