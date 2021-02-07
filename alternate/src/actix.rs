use crate::openapi::{Header, Operation, Parameter, RequestBody, Response, Schema, SecurityScheme};
use crate::{Internal, Method, Plugin};

pub struct ActixPlugin {}

impl ActixPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin<'_> for ActixPlugin {
    fn on_operation(
        &mut self,
        method: Method,
        path: &str,
        operation: &Operation,
        internal: &Internal<'_>,
    ) {
        println!("[operation] {:?} {}", method, path);
        println!(
            "- {:#?}",
            operation
                .parameters
                .iter()
                .map(|i| internal.resolve(i))
                .collect::<Vec<_>>()
        );
    }

    fn on_header(&mut self, name: &str, _header: &Header, _: &Internal<'_>) {
        println!("[header] {}", name);
    }

    fn on_parameter(&mut self, name: &str, _parameter: &Parameter, _: &Internal<'_>) {
        println!("[parameter] {}", name);
    }

    fn on_request_body(
        &mut self,
        name: &str,
        _request_body: &RequestBody,
        _internal: &Internal<'_>,
    ) {
        println!("[request_body] {}", name);
    }

    fn on_response(&mut self, name: &str, _response: &Response, _: &Internal<'_>) {
        println!("[response] {}", name);
    }

    fn on_schema(&mut self, name: &str, _schema: &Schema, _: &Internal<'_>) {
        println!("[schema] {}", name);
    }

    fn on_security_scheme(
        &mut self,
        name: &str,
        _security_scheme: &SecurityScheme,
        _: &Internal<'_>,
    ) {
        println!("[security_scheme] {}", name);
    }

    fn pre_paths(&mut self, _: &Internal<'_>) {
        println!("[PRE] paths");
    }

    fn post_paths(&mut self, _: &Internal<'_>) {
        println!("[POST] paths");
    }

    fn pre_components(&mut self, internal: &Internal<'_>) {
        println!("[PRE] components");
        println!(
            "{} {}",
            internal.root().info.title,
            internal.root().info.version
        );
    }

    fn post_components(&mut self, _: &Internal<'_>) {
        println!("[POST] components");
    }

    fn proceed(&mut self, internal: &mut Internal<'_>) {
        internal.create_file("lib.rs".to_owned(), "".to_owned());
    }
}
