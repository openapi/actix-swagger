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
            self.api = self.api.bind("/session".to_owned(), GET, handler);
            self
        }
        pub fn bind_session_create<F, T, R>(mut self, handler: F) -> Self
        where
            F: Factory<T, R, Answer<'static, paths::session_create::Response>>,
            T: FromRequest + 'static,
            R: Future<Output = Answer<'static, paths::session_create::Response>> + 'static,
        {
            self.api = self.api.bind("/session".to_owned(), POST, handler);
            self
        }
        pub fn bind_register_confirmation<F, T, R>(mut self, handler: F) -> Self
        where
            F: Factory<T, R, Answer<'static, paths::register_confirmation::Response>>,
            T: FromRequest + 'static,
            R: Future<Output = Answer<'static, paths::register_confirmation::Response>> + 'static,
        {
            self.api = self.api.bind("/register/confirmation".to_owned(), POST, handler);
            self
        }
    }
}
pub mod components {
    pub mod request_bodies {}
    pub mod responses {
        use serde::{Deserialize, Serialize};
        #[doc = "Answer for registration confirmation"]
        #[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
        pub struct RegisterConfirmationFailed {
            pub error: RegisterConfirmationFailedError,
        }
        #[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
        pub enum RegisterConfirmationFailedError {
            #[serde(rename = "code_invalid_or_expired")]
            CodeInvalidOrExpired,
            #[serde(rename = "email_already_activated")]
            EmailAlreadyActivated,
            #[serde(rename = "invalid_form")]
            InvalidForm,
        }
        #[doc = "Registration link sent to email, now user can find out when the link expires"]
        #[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
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
    }
}
