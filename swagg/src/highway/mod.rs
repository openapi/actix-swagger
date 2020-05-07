//! # Highway
//!
//! This module is a database of components and paths
//! Useful when converting OpenAPI structures to printer structures

use indexmap::IndexMap;
use openapiv3::ReferenceOr;

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
    pub name: String,
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
