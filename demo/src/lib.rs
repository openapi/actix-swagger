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
}
pub mod paths {
    use super::components::{parameters, responses};
}
