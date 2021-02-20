use inflections::Inflect;
use openapi_hooks::v3::{
    self, Header, Operation, Parameter, ParameterData, ParameterSchemaOrContent, RequestBody,
    Response, Schema, SchemaKind,
};
use openapi_hooks::{Internal, Method, Plugin};
use printer::{
    components::{
        component::Component, parameters::ParametersModule, Field, FieldType, FormatFloat,
        FormatInteger, FormatString, NativeType,
    },
    Printable,
};
use v3::VariantOrUnknownOrEmpty;

mod printer;
#[cfg(test)]
pub mod test;

#[derive(Default)]
pub struct ActixPlugin {
    schemas: Vec<(String, String)>,
    parameters_module: ParametersModule,
}

impl ActixPlugin {
    pub fn new() -> Self {
        Default::default()
    }

    fn add_parameter<'i, I: Internal<'i>>(&'i mut self, parameter: &'i Parameter, internal: &'i I) {
        match parameter {
            Parameter::Query { parameter_data, .. } => {
                let ParameterData {
                    name,
                    description,
                    required,
                    deprecated,
                    format,
                    ..
                } = parameter_data;

                let schema = self
                    .resolve_schema_from_parameter_format(format, internal)
                    .expect(
                        format!(
                            "Failed to resolve schema reference for parameter '{}'",
                            name,
                        )
                        .as_str(),
                    );

                match &schema.schema_kind {
                    SchemaKind::OneOf { .. } => {}
                    SchemaKind::AllOf { .. } => {}
                    SchemaKind::AnyOf { .. } => {}
                    SchemaKind::Any(_) => {}
                    SchemaKind::Type(schema_type) => {
                        use v3::VariantOrUnknownOrEmpty::{Empty, Item, Unknown};

                        let content_type = match schema_type {
                            v3::Type::Number(s) => match s.format {
                                Empty | Unknown(_) => FieldType::native_float(Default::default()),
                                Item(v3::NumberFormat::Float) => {
                                    FieldType::native_float(FormatFloat::Float)
                                }
                                Item(v3::NumberFormat::Double) => {
                                    FieldType::native_float(FormatFloat::Double)
                                }
                            },
                            v3::Type::Integer(s) => match s.format {
                                Empty | Unknown(_) => FieldType::native_integer(Default::default()),
                                Item(v3::IntegerFormat::Int32) => {
                                    FieldType::native_integer(FormatInteger::Int32)
                                }
                                Item(v3::IntegerFormat::Int64) => {
                                    FieldType::native_integer(FormatInteger::Int64)
                                }
                            },
                            v3::Type::String(_) => FieldType::native_string(Default::default()),
                            v3::Type::Boolean {} => FieldType::native_boolean(),
                            v3::Type::Array(list) => unimplemented!(),
                            v3::Type::Object(object) => {
                                unimplemented!()
                            }
                        };

                        let component_name = name.to_pascal_case();
                        let component = match content_type {
                            FieldType::Native(_) => Component::Type {
                                name: component_name.clone(),
                                description: None,
                                type_value: content_type,
                            },
                            FieldType::Array(_) => Component::Type {
                                name: component_name.clone(),
                                description: None,
                                type_value: content_type,
                            },
                            _ => unimplemented!(),
                            // FieldType::Object(_) => Component::Object {
                            //     name: component_name.clone(),
                            //     description: None,
                            //     fields: vec![]
                            // }
                        };

                        self.parameters_module.list.push(component);

                        // self.parameters_module.list.push(Component::Type {
                        //     name: name.clone().to_pascal_case(),
                        //     description: None,
                        //     type_value: FieldType::Internal(format!(
                        //         "actix_web::web::Query<{}>",
                        //         component_name
                        //     )),
                        // });
                    }
                }
            }
            rest => println!("Parameter type is not supported yet {:#?}", rest),
        }
    }

    fn resolve_schema_from_parameter_format<'i, I: Internal<'i>>(
        &self,
        param: &'i ParameterSchemaOrContent,
        internal: &'i I,
    ) -> Option<&'i Schema> {
        match param {
            ParameterSchemaOrContent::Schema(refor) => internal.resolve(refor),
            ParameterSchemaOrContent::Content(content_map) => {
                let value = content_map.get("application/json")?;
                match value.schema {
                    Some(ref value) => internal.resolve(value),
                    None => return None,
                }
            }
        }
    }
}

impl<'i, I: Internal<'i>> Plugin<'i, I> for ActixPlugin {
    fn on_parameter(&'i mut self, _name: &'i str, parameter: &'i Parameter, internal: &'i I) {
        self.add_parameter(parameter, internal);
    }

    fn pre_components(&'i mut self, internal: &'i I) {
        println!(
            "{} {}",
            internal.root().info.version,
            internal.root().info.title,
        );
    }

    fn post_components(&'i mut self, internal: &'i I) {
        let result = self.parameters_module.print().to_string();
        println!("{}", result);
    }

    fn proceed(&'i mut self, internal: &'i mut I) {
        internal.create_file("lib.rs".to_owned(), "".to_owned());
    }
}
