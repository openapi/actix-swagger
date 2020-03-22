use super::structure::to_struct_name;
use crate::printer::Printable;
use inflections::Inflect;
use quote::{format_ident, quote};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum HttpMethod {
    Delete,
    Get,
    Patch,
    Post,
    Put,
}

impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        match self {
            HttpMethod::Delete => "DELETE",
            HttpMethod::Get => "GET",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
        }
        .to_owned()
    }
}

#[derive(Debug, Serialize)]
pub struct BindApiMethod {
    pub method: HttpMethod,
    pub path: String,
    pub name: String,
    pub request_body: Option<String>,
}

impl Printable for BindApiMethod {
    fn print(&self) -> proc_macro2::TokenStream {
        let request_path = self.path.clone();
        let http_method = format_ident!("{}", self.method.to_string());
        let path_name = format_ident!("{}", self.name.to_snake_case());
        let bind_method_name = format_ident!("bind_{}", self.name.to_snake_case());
        let request_body_stream = match &self.request_body {
            Some(request_body) => {
                let body = request_body.to_pascal_case();
                let doc = format!("Request body - super::requst_bodies::{}", body);
                quote! { #[doc = #doc] }
            }
            None => quote! {},
        };

        quote! {
            #request_body_stream
            pub fn #bind_method_name<F, T, R>(mut self, handler: F) -> Self
            where
                F: Factory<T, R, Answer<'static, paths::#path_name::Response>>,
                T: FromRequest + 'static,
                R: Future<Output = Answer<'static, paths::#path_name::Response>> + 'static,
            {
                self.api = self.api.bind(#request_path.to_owned(), Method::#http_method, handler);
                self
            }
        }
    }
}

pub struct ImplApi {
    pub api_name: String,
    pub methods: Vec<BindApiMethod>,
}

impl Default for ImplApi {
    fn default() -> Self {
        Self {
            api_name: "Api".to_owned(),
            methods: vec![],
        }
    }
}

impl Printable for ImplApi {
    fn print(&self) -> proc_macro2::TokenStream {
        let api_name = format_ident!("{}", to_struct_name(self.api_name.to_owned()));
        let methods = self.methods.print();

        quote! {
            use actix_web::{FromRequest, dev::Factory};
            use actix_swagger::{Answer, Method};
            use std::future::Future;
            use super::paths;

            impl #api_name {
                #methods
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::shot;
    use insta::assert_yaml_snapshot;

    fn api(methods: Vec<BindApiMethod>) -> ImplApi {
        ImplApi {
            api_name: "test_api".to_owned(),
            methods,
        }
    }

    #[test]
    fn default_api() {
        assert_yaml_snapshot!(shot(ImplApi::default()), @r###"
        ---
        - "use super::paths;"
        - "use actix_swagger::{Answer, Method};"
        - "use actix_web::{dev::Factory, FromRequest};"
        - "use std::future::Future;"
        - "impl Api {}"
        - ""
        "###);
    }

    #[test]
    fn impl_api_without_methods() {
        let api = ImplApi {
            api_name: "Hello".to_owned(),
            methods: vec![],
        };

        assert_yaml_snapshot!(shot(api), @r###"
        ---
        - "use super::paths;"
        - "use actix_swagger::{Answer, Method};"
        - "use actix_web::{dev::Factory, FromRequest};"
        - "use std::future::Future;"
        - "impl Hello {}"
        - ""
        "###);
    }

    #[test]
    fn impl_api_pascal_naming() {
        let api1 = ImplApi {
            api_name: "HelloGoof".to_owned(),
            methods: vec![],
        };
        let api2 = ImplApi {
            api_name: "thats_my_name".to_owned(),
            methods: vec![],
        };
        let api3 = ImplApi {
            api_name: "RANDOMIZE_THIS_F_WOOORLD".to_owned(),
            methods: vec![],
        };

        assert_yaml_snapshot!(shot(vec![api1, api2, api3]), @r###"
        ---
        - "use super::paths;"
        - "use actix_swagger::{Answer, Method};"
        - "use actix_web::{dev::Factory, FromRequest};"
        - "use std::future::Future;"
        - "impl HelloGoof {}"
        - "use super::paths;"
        - "use actix_swagger::{Answer, Method};"
        - "use actix_web::{dev::Factory, FromRequest};"
        - "use std::future::Future;"
        - "impl ThatsMyName {}"
        - "use super::paths;"
        - "use actix_swagger::{Answer, Method};"
        - "use actix_web::{dev::Factory, FromRequest};"
        - "use std::future::Future;"
        - "impl RandomizeThisFWooorld {}"
        - ""
        "###);
    }

    #[test]
    fn bind_api_method_without_body() {
        let method = BindApiMethod {
            method: HttpMethod::Post,
            name: "hey_make_my_day".to_owned(),
            path: "/hey-make/my-day".to_owned(),
            request_body: None,
        };

        assert_yaml_snapshot!(shot(api(vec![method])), @r###"
        ---
        - "use super::paths;"
        - "use actix_swagger::{Answer, Method};"
        - "use actix_web::{dev::Factory, FromRequest};"
        - "use std::future::Future;"
        - "impl TestApi {"
        - "    pub fn bind_hey_make_my_day<F, T, R>(mut self, handler: F) -> Self"
        - "    where"
        - "        F: Factory<T, R, Answer<'static, paths::hey_make_my_day::Response>>,"
        - "        T: FromRequest + 'static,"
        - "        R: Future<Output = Answer<'static, paths::hey_make_my_day::Response>> + 'static,"
        - "    {"
        - "        self.api = self"
        - "            .api"
        - "            .bind(\"/hey-make/my-day\".to_owned(), Method::POST, handler);"
        - "        self"
        - "    }"
        - "}"
        - ""
        "###);
    }

    #[test]
    fn two_methods() {
        let method1 = BindApiMethod {
            method: HttpMethod::Post,
            name: "hey_make_my_day".to_owned(),
            path: "/hey-make/my-day".to_owned(),
            request_body: None,
        };

        let method2 = BindApiMethod {
            method: HttpMethod::Delete,
            name: "ThisIsMyTestNameInPascalCase".to_owned(),
            path: "/Very/Very/VEry/Loo000ng/Path".to_owned(),
            request_body: None,
        };

        assert_yaml_snapshot!(shot(api(vec![method1, method2])), @r###"
        ---
        - "use super::paths;"
        - "use actix_swagger::{Answer, Method};"
        - "use actix_web::{dev::Factory, FromRequest};"
        - "use std::future::Future;"
        - "impl TestApi {"
        - "    pub fn bind_hey_make_my_day<F, T, R>(mut self, handler: F) -> Self"
        - "    where"
        - "        F: Factory<T, R, Answer<'static, paths::hey_make_my_day::Response>>,"
        - "        T: FromRequest + 'static,"
        - "        R: Future<Output = Answer<'static, paths::hey_make_my_day::Response>> + 'static,"
        - "    {"
        - "        self.api = self"
        - "            .api"
        - "            .bind(\"/hey-make/my-day\".to_owned(), Method::POST, handler);"
        - "        self"
        - "    }"
        - "    pub fn bind_this_is_my_test_name_in_pascal_case<F, T, R>(mut self, handler: F) -> Self"
        - "    where"
        - "        F: Factory<T, R, Answer<'static, paths::this_is_my_test_name_in_pascal_case::Response>>,"
        - "        T: FromRequest + 'static,"
        - "        R: Future<Output = Answer<'static, paths::this_is_my_test_name_in_pascal_case::Response>>"
        - "            + 'static,"
        - "    {"
        - "        self.api = self.api.bind("
        - "            \"/Very/Very/VEry/Loo000ng/Path\".to_owned(),"
        - "            Method::DELETE,"
        - "            handler,"
        - "        );"
        - "        self"
        - "    }"
        - "}"
        - ""
        "###);
    }

    #[test]
    fn with_request_body() {
        let method = BindApiMethod {
            method: HttpMethod::Post,
            name: "sessionCreate".to_owned(),
            path: "/session".to_owned(),
            request_body: Some("SessionCreateBody".to_owned()),
        };

        assert_yaml_snapshot!(shot(api(vec![method])), @r###"
        ---
        - "use super::paths;"
        - "use actix_swagger::{Answer, Method};"
        - "use actix_web::{dev::Factory, FromRequest};"
        - "use std::future::Future;"
        - "impl TestApi {"
        - "    #[doc = \"Request body - super::requst_bodies::SessionCreateBody\"]"
        - "    pub fn bind_session_create<F, T, R>(mut self, handler: F) -> Self"
        - "    where"
        - "        F: Factory<T, R, Answer<'static, paths::session_create::Response>>,"
        - "        T: FromRequest + 'static,"
        - "        R: Future<Output = Answer<'static, paths::session_create::Response>> + 'static,"
        - "    {"
        - "        self.api = self.api.bind(\"/session\".to_owned(), Method::POST, handler);"
        - "        self"
        - "    }"
        - "}"
        - ""
        "###);
    }
}
