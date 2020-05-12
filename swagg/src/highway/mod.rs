//! # Highway
//!
//! This module is a database of components and paths
//! Useful when converting OpenAPI structures to printer structures

use crate::printer;
use indexmap::IndexMap;
use openapiv3::{ReferenceOr, Schema};

/// List of components ready to be printed
#[derive(Debug, Default)]
pub struct Components {
    pub request_bodies: IndexMap<String, RequestBody>,
    pub schemas: IndexMap<String, Component>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct RequestBody {
    pub name: String,
    pub required: bool,
    pub description: Option<String>,
    pub fields: IndexMap<String, ComponentField>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Component {
    pub name: String,
    pub description: Option<String>,
    pub kind: ComponentKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ComponentKind {
    Object {
        fields: IndexMap<String, ComponentField>,
    },
    Array {
        items: FieldType,
    },
    Integer,
    String,
    Number,
    Boolean,
}

impl Default for ComponentKind {
    fn default() -> Self {
        Self::String
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
pub struct ComponentField {
    pub required: bool,
    pub description: Option<String>,
    pub field_type: FieldType,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum FieldType {
    String,
    Number,
    Integer,
    Boolean,

    /// Name of the type in module
    Type(String),
}

impl Default for FieldType {
    fn default() -> Self {
        Self::String
    }
}

impl Components {
    pub fn new() -> Self {
        Self::default()
    }

    fn insert_request_body(&mut self, body: RequestBody) {
        self.request_bodies.insert(body.name.clone(), body);
    }

    fn insert_schema(&mut self, component: Component) {
        self.schemas.insert(component.name.clone(), component);
    }

    pub fn parse_schema(
        &mut self,
        name: &String,
        schema: &ReferenceOr<Schema>,
    ) -> Result<(), ParseSchemaError> {
        let (_, created_components) = parse_ref_or_schema(&name, &schema)?;

        for component in created_components.into_iter() {
            self.insert_schema(component);
        }

        Ok(())
    }

    // Parse request body from openapiv3 and add it to index
    // pub fn parse_request_body(&mut self, name: &String, request_body: &openapiv3::RequestBody) {
    //     log::trace!("Parsing request body {}", name);
    //     let mut highway_body = RequestBody::default();

    //     highway_body.name = name.clone();
    //     highway_body.description = request_body.description.clone();
    //     highway_body.required = request_body.required;

    //     if let Some(media_type) = request_body.content.get("application/json") {
    //         if let Some(ReferenceOr::Item(schema)) = media_type.schema.clone() {
    //             self.parse_body_schema(&schema);
    //         } else {
    //             log::info!(
    //                 "skipping media type {:?} in {}.content, supported only inline schema",
    //                 media_type.schema,
    //                 name
    //             );
    //         }
    //     } else {
    //         log::info!(
    //             "skipping content {:?} in {}, supported only application/json",
    //             request_body.content,
    //             name
    //         );
    //     }

    //     self.insert_request_body(highway_body);
    // }

    // fn parse_body_schema(&mut self, schema: &openapiv3::Schema) {
    //     log::trace!("Parsing schema {:?}", schema);
    //     use openapiv3::{SchemaKind, Type};

    //     match schema.schema_kind.clone() {
    //         SchemaKind::Type(internal_type) => match internal_type {
    //             Type::Object(object) => {
    //                 let component = Self::parse_object_to_component(&object);
    //                 let body = RequestBody {
    //                     name: component.name,
    //                     description: component.description,
    //                     required: true,
    //                     fields: component.fields,
    //                 };
    //                 self.insert_request_body(body);
    //             }
    //             unsupported_type => {
    //                 log::info!("unsupported schema type {:?}", unsupported_type);
    //             }
    //         },
    //         unsupported_kind => {
    //             log::info!("unsupported schema kind {:?}", unsupported_kind);
    //         }
    //     }
    // }

    // Can't parse description and name
    // fn parse_object_to_component(object: &openapiv3::ObjectType) -> Component {
    //     use openapiv3::{SchemaKind, Type};
    //     let mut component = Component::default();

    //     for (name, schema) in object.properties.iter() {
    //         let required = object.required.iter().find(|e| *e == name).is_some();

    //         if let ReferenceOr::Item(schema) = schema.clone().unbox() {
    //             match schema.schema_kind {
    //                 SchemaKind::Type(Type::String(_string_type)) => {
    //                     let field_type = FieldType::String;
    //                     let field = ComponentField {
    //                         name: name.clone(),
    //                         description: schema.schema_data.description.clone(),
    //                         required,
    //                         field_type,
    //                     };
    //                     component.fields.insert(name.clone(), field);
    //                 }
    //                 unsupported_kind => {
    //                     log::info!("skipping unsupported kind {:?}", unsupported_kind);
    //                     continue;
    //                 }
    //             }
    //         } else {
    //             log::info!("skipping property {} due to unsupported refs", name);
    //         }
    //     }

    //     component
    // }
}

pub enum Reference {
    /// References to the current file
    Relative(ReferenceRelative),

    /// References to the file on the current disk
    File(String),

    /// References to the file in the web
    Remote,
}

pub enum ReferenceRelative {
    Response { name: String },
    Parameter { name: String },
    RequestBody { name: String },
    Schema { name: String },
    Responses { name: String },
}

#[derive(Debug)]
pub enum ParseSchemaError {
    UnsupportedType,
    ReferenceNotSupported,
}

fn parse_ref_or_schema(
    name: &String,
    ref_or: &ReferenceOr<openapiv3::Schema>,
) -> Result<(FieldType, Vec<Component>), ParseSchemaError> {
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
) -> Result<(FieldType, Vec<Component>), ParseSchemaError> {
    use inflections::Inflect;

    let mut list = vec![];

    use openapiv3::{SchemaKind, Type};

    match &schema.schema_kind {
        SchemaKind::Type(schema_type) => {
            let field_type = match schema_type {
                Type::Number(_number) => FieldType::Number,
                Type::Integer(_integer) => FieldType::Integer,
                Type::String(_string) => FieldType::String,
                Type::Boolean {} => FieldType::Boolean,
                Type::Object(object) => {
                    let (fields, mut created_components) = parse_schema_object(&name, object)?;

                    let component_name = name.clone().to_pascal_case();
                    let component = Component {
                        name: component_name.clone(),
                        description: schema.schema_data.description.clone(),
                        kind: ComponentKind::Object { fields },
                    };

                    list.push(component);
                    list.append(&mut created_components);

                    FieldType::Type(component_name.clone())
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
) -> Result<(indexmap::IndexMap<String, ComponentField>, Vec<Component>), ParseSchemaError> {
    use inflections::Inflect;

    let mut components = vec![];
    let mut fields = indexmap::IndexMap::new();

    for (field_name, schema) in schema_object.properties.iter() {
        let inner_name = format!("{}{}", name, field_name.clone().to_pascal_case());

        let (field_type, mut created_components) =
            parse_ref_or_schema(&inner_name, &schema.clone().unbox())?;

        components.append(&mut created_components);

        let field = ComponentField {
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

impl Into<printer::GeneratedModule> for Components {
    fn into(self) -> printer::GeneratedModule {
        let module = Default::default();

        module
    }
}
