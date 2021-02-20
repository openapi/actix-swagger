use openapiv3::{OpenAPI, ReferenceOr};

#[cfg(feature = "actix")]
use openapi_actix::ActixPlugin;

use openapi_hooks::{Internal, Method, Plugin};
use openapi_resolver::{resolve_reference, RefResolve};
pub use openapiv3 as openapi;

struct DefaultPlugin {}

impl<'a, I: Internal<'a>> Plugin<'a, I> for DefaultPlugin {}

pub struct InternalApi<'a> {
    files: std::collections::HashMap<String, String>,
    api: &'a OpenAPI,
}

impl<'a> InternalApi<'a> {
    fn new(api: &'a OpenAPI) -> Self {
        Self {
            api,
            files: Default::default(),
        }
    }
}

impl<'a> Internal<'a> for InternalApi<'a> {
    fn create_file(&mut self, name: String, content: String) {
        self.files.insert(name, content);
    }

    fn resolve<T>(&'a self, source: &'a ReferenceOr<T>) -> Option<&'a T>
    where
        &'a T: RefResolve<'a>,
    {
        resolve_reference(source, &self.api)
    }

    fn root(&'a self) -> &'a OpenAPI {
        self.api
    }
}

fn main() {
    let schema = std::path::Path::new("./demo/openapi.yaml");
    let content = std::fs::read_to_string(&schema).expect("file found");
    let root: OpenAPI = serde_yaml::from_str(&content).expect("parsed to struct");

    let mut internal = InternalApi::new(&root);

    #[cfg(feature = "actix")]
    let mut plugin = ActixPlugin::new();

    #[cfg(not(feature = "actix"))]
    let mut plugin = DefaultPlugin {};

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
