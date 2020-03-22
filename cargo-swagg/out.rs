pub mod api {
    #[doc = "Public API for frontend and OAuth applications [Review Github](https://developer.github.com/apps/building-oauth-apps/authorizing-oauth-apps/)"]
    pub struct AuthmenowPublicApi {
        api: actix_swagger::Api,
    }
    impl AuthmenowPublicApi {
        pub fn new() -> Self {
            Self {
                api: actix_swagger::Api::new(),
            }
        }
    }
    impl Default for AuthmenowPublicApi {
        fn default() -> Self {
            let api = Self::new();
            api
        }
    }
    impl actix_web::dev::HttpServiceFactory for AuthmenowPublicApi {
        fn register(mut self, config: &mut actix_web::dev::AppService) {
            self.api.register(config);
        }
    }
    use super::paths;
    use actix_swagger::{Answer, Method};
    use actix_web::{dev::Factory, FromRequest};
    use std::future::Future;
    impl AuthmenowPublicApi {
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
    pub mod register_confirmation {
        use super::components::responses;
        use actix_swagger::{Answer, ContentType};
        use actix_web::http::StatusCode;
        use serde::Serialize;
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
        use super::components::responses;
        use actix_swagger::{Answer, ContentType};
        use actix_web::http::StatusCode;
        use serde::Serialize;
        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            #[doc = "User logined, cookies writed\nFoo"]
            Created,
            BadRequest(responses::sessionCreateFailed),
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
        use super::components::parameters;
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
