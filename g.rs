#![allow(dead_code, unused_imports)]
pub mod api {
    #[doc = "Internal API for frontend"]
    pub struct AccessoAppInternalApi {
        api: actix_swagger::Api,
    }
    impl AccessoAppInternalApi {
        pub fn new() -> Self {
            Self {
                api: actix_swagger::Api::new(),
            }
        }
    }
    impl Default for AccessoAppInternalApi {
        fn default() -> Self {
            let api = Self::new();
            api
        }
    }
    impl actix_web::dev::HttpServiceFactory for AccessoAppInternalApi {
        fn register(self, config: &mut actix_web::dev::AppService) {
            self.api.register(config);
        }
    }
    use super::paths;
    use actix_swagger::{Answer, Method};
    use actix_web::FromRequest;
    use std::future::Future;
    impl AccessoAppInternalApi {}
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
        }
    }
}
pub mod paths {
    use super::components::{parameters, responses};
}
