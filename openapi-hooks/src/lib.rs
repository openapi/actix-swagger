use openapi_resolver::RefResolve;
use openapiv3::{
    Header, OpenAPI, Operation, Parameter, ReferenceOr, RequestBody, Response, Schema,
    SecurityScheme,
};

pub use openapiv3 as v3;

#[derive(Debug)]
pub enum Method {
    Get,
    Put,
    Post,
    Delete,
    Options,
    Head,
    Patch,
    Trace,
}

pub trait Internal<'a> {
    fn create_file(&mut self, name: String, content: String);

    fn resolve<T>(&'a self, source: &'a ReferenceOr<T>) -> Option<&'a T>
    where
        &'a T: RefResolve<'a>;

    fn root(&'a self) -> &'a OpenAPI;
}

pub trait Plugin<'a, I: Internal<'a>> {
    fn on_operation(
        &'a mut self,
        _method: Method,
        _path: &'a str,
        _operation: &'a Operation,
        _internal: &'a I,
    ) {
    }

    fn on_security_scheme(
        &'a mut self,
        _name: &'a str,
        _security_scheme: &'a SecurityScheme,
        _internal: &'a I,
    ) {
    }

    fn on_response(&'a mut self, _name: &'a str, _response: &'a Response, _internal: &'a I) {}

    fn on_parameter(&'a mut self, _name: &'a str, _parameter: &'a Parameter, _internal: &'a I) {}

    fn on_request_body(
        &'a mut self,
        _name: &'a str,
        _request_body: &'a RequestBody,
        _internal: &'a I,
    ) {
    }

    fn on_header(&'a mut self, _name: &'a str, _header: &'a Header, _internal: &'a I) {}

    fn on_schema(&'a mut self, _name: &'a str, _schema: &'a Schema, _internal: &'a I) {}

    /// Setup self with data from openapi before iterating over paths
    fn pre_paths(&'a mut self, _internal: &'a I) {}

    /// Collect data after iterating over paths
    fn post_paths(&'a mut self, _internal: &'a I) {}

    /// First setup before iterating over components
    fn pre_components(&'a mut self, _internal: &'a I) {}

    /// Finish collecting components
    fn post_components(&'a mut self, _internal: &'a I) {}

    fn proceed(&'a mut self, internal: &'a mut I) {
        internal.create_file("lib.rs".to_owned(), "".to_owned());
    }
}
