pub mod api;
pub mod components;
pub mod paths;

pub trait Printable {
    fn print(&self) -> proc_macro2::TokenStream;
}

impl<T> Printable for Vec<T>
where
    T: Printable,
{
    fn print(&self) -> proc_macro2::TokenStream {
        use quote::quote;

        let list = self.iter().map(|x| x.print());

        quote! {
            #(#list)*
        }
    }
}

#[derive(Default)]
pub struct GeneratedModule {
    pub api_module: api::module::ApiModule,
    pub components_module: components::module::ComponentsModule,
    pub paths_module: paths::module::PathsModule,
}

impl GeneratedModule {
    pub fn set_name(&mut self, name: String) {
        self.api_module.api.api_name = name.clone();
        self.api_module.methods.api_name = name;
    }

    pub fn set_description(&mut self, description: String) {
        self.api_module.api.description = Some(description);
    }
}

impl Printable for GeneratedModule {
    fn print(&self) -> proc_macro2::TokenStream {
        let api_module = self.api_module.print();
        let components_module = self.components_module.print();
        let paths_module = self.paths_module.print();

        quote::quote! {
            #![allow(dead_code, unused_imports)]

            #api_module
            #components_module
            #paths_module
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        api::{ApiModule, ApiStruct, BindApiMethod, HttpMethod, ImplApi},
        components::{
            parameters::ParametersModule, request_bodies::RequestBodiesModule,
            responses::ResponsesModule, Component, ComponentsModule, EnumVariant, Field, FieldType,
            FormatFloat, FormatInteger, FormatString, NativeType,
        },
        paths::{
            ContentType, Path, PathsModule, QueryParam, ResponseEnum, ResponseStatus, StatusVariant,
        },
        GeneratedModule,
    };
    use crate::test::shot;
    use insta::assert_snapshot;

    #[test]
    fn huge_test_all_generated() {
        let api: ApiStruct = ApiStruct {
        api_name: "ExampleApiDef".to_owned(),
        description: Some("Public API for frontend and OAuth applications [Review Github](https://developer.github.com/apps/building-oauth-apps/authorizing-oauth-apps/)".to_owned()),
        terms_of_service: None,
    };

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
            parameters: ParametersModule {
                list: vec![
                Component::Enum {
                    name: "OAuthResponseType".to_owned(),
                    description: Some(
                        "response_type is set to code indicating that you want an authorization code as the response."
                            .to_owned(),
                    ),
                    variants: vec![EnumVariant {
                        name: "code".to_owned(),
                        description: None,
                    }],
                },
                Component::Type {
                    name: "OAuthClientId".to_owned(),
                    description: Some("The client_id is the identifier for your app".to_owned()),
                    type_value: FieldType::Internal("uuid::Uuid".to_owned()),
                },
                Component::Type {
                    name: "OAuthRedirectUri".to_owned(),
                    description: Some(
                        "redirect_uri may be optional depending on the API, but is highly recommended".to_owned(),
                    ),
                    type_value: FieldType::Native(NativeType::String { format: Default::default() }),
                },
            ],
            },
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
            query_params: vec![],
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
            query_params: vec![
                QueryParam {
                    name: "responseType".to_owned(),
                    type_ref: "OAuthResponseType".to_owned(),
                    description: Some(
                        "response_type is set to code indicating that you want an authorization code as the response."
                            .to_owned(),
                    ),
                    required: true,
                },
                QueryParam {
                    name: "redirect_uri".to_owned(),
                    type_ref: "OAuthRedirectUri".to_owned(),
                    description: None,
                    required: false,
                },
                QueryParam {
                    name: "GlobalNameOfTheUniverse".to_owned(),
                    type_ref: "OAuthClientId".to_owned(),
                    description: None,
                    required: false,
                },
            ],
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

        let paths_module = PathsModule {
            paths: vec![p1, p2],
        };

        let generated_module = GeneratedModule {
            api_module,
            components_module,
            paths_module,
        };

        assert_snapshot!(shot(generated_module), @r###"
        #![allow(dead_code, unused_imports)]
        pub mod api {
            #[doc = "Public API for frontend and OAuth applications [Review Github](https://developer.github.com/apps/building-oauth-apps/authorizing-oauth-apps/)"]
            pub struct ExampleApiDef {
                api: actix_swagger::Api,
            }
            impl ExampleApiDef {
                pub fn new() -> Self {
                    Self {
                        api: actix_swagger::Api::new(),
                    }
                }
            }
            impl Default for ExampleApiDef {
                fn default() -> Self {
                    let api = Self::new();
                    api
                }
            }
            impl actix_web::dev::HttpServiceFactory for ExampleApiDef {
                fn register(self, config: &mut actix_web::dev::AppService) {
                    self.api.register(config);
                }
            }
            use super::paths;
            use actix_swagger::{Answer, Method};
            use actix_web::{dev::Factory, FromRequest};
            use std::future::Future;
            impl ExampleApiDef {
                pub fn bind_session_get<F, T, R>(mut self, handler: F) -> Self
                where
                    F: Factory<T, R, Answer<'static, paths::session_get::Response>>,
                    T: FromRequest + 'static,
                    R: Future<Output = Answer<'static, paths::session_get::Response>> + 'static,
                {
                    self.api = self.api.bind("/session".to_owned(), Method::GET, handler);
                    self
                }
                #[doc = "Request body - super::requst_bodies::SessionCreateBody"]
                pub fn bind_session_create<F, T, R>(mut self, handler: F) -> Self
                where
                    F: Factory<T, R, Answer<'static, paths::session_create::Response>>,
                    T: FromRequest + 'static,
                    R: Future<Output = Answer<'static, paths::session_create::Response>> + 'static,
                {
                    self.api = self.api.bind("/session".to_owned(), Method::POST, handler);
                    self
                }
                #[doc = "Request body - super::requst_bodies::RegisterConfirmation"]
                pub fn bind_register_confirmation<F, T, R>(mut self, handler: F) -> Self
                where
                    F: Factory<T, R, Answer<'static, paths::register_confirmation::Response>>,
                    T: FromRequest + 'static,
                    R: Future<Output = Answer<'static, paths::register_confirmation::Response>> + 'static,
                {
                    self.api = self
                        .api
                        .bind("/register/confirmation".to_owned(), Method::POST, handler);
                    self
                }
            }
        }
        pub mod components {
            pub mod parameters {
                use serde::{Deserialize, Serialize};
                #[doc = "response_type is set to code indicating that you want an authorization code as the response."]
                #[derive(Debug, Serialize, Deserialize)]
                pub enum OauthResponseType {
                    #[serde(rename = "code")]
                    Code,
                }
                #[doc = "The client_id is the identifier for your app"]
                pub type OauthClientId = uuid::Uuid;
                #[doc = "redirect_uri may be optional depending on the API, but is highly recommended"]
                pub type OauthRedirectUri = String;
            }
            pub mod request_bodies {
                use serde::{Deserialize, Serialize};
                #[derive(Debug, Serialize, Deserialize)]
                pub struct Register {
                    pub email: String,
                    pub demo: Option<Vec<Vec<String>>>,
                }
                #[derive(Debug, Serialize, Deserialize)]
                pub struct RegisterConfirmation {
                    #[serde(rename = "confirmationCode")]
                    pub confirmation_code: String,
                    #[serde(rename = "firstName")]
                    pub first_name: String,
                    #[serde(rename = "lastName")]
                    pub last_name: String,
                    pub password: String,
                    pub demo: Option<f32>,
                    pub customizer: Option<crate::app::MySuperType>,
                }
            }
            pub mod responses {
                use serde::{Deserialize, Serialize};
                #[doc = "Answer for registration confirmation"]
                #[derive(Debug, Serialize, Deserialize)]
                pub struct RegisterConfirmationFailed {
                    pub error: RegisterConfirmationFailedError,
                }
                #[derive(Debug, Serialize, Deserialize)]
                pub enum RegisterConfirmationFailedError {
                    #[serde(rename = "code_invalid_or_expired")]
                    CodeInvalidOrExpired,
                    #[serde(rename = "email_already_activated")]
                    EmailAlreadyActivated,
                    #[serde(rename = "invalid_form")]
                    InvalidForm,
                }
                #[doc = "Registration link sent to email, now user can find out when the link expires"]
                #[derive(Debug, Serialize, Deserialize)]
                pub struct RegistrationRequestCreated {
                    #[doc = "UTC Unix TimeStamp when the link expires"]
                    #[serde(rename = "expiresAt")]
                    pub expires_at: i64,
                }
            }
        }
        pub mod paths {
            use super::components::{parameters, responses};
            pub mod register_confirmation {
                use super::responses;
                use actix_swagger::{Answer, ContentType, StatusCode};
                use serde::{Deserialize, Serialize};
                #[derive(Debug, Serialize)]
                #[serde(untagged)]
                pub enum Response {
                    Created,
                    BadRequest(responses::RegisterConfirmationFailed),
                    Unexpected,
                }
                impl Response {
                    #[inline]
                    pub fn to_answer(self) -> Answer<'static, Self> {
                        let status = match self {
                            Self::Created => StatusCode::CREATED,
                            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
                            Self::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
                        };
                        let content_type = match self {
                            Self::Created => None,
                            Self::BadRequest(_) => Some(ContentType::Json),
                            Self::Unexpected => Some(ContentType::Json),
                        };
                        Answer::new(self).status(status).content_type(content_type)
                    }
                }
            }
            pub mod session_create {
                use super::responses;
                use actix_swagger::{Answer, ContentType, StatusCode};
                use serde::{Deserialize, Serialize};
                #[derive(Debug, Serialize)]
                #[serde(untagged)]
                pub enum Response {
                    #[doc = "User logined, cookies writed\nFoo"]
                    Created,
                    BadRequest(responses::SessionCreateFailed),
                    Unexpected,
                }
                impl Response {
                    #[inline]
                    pub fn to_answer(self) -> Answer<'static, Self> {
                        let status = match self {
                            Self::Created => StatusCode::CREATED,
                            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
                            Self::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
                        };
                        let content_type = match self {
                            Self::Created => None,
                            Self::BadRequest(_) => Some(ContentType::Json),
                            Self::Unexpected => Some(ContentType::Json),
                        };
                        Answer::new(self).status(status).content_type(content_type)
                    }
                }
                use super::parameters;
                #[derive(Debug, Deserialize)]
                pub struct QueryParams {
                    #[doc = "response_type is set to code indicating that you want an authorization code as the response."]
                    #[serde(rename = "responseType")]
                    pub response_type: parameters::OauthResponseType,
                    pub redirect_uri: Option<parameters::OauthRedirectUri>,
                    #[serde(rename = "GlobalNameOfTheUniverse")]
                    pub global_name_of_the_universe: Option<parameters::OauthClientId>,
                }
                pub type Query = actix_web::http::Query<QueryParams>;
            }
        }
        "###);
    }
}
