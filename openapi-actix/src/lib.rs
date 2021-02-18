use openapi_hooks::{
    v3::{Header, Operation, Parameter, RequestBody, Response, Schema},
    Hooks, Internal, Method,
};

pub struct ActixPlugin {
    schemas: Vec<(String, String)>,
}

impl ActixPlugin {
    pub fn new() -> Self {
        Self {
            schemas: Vec::new(),
        }
    }
}

impl<'a, I: Internal<'a>> Hooks<'a, I> for ActixPlugin {
    fn on_operation(
        &'a mut self,
        method: Method,
        path: &'a str,
        _operation: &'a Operation,
        _internal: &'a I,
    ) {
        println!("(actix) [operation] {} {:?}", path, method);
    }

    fn on_header(&'a mut self, name: &'a str, _header: &'a Header, _internal: &'a I) {
        println!("(actix) [header] {}", name);
        self.schemas.push(("header".to_owned(), name.to_owned()));
    }

    fn on_parameter(&'a mut self, name: &'a str, _parameter: &'a Parameter, _internal: &'a I) {
        println!("(actix) [parameter] {}", name);
        self.schemas.push(("parameter".to_owned(), name.to_owned()));
    }

    fn on_request_body(
        &'a mut self,
        name: &'a str,
        _request_body: &'a RequestBody,
        _internal: &'a I,
    ) {
        println!("(actix) [request_body] {}", name);
        self.schemas
            .push(("request_body".to_owned(), name.to_owned()));
    }

    fn on_response(&'a mut self, name: &'a str, _response: &'a Response, _internal: &'a I) {
        self.schemas.push(("response".to_owned(), name.to_owned()));
    }

    fn on_schema(&'a mut self, name: &'a str, _schema: &'a Schema, _internal: &'a I) {
        self.schemas.push(("schema".to_owned(), name.to_owned()));
    }

    fn post_components(&'a mut self, _internal: &'a I) {
        self.schemas.sort();
        println!(
            "(actix) [POST] schemas: {:#?}",
            self.schemas
                .iter()
                .map(|(typ, nam)| format!("{} {}", typ, nam))
                .collect::<Vec<_>>()
        );
    }

    fn pre_components(&'a mut self, internal: &'a I) {
        println!("(actix) [PRE] components");
        println!(
            "{} {}",
            internal.root().info.title,
            internal.root().info.version
        );
    }

    fn proceed(&'a mut self, internal: &'a mut I) {
        internal.create_file("lib.rs".to_owned(), "".to_owned());
    }
}
