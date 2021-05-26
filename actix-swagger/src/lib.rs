#![deny(warnings)]

mod error;

pub use error::Error;

use actix_web::{
    cookie::Cookie,
    dev::{AppService, HttpServiceFactory},
    http::header::{self, IntoHeaderValue},
    http::{HeaderName, HeaderValue},
    web, FromRequest, HttpRequest, HttpResponse, Responder, Scope,
};
use serde::Serialize;
use std::collections::HashMap;

pub use actix_web::http::StatusCode;
use std::future::Future;
use actix_web::dev::Handler;

/// Set content-type supported by actix-swagger
#[derive(Debug)]
pub enum ContentType {
    Json,
    FormData,
    // TextPlain,
}

#[derive(Debug)]
pub enum Method {
    DELETE,
    GET,
    PATCH,
    POST,
    PUT,
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            ContentType::Json => "application/json".to_string(),
            // ContentType::TextPlain => "text/plain".to_string(),
            ContentType::FormData => "application/x-www-form-urlencoded".to_string(),
        }
    }
}

/// Strict answer to complain with generated code by cargo-swagg
pub struct Answer<'a, T> {
    response: T,
    status_code: Option<StatusCode>,
    cookies: Vec<Cookie<'a>>,
    headers: HashMap<String, HeaderValue>,
    content_type: Option<ContentType>,
}

impl<'a, T: Serialize> Answer<'a, T> {
    pub fn new(response: T) -> Answer<'a, T> {
        Answer {
            response,
            status_code: None,
            cookies: vec![],
            headers: HashMap::new(),
            content_type: None,
        }
    }

    /// Set header to answer
    pub fn header<V>(mut self, key: String, value: V) -> Self
    where
        V: IntoHeaderValue,
    {
        if let Ok(value) = value.try_into_value() {
            self.headers.insert(key, value);
        }

        self
    }

    /// Add cookie to answer
    pub fn cookie(mut self, cookie: Cookie<'a>) -> Self {
        self.cookies.push(cookie);

        self
    }

    /// Set status code
    pub fn status(mut self, status: StatusCode) -> Self {
        self.status_code = Some(status);

        self
    }

    /// Set content-type
    /// Content-Type changes serializer for answer
    pub fn content_type(mut self, content_type: Option<ContentType>) -> Self {
        self.content_type = content_type;

        self
    }

    /// Serialize answer
    pub fn to_string(&self) -> Result<String, Error> {
        match self.content_type {
            Some(ContentType::Json) => Ok(serde_json::to_string(&self.response)?),
            Some(ContentType::FormData) => Ok(serde_urlencoded::to_string(&self.response)?),
            // Some(ContentType::TextPlain) => Ok(serde_plain::to_string(&self.response)?),
            _ => Ok("".to_owned()),
        }
    }
}

impl<'a, T: Serialize> Responder for Answer<'a, T> {
    fn respond_to(self, _: &HttpRequest) -> HttpResponse {
        let body = match self.to_string() {
            Ok(body) => body,
            Err(e) => return HttpResponse::from_error(e)
        };

        let mut response = &mut HttpResponse::build(self.status_code.unwrap_or(StatusCode::OK));

        if let Some(content_type) = self.content_type {
            response = response.append_header((header::CONTENT_TYPE, content_type.to_string()));
        }

        for (name, value) in self.headers {
            if let Some(header_name) = name.parse::<HeaderName>().ok() {
                response = response.append_header((header_name, value))
            }
        }

        for cookie in self.cookies {
            response = response.cookie(cookie);
        }

        response.body(body)
    }
}

// https://actix.rs/docs/errors/

/// Handler scope and routes
pub struct Api {
    root: Scope,
    routes: HashMap<String, Scope>,
}

impl Api {
    pub fn new() -> Self {
        Api {
            root: Scope::new("/"),
            routes: HashMap::new(),
        }
    }

    /// Attach route to path
    pub fn bind<T, F, R>(mut self, path: String, method: Method, handler: F) -> Self
    where
        T: FromRequest + 'static,
        R: Future + 'static,
        R::Output: Responder + 'static,
        F: Handler<T, R>
    {
        let scope_path = path.clone();
        take_mut::take(
            self.routes
                .entry(path)
                .or_insert_with(move || web::scope(scope_path.as_ref())),
            |scope| {
                scope.route(
                    "",
                    match method {
                        Method::DELETE => web::delete(),
                        Method::GET => web::get(),
                        Method::PATCH => web::patch(),
                        Method::POST => web::post(),
                        Method::PUT => web::put(),
                    }
                    .to(handler),
                )
            },
        );

        self
    }
}

impl HttpServiceFactory for Api {
    fn register(mut self, config: &mut AppService) {
        let keys: Vec<String> = self.routes.keys().cloned().collect();

        for key in keys.iter() {
            if let Some(resource) = self.routes.remove(key) {
                self.root = self.root.service(resource);
            }
        }

        self.root.register(config);
    }
}
