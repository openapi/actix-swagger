use openapiv3::OpenAPI;
use std::fs;

mod printer;

use printer::{
    api::{ApiModule, ApiStruct, BindApiMethod, HttpMethod, ImplApi},
    components::{
        request_bodies::RequestBodiesModule, responses::ResponsesModule, Component, ComponentsModule, EnumVariant,
        Field, FieldType, FormatFloat, FormatInteger, FormatString, NativeType,
    },
    paths::{ContentType, Path, PathsModule, ResponseEnum, ResponseStatus, StatusVariant},
    GeneratedModule, Printable,
};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let file_path = "/Users/sergeysova/Projects/authmenow/backend/public-api.openapi.yaml";
    let content = fs::read_to_string(&file_path)?;
    let api: OpenAPI = serde_yaml::from_str(&content)?;

    // println!("{}", "OAuthAuthorizeRequest".to_snake_case());

    let api: ApiStruct = api.info.into();

    let m1 = BindApiMethod {
        method: HttpMethod::Get,
        name: "sessionGet".to_owned(),
        path: "/session".to_owned(),
        request_body: None,
    };

    let m2 = BindApiMethod {
        method: HttpMethod::Post,
        name: "sessionCreate".to_owned(),
        path: "/session".to_owned(),
        request_body: Some("SessionCreateBody".to_owned()),
    };

    let m3 = BindApiMethod {
        method: HttpMethod::Post,
        name: "registerConfirmation".to_owned(),
        path: "/register/confirmation".to_owned(),
        request_body: Some("RegisterConfirmation".to_owned()),
    };

    let methods = ImplApi {
        api_name: api.api_name.clone(),
        methods: vec![m1, m2, m3],
    };

    let api_module = ApiModule { api, methods };

    let components_module = ComponentsModule {
        responses: ResponsesModule {
            list: vec![
                Component::Object {
                    name: "RegisterConfirmationFailed".to_owned(),
                    fields: vec![Field {
                        name: "error".to_owned(),
                        required: true,
                        description: None,
                        field_type: FieldType::Custom("RegisterConfirmationFailedError".to_owned()),
                    }],
                    description: Some("Answer for registration confirmation".to_owned()),
                },
                Component::Enum {
                    name: "RegisterConfirmationFailedError".to_owned(),
                    variants: vec![
                        EnumVariant {
                            name: "code_invalid_or_expired".to_owned(),
                            description: None,
                        },
                        EnumVariant {
                            name: "email_already_activated".to_owned(),
                            description: None,
                        },
                        EnumVariant {
                            name: "invalid_form".to_owned(),
                            description: None,
                        },
                    ],
                    description: None,
                },
                Component::Object {
                    name: "RegistrationRequestCreated".to_owned(),
                    description: Some(
                        "Registration link sent to email, now user can find out when the link expires".to_owned(),
                    ),
                    fields: vec![Field {
                        name: "expiresAt".to_owned(),
                        required: true,
                        description: Some("UTC Unix TimeStamp when the link expires".to_owned()),
                        field_type: FieldType::Native(NativeType::Integer {
                            format: FormatInteger::Int64,
                        }),
                    }],
                },
            ],
        },
        request_bodies: RequestBodiesModule {
            list: vec![
                Component::Object {
                    name: "Register".to_owned(),
                    description: None,
                    fields: vec![
                        Field {
                            name: "email".to_owned(),
                            required: true,
                            description: None,
                            field_type: FieldType::Native(NativeType::String {
                                format: FormatString::Email,
                            }),
                        },
                        Field {
                            name: "demo".to_owned(),
                            required: false,
                            description: None,
                            field_type: FieldType::Array(Box::new(FieldType::Array(Box::new(FieldType::Native(
                                NativeType::String {
                                    format: FormatString::Email,
                                },
                            ))))),
                        },
                    ],
                },
                Component::Object {
                    name: "RegisterConfirmation".to_owned(),
                    description: None,
                    fields: vec![
                        Field {
                            name: "confirmationCode".to_owned(),
                            required: true,
                            description: None,
                            field_type: FieldType::Native(NativeType::String {
                                format: FormatString::default(),
                            }),
                        },
                        Field {
                            name: "firstName".to_owned(),
                            required: true,
                            description: None,
                            field_type: FieldType::Native(NativeType::String {
                                format: FormatString::default(),
                            }),
                        },
                        Field {
                            name: "lastName".to_owned(),
                            required: true,
                            description: None,
                            field_type: FieldType::Native(NativeType::String {
                                format: FormatString::default(),
                            }),
                        },
                        Field {
                            name: "password".to_owned(),
                            required: true,
                            description: None,
                            field_type: FieldType::Native(NativeType::String {
                                format: FormatString::default(),
                            }),
                        },
                        Field {
                            name: "demo".to_owned(),
                            required: false,
                            description: None,
                            field_type: FieldType::Native(NativeType::Float {
                                format: FormatFloat::default(),
                            }),
                        },
                        Field {
                            name: "customizer".to_owned(),
                            required: false,
                            description: None,
                            field_type: FieldType::Internal("crate::app::MySuperType".to_owned()),
                        },
                    ],
                },
            ],
        },
    };

    let p1 = Path {
        name: "registerConfirmation".to_owned(),
        response: ResponseEnum {
            responses: vec![
                StatusVariant {
                    status: ResponseStatus::Created,
                    response_type_name: None,
                    description: None,
                    content_type: None,
                    x_variant_name: None,
                },
                StatusVariant {
                    status: ResponseStatus::BadRequest,
                    response_type_name: Some("RegisterConfirmationFailed".to_owned()),
                    description: None,
                    content_type: Some(ContentType::Json),
                    x_variant_name: None,
                },
                StatusVariant {
                    status: ResponseStatus::InternalServerError,
                    response_type_name: None,
                    description: None,
                    content_type: Some(ContentType::Json),
                    x_variant_name: Some("Unexpected".to_owned()),
                },
            ],
        },
    };

    let p2 = Path {
        name: "sessionCreate".to_owned(),
        response: ResponseEnum {
            responses: vec![
                StatusVariant {
                    status: ResponseStatus::Created,
                    response_type_name: None,
                    description: Some("User logined, cookies writed\nFoo".to_owned()),
                    content_type: None,
                    x_variant_name: None,
                },
                StatusVariant {
                    status: ResponseStatus::BadRequest,
                    response_type_name: Some("sessionCreateFailed".to_owned()),
                    description: None,
                    content_type: Some(ContentType::Json),
                    x_variant_name: None,
                },
                StatusVariant {
                    status: ResponseStatus::InternalServerError,
                    response_type_name: None,
                    description: None,
                    content_type: Some(ContentType::Json),
                    x_variant_name: Some("Unexpected".to_owned()),
                },
            ],
        },
    };

    let paths_module = PathsModule { paths: vec![p1, p2] };

    let generated_module = GeneratedModule {
        api_module,
        components_module,
        paths_module,
    };

    println!("{}", generated_module.print());

    Ok(())
}
