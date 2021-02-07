use openapiv3::{
    Header, OpenAPI, Operation, Parameter, ReferenceOr, RequestBody, Response, Schema,
    SecurityScheme,
};

mod actix;
mod resolver;

pub use openapiv3 as openapi;
use resolver::{resolve_reference, RefResolve};

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
pub struct Internal<'a> {
    files: std::collections::HashMap<String, String>,
    api: &'a OpenAPI,
}

impl<'a> Internal<'a> {
    fn new(api: &'a OpenAPI) -> Self {
        Self {
            api,
            files: Default::default(),
        }
    }
}

// impl<'a> Internal<'a> for Api<'a> {
impl<'a> Internal<'a> {
    pub fn create_file(&mut self, name: String, content: String) {
        self.files.insert(name, content);
    }

    pub fn resolve<T>(&'a self, source: &'a ReferenceOr<T>) -> Option<&'a T>
    where
        &'a T: RefResolve<'a>,
    {
        resolve_reference(source, &self.api)
    }

    pub fn root(&'a self) -> &'a OpenAPI {
        self.api
    }
}

trait Plugin<'a> {
    fn on_operation(
        &'a mut self,
        _method: Method,
        _path: &'a str,
        _operation: &'a Operation,
        _internal: &'a Internal<'a>,
    ) {
    }

    fn on_security_scheme(
        &'a mut self,
        _name: &'a str,
        _security_scheme: &'a SecurityScheme,
        _internal: &'a Internal<'a>,
    ) {
    }

    fn on_response(
        &'a mut self,
        _name: &'a str,
        _response: &'a Response,
        _internal: &'a Internal<'a>,
    ) {
    }

    fn on_parameter(
        &'a mut self,
        _name: &'a str,
        _parameter: &'a Parameter,
        _internal: &'a Internal<'a>,
    ) {
    }

    fn on_request_body(
        &'a mut self,
        _name: &'a str,
        _request_body: &'a RequestBody,
        _internal: &'a Internal<'a>,
    ) {
    }

    fn on_header(&'a mut self, _name: &'a str, _header: &'a Header, _internal: &'a Internal<'a>) {}

    fn on_schema(&'a mut self, _name: &'a str, _schema: &'a Schema, _internal: &'a Internal<'a>) {}

    /// Setup self with data from openapi before iterating over paths
    fn pre_paths(&'a mut self, _internal: &'a Internal<'a>) {}

    /// Collect data after iterating over paths
    fn post_paths(&'a mut self, _internal: &'a Internal<'a>) {}

    /// First setup before iterating over components
    fn pre_components(&'a mut self, _internal: &'a Internal<'a>) {}

    /// Finish collecting components
    fn post_components(&'a mut self, _internal: &'a Internal<'a>) {}

    fn proceed(&'a mut self, internal: &'a mut Internal<'a>) {
        internal.create_file("lib.rs".to_owned(), "".to_owned());
    }
}

fn main() {
    let schema = std::path::Path::new("./demo/openapi.yaml");
    let content = std::fs::read_to_string(&schema).expect("file found");
    let root: OpenAPI = serde_yaml::from_str(&content).expect("parsed to struct");

    let mut plugin = actix::ActixPlugin::new();
    let mut internal = Internal::new(&root);

    plugin.pre_components(&internal);
    if let Some(ref components) = root.components {
        for (name, security_scheme) in components.security_schemes.iter() {
            match resolve_reference(security_scheme, &root) {
                None => panic!("failed to resolve reference for security scheme '{}'", name),
                Some(schema) => plugin.on_security_scheme(name, schema, &internal),
            }
        }

        for (name, response) in components.responses.iter() {
            match resolve_reference(response, &root) {
                None => panic!("failed to resolve reference for response '{}'", name),
                Some(schema) => plugin.on_response(name, schema, &internal),
            }
        }

        for (name, parameter) in components.parameters.iter() {
            match resolve_reference(parameter, &root) {
                None => panic!("failed to resolve reference for parameter '{}'", name),
                Some(schema) => plugin.on_parameter(name, schema, &internal),
            }
        }

        for (name, request_body) in components.request_bodies.iter() {
            match resolve_reference(request_body, &root) {
                None => panic!("failed to resolve reference for request body '{}'", name),
                Some(schema) => plugin.on_request_body(name, schema, &internal),
            }
        }

        for (name, header) in components.headers.iter() {
            match resolve_reference(header, &root) {
                None => panic!("failed to resolve reference for header '{}'", name),
                Some(schema) => plugin.on_header(name, schema, &internal),
            }
        }

        for (name, schema) in components.schemas.iter() {
            match resolve_reference(schema, &root) {
                None => panic!("failed to resolve reference for schema '{}'", name),
                Some(schema) => plugin.on_schema(name, schema, &internal),
            }
        }
    }
    plugin.post_components(&internal);

    plugin.pre_paths(&internal);
    for (name, path) in root.paths.iter() {
        match resolve_reference(path, &root) {
            None => panic!("failed to resolve reference for '{}'", name),
            Some(path) => {
                if let Some(ref operation) = path.get {
                    plugin.on_operation(Method::Get, name, &operation, &internal);
                }
                if let Some(ref operation) = path.put {
                    plugin.on_operation(Method::Put, name, &operation, &internal);
                }
                if let Some(ref operation) = path.post {
                    plugin.on_operation(Method::Post, name, &operation, &internal);
                }
                if let Some(ref operation) = path.delete {
                    plugin.on_operation(Method::Delete, name, &operation, &internal);
                }
                if let Some(ref operation) = path.options {
                    plugin.on_operation(Method::Options, name, &operation, &internal);
                }
                if let Some(ref operation) = path.head {
                    plugin.on_operation(Method::Head, name, &operation, &internal);
                }
                if let Some(ref operation) = path.patch {
                    plugin.on_operation(Method::Patch, name, &operation, &internal);
                }
                if let Some(ref operation) = path.trace {
                    plugin.on_operation(Method::Trace, name, &operation, &internal);
                }
            }
        }
    }
    plugin.post_paths(&internal);

    plugin.proceed(&mut internal);
}
