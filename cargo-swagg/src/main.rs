use openapiv3::OpenAPI;
use std::fs;

mod printer;

use printer::{
    api::{ApiModule, ApiStruct, BindApiMethod, HttpMethod, ImplApiMethods},
    components::{request_bodies::RequestBodiesModule, responses::ResponsesModule, ComponentsModule},
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
        response_type: "sessionGetResponse".to_owned(),
    };

    let m2 = BindApiMethod {
        method: HttpMethod::Post,
        name: "sessionCreate".to_owned(),
        path: "/session".to_owned(),
        response_type: "sessionCreateResponse".to_owned(),
    };

    let m3 = BindApiMethod {
        method: HttpMethod::Post,
        name: "registerConfirmation".to_owned(),
        path: "/register/confirmation".to_owned(),
        response_type: "registerConfirmationResponse".to_owned(),
    };

    let methods = ImplApiMethods {
        api_name: api.api_name.clone(),
        methods: vec![m1, m2, m3],
    };

    let api_module = ApiModule { api, methods };

    let components_module = ComponentsModule {
        responses: ResponsesModule { components: vec![] },
        request_bodies: RequestBodiesModule {},
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
