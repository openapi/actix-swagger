#![allow(dead_code, unused_imports)]
pub mod api {
    #[doc = "Public API for Demo"]
    pub struct DemoApi {
        api: actix_swagger::Api,
    }
    impl DemoApi {
        pub fn new() -> Self {
            Self {
                api: actix_swagger::Api::new(),
            }
        }
    }
    impl Default for DemoApi {
        fn default() -> Self {
            let api = Self::new();
            api
        }
    }
    impl actix_web::dev::HttpServiceFactory for DemoApi {
        fn register(self, config: &mut actix_web::dev::AppService) {
            self.api.register(config);
        }
    }
    use super::paths;
    use actix_swagger::{Answer, Method};
    use actix_web::{dev::Factory, FromRequest};
    use std::future::Future;
    impl DemoApi {}
}
pub mod components {
    pub mod parameters {
        use serde::{Deserialize, Serialize};
    }
    pub mod request_bodies {
        use serde::{Deserialize, Serialize};
    }
    pub mod responses {
        use serde::{Deserialize, Serialize};
    }
    pub mod schemas {
        use serde::{Deserialize, Serialize};
        #[doc = "Current user in a session"]
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SessionUser {
            #[serde(rename = "firstName")]
            pub first_name: String,
            #[serde(rename = "lastName")]
            pub last_name: String,
            pub inner: Option<SessionUserInner>,
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SessionUserInner {
            pub foo: Option<f32>,
            pub bar: i32,
            pub baz: SessionUserInnerBaz,
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SessionUserInnerBaz {
            pub demo: Option<String>,
        }
    }
}
pub mod paths {
    use super::components::{parameters, responses};
}
