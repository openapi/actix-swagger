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
        fn register(self, config: &mut actix_web::dev::AppService) {
            self.api.register(config);
        }
    }
    use super::paths;
    use actix_swagger::{Answer, Method};
    use actix_web::{dev::Factory, FromRequest};
    use std::future::Future;
    impl AuthmenowPublicApi {}
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
