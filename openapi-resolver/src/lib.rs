use openapiv3::{
    Header, OpenAPI, Parameter, PathItem, ReferenceOr, RequestBody, Response, Schema,
    SecurityScheme,
};

pub fn resolve_reference<'a, T>(source: &'a ReferenceOr<T>, root: &'a OpenAPI) -> Option<&'a T>
where
    &'a T: RefResolve<'a>,
{
    match source {
        ReferenceOr::Item(item) => Some(item),
        ReferenceOr::Reference { reference } => RefResolve::resolve(&root, reference),
    }
}

pub trait RefResolve<'d>: Sized {
    fn resolve(api: &'d OpenAPI, link: &'d str) -> Option<Self>;
}

impl<'a> RefResolve<'a> for &'a Response {
    fn resolve(api: &'a OpenAPI, link: &'a str) -> Option<Self> {
        if let Some(name) = link.strip_prefix("#/components/responses/") {
            match &api.components {
                Some(components) => match components.responses.get(name) {
                    Some(ReferenceOr::Reference { reference }) => Self::resolve(api, reference),
                    Some(ReferenceOr::Item(item)) => Some(item),
                    None => None,
                },
                None => None,
            }
        } else {
            None
        }
    }
}

impl<'a> RefResolve<'a> for &'a Parameter {
    fn resolve(api: &'a OpenAPI, link: &'a str) -> Option<Self> {
        if let Some(name) = link.strip_prefix("#/components/parameters/") {
            match &api.components {
                Some(components) => match components.parameters.get(name) {
                    Some(ReferenceOr::Reference { reference }) => Self::resolve(api, reference),
                    Some(ReferenceOr::Item(item)) => Some(item),
                    None => None,
                },
                None => None,
            }
        } else {
            None
        }
    }
}

impl<'a> RefResolve<'a> for &'a SecurityScheme {
    fn resolve(api: &'a OpenAPI, link: &'a str) -> Option<Self> {
        if let Some(name) = link.strip_prefix("#/components/securitySchemes/") {
            match &api.components {
                Some(components) => match components.security_schemes.get(name) {
                    Some(ReferenceOr::Reference { reference }) => Self::resolve(api, reference),
                    Some(ReferenceOr::Item(item)) => Some(item),
                    None => None,
                },
                None => None,
            }
        } else {
            None
        }
    }
}

impl<'a> RefResolve<'a> for &'a RequestBody {
    fn resolve(api: &'a OpenAPI, link: &'a str) -> Option<Self> {
        if let Some(name) = link.strip_prefix("#/components/requestBodies/") {
            match &api.components {
                Some(components) => match components.request_bodies.get(name) {
                    Some(ReferenceOr::Reference { reference }) => Self::resolve(api, reference),
                    Some(ReferenceOr::Item(item)) => Some(item),
                    None => None,
                },
                None => None,
            }
        } else {
            None
        }
    }
}

impl<'a> RefResolve<'a> for &'a Header {
    fn resolve(api: &'a OpenAPI, link: &'a str) -> Option<Self> {
        if let Some(name) = link.strip_prefix("#/components/headers/") {
            match &api.components {
                Some(components) => match components.headers.get(name) {
                    Some(ReferenceOr::Reference { reference }) => Self::resolve(api, reference),
                    Some(ReferenceOr::Item(item)) => Some(item),
                    None => None,
                },
                None => None,
            }
        } else {
            None
        }
    }
}

impl<'a> RefResolve<'a> for &'a Schema {
    fn resolve(api: &'a OpenAPI, link: &'a str) -> Option<Self> {
        if let Some(name) = link.strip_prefix("#/components/schemas/") {
            match &api.components {
                Some(components) => match components.schemas.get(name) {
                    Some(ReferenceOr::Reference { reference }) => Self::resolve(api, reference),
                    Some(ReferenceOr::Item(item)) => Some(item),
                    None => None,
                },
                None => None,
            }
        } else {
            None
        }
    }
}

impl<'a> RefResolve<'a> for &'a PathItem {
    fn resolve(api: &'a OpenAPI, link: &'a str) -> Option<Self> {
        if let Some(name) = link.strip_prefix("#/paths/") {
            match api.paths.get(name) {
                Some(ReferenceOr::Reference { reference }) => Self::resolve(api, reference),
                Some(ReferenceOr::Item(item)) => Some(item),
                None => None,
            }
        } else {
            None
        }
    }
}
