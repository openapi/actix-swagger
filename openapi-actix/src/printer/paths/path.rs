use super::ResponseStatus;
use crate::printer::Printable;
use inflections::Inflect;
use quote::{format_ident, quote};

pub struct Path {
    pub name: String,
    pub response: ResponseEnum,
    pub query_params: Vec<QueryParam>,
}

impl Path {
    fn print_enum_variants(&self) -> proc_macro2::TokenStream {
        let variants = self
            .response
            .responses
            .iter()
            .map(|r| r.print_enum_variant());

        quote! { #(#variants,)* }
    }

    fn print_status_variants(&self) -> proc_macro2::TokenStream {
        let variants = self
            .response
            .responses
            .iter()
            .map(|r| r.print_status_variant());
        let tokens = quote! { #(#variants,)* };

        quote! {
            match self {
                #tokens
            }
        }
    }

    fn print_content_type_variants(&self) -> proc_macro2::TokenStream {
        let variants = self
            .response
            .responses
            .iter()
            .map(|r| r.print_content_type_variant());
        let tokens = quote! { #(#variants,)* };

        quote! {
            match self {
                #tokens
            }
        }
    }

    fn query_params_impl(&self) -> proc_macro2::TokenStream {
        if self.query_params.is_empty() {
            quote! {}
        } else {
            let query_params = self.query_params.print();

            quote! {
                use super::parameters;

                #[derive(Debug, Deserialize)]
                pub struct QueryParams {
                    #query_params
                }

                pub type Query = actix_web::http::Query<QueryParams>;
            }
        }
    }
}

impl Printable for Path {
    fn print(&self) -> proc_macro2::TokenStream {
        let module_name = format_ident!("{}", self.name.to_snake_case());
        let enum_variants = self.print_enum_variants();
        let status_match = self.print_status_variants();
        let content_type_match = self.print_content_type_variants();
        let query_params = self.query_params_impl();

        quote! {
            pub mod #module_name {
                use super::responses;
                use actix_swagger::{Answer, ContentType, StatusCode};
                use serde::{Serialize, Deserialize};

                #[derive(Debug, Serialize)]
                #[serde(untagged)]
                pub enum Response {
                    #enum_variants
                }

                impl Response {
                    #[inline]
                    pub fn to_answer(self) -> Answer<'static, Self> {
                        let status = #status_match;
                        let content_type = #content_type_match;

                        Answer::new(self).status(status).content_type(content_type)
                    }
                }

                #query_params
            }
        }

        /*
            impl<'a> Into<Answer<'a, Response>> for Response {
                #[inline]
                fn into(self: Response) -> Answer<'a, Response> {
                    self.to_answer()
                }
            }
        */
    }
}

pub struct ResponseEnum {
    pub responses: Vec<StatusVariant>,
}

pub struct StatusVariant {
    pub status: ResponseStatus,

    /// Should be in `#/components/responses/`
    pub response_type_name: Option<String>,

    /// Comment for response status
    pub description: Option<String>,

    /// Now supports only one content type per response
    pub content_type: Option<ContentType>,

    /// Variant can be renamed with `x-variant-name`
    pub x_variant_name: Option<String>,
}

impl StatusVariant {
    pub fn name(&self) -> proc_macro2::Ident {
        let name = self
            .x_variant_name
            .clone()
            .unwrap_or(self.status.to_string());
        format_ident!("{}", name.to_pascal_case())
    }

    pub fn description(&self) -> proc_macro2::TokenStream {
        match &self.description {
            Some(text) => quote! { #[doc = #text] },
            None => quote! {},
        }
    }

    pub fn content_type(&self) -> proc_macro2::TokenStream {
        match self.content_type.clone() {
            Some(t) => {
                let content = t.print();
                quote! { Some(ContentType::#content) }
            }
            None => quote! { None },
        }
    }

    pub fn print_enum_variant(&self) -> proc_macro2::TokenStream {
        let description = self.description();
        let variant_name = self.name();

        if let Some(response) = self.response_type_name.clone() {
            let response_name = format_ident!("{}", response.to_pascal_case());

            quote! {
                #description
                #variant_name(responses::#response_name)
            }
        } else {
            quote! {
                #description
                #variant_name
            }
        }
    }

    pub fn print_status_variant(&self) -> proc_macro2::TokenStream {
        let variant_name = self.name();
        let status = format_ident!("{}", self.status.to_string().to_constant_case());

        match self.response_type_name {
            Some(_) => quote! { Self::#variant_name(_) => StatusCode::#status },
            None => quote! { Self::#variant_name => StatusCode::#status },
        }
    }

    pub fn print_content_type_variant(&self) -> proc_macro2::TokenStream {
        let variant_name = self.name();
        let content_type = self.content_type();

        match self.response_type_name {
            Some(_) => quote! { Self::#variant_name(_) => #content_type },
            None => quote! { Self::#variant_name => #content_type },
        }
    }
}

#[derive(Debug, Clone)]
pub enum ContentType {
    Json,
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            ContentType::Json => "Json",
        }
        .to_owned()
    }
}

impl Printable for ContentType {
    fn print(&self) -> proc_macro2::TokenStream {
        let ident = format_ident!("{}", self.to_string());

        quote! { #ident }
    }
}

pub struct QueryParam {
    /// Name of the parameter in the query, can be in any case, will be converted to snake_case
    pub name: String,

    /// should be reference to type in `components::parameters` module
    /// Will be converted to PascalCase
    pub type_ref: String,

    pub description: Option<String>,

    pub required: bool,
}

impl Printable for QueryParam {
    fn print(&self) -> proc_macro2::TokenStream {
        let name_original = self.name.clone();
        let name_snake = name_original.to_snake_case();
        let name_ident = format_ident!("{}", name_snake);
        let rename = match name_snake != name_original {
            true => quote! { #[serde(rename = #name_original)] },
            false => quote! {},
        };

        let type_name = format_ident!("{}", self.type_ref.to_pascal_case());
        let description = match &self.description {
            Some(description) => quote! { #[doc = #description]},
            None => quote! {},
        };

        let type_result = match self.required {
            true => quote! { parameters::#type_name },
            false => quote! { Option<parameters::#type_name> },
        };

        quote! {
            #description
            #rename
            pub #name_ident: #type_result,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::shot;
    use insta::assert_snapshot;

    #[test]
    fn path_empty() {
        assert_snapshot!(shot(Path {
            name: "Example".to_owned(),
            response: ResponseEnum {
                responses: vec![]
            },
            query_params: vec![]
        }), @r###"
        pub mod example {
            use super::responses;
            use actix_swagger::{Answer, ContentType, StatusCode};
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Serialize)]
            #[serde(untagged)]
            pub enum Response {}
            impl Response {
                #[inline]
                pub fn to_answer(self) -> Answer<'static, Self> {
                    let status = match self {};
                    let content_type = match self {};
                    Answer::new(self).status(status).content_type(content_type)
                }
            }
        }
        "###);
    }

    #[test]
    fn path_with_responses() {
        assert_snapshot!(shot(Path {
            name: "Example".to_owned(),
            response: ResponseEnum {
                responses: vec![
                    StatusVariant {
                        status: ResponseStatus::Ok,
                        content_type: None,
                        response_type_name: None,
                        description: None,
                        x_variant_name: None,
                    },
                    StatusVariant {
                        status: ResponseStatus::Created,
                        content_type: Some(ContentType::Json),
                        response_type_name: None,
                        description: None,
                        x_variant_name: None,
                    },
                    StatusVariant {
                        status: ResponseStatus::Accepted,
                        content_type: Some(ContentType::Json),
                        response_type_name: Some("unexpected_FRIEND_Name".to_owned()),
                        description: None,
                        x_variant_name: None,
                    },
                    StatusVariant {
                        status: ResponseStatus::BadRequest,
                        content_type: Some(ContentType::Json),
                        response_type_name: Some("unexpected_FRIEND_Name".to_owned()),
                        description: Some("My super simple description.\nAnother back".to_owned()),
                        x_variant_name: None,
                    },
                    StatusVariant {
                        status: ResponseStatus::InternalServerError,
                        content_type: Some(ContentType::Json),
                        response_type_name: Some("unexpected_FRIEND_Name".to_owned()),
                        description: Some("My super simple description.\nAnother back".to_owned()),
                        x_variant_name: Some("Unexpected".to_owned()),
                    },
                    StatusVariant {
                        status: ResponseStatus::ExpectationFailed,
                        content_type: None,
                        response_type_name: Some("unexpected_FRIEND_Name".to_owned()),
                        description: Some("My super simple description.\nAnother back".to_owned()),
                        x_variant_name: Some("Expectation".to_owned()),
                    },
                    StatusVariant {
                        status: ResponseStatus::NotFound,
                        content_type: None,
                        response_type_name: None,
                        description: Some("My super simple description.\nAnother back".to_owned()),
                        x_variant_name: Some("No".to_owned()),
                    },
                ]
            },
            query_params: vec![]
        }), @r###"
        pub mod example {
            use super::responses;
            use actix_swagger::{Answer, ContentType, StatusCode};
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Serialize)]
            #[serde(untagged)]
            pub enum Response {
                Ok,
                Created,
                Accepted(responses::UnexpectedFriendName),
                #[doc = "My super simple description.\nAnother back"]
                BadRequest(responses::UnexpectedFriendName),
                #[doc = "My super simple description.\nAnother back"]
                Unexpected(responses::UnexpectedFriendName),
                #[doc = "My super simple description.\nAnother back"]
                Expectation(responses::UnexpectedFriendName),
                #[doc = "My super simple description.\nAnother back"]
                No,
            }
            impl Response {
                #[inline]
                pub fn to_answer(self) -> Answer<'static, Self> {
                    let status = match self {
                        Self::Ok => StatusCode::OK,
                        Self::Created => StatusCode::CREATED,
                        Self::Accepted(_) => StatusCode::ACCEPTED,
                        Self::BadRequest(_) => StatusCode::BAD_REQUEST,
                        Self::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
                        Self::Expectation(_) => StatusCode::EXPECTATION_FAILED,
                        Self::No => StatusCode::NOT_FOUND,
                    };
                    let content_type = match self {
                        Self::Ok => None,
                        Self::Created => Some(ContentType::Json),
                        Self::Accepted(_) => Some(ContentType::Json),
                        Self::BadRequest(_) => Some(ContentType::Json),
                        Self::Unexpected(_) => Some(ContentType::Json),
                        Self::Expectation(_) => None,
                        Self::No => None,
                    };
                    Answer::new(self).status(status).content_type(content_type)
                }
            }
        }
        "###);
    }

    #[test]
    fn path_with_query_params() {
        assert_snapshot!(shot(Path {
            name: "Example".to_owned(),
            response: ResponseEnum {
                responses: vec![]
            },
            query_params: vec![
                QueryParam {
                    name: "simple_LONG_DescriptionFor-Me".to_owned(),
                    description: None,
                    required: false,
                    type_ref: "simple_LONG_DescriptionFor-Me".to_owned()
                },
                QueryParam {
                    name: "ARE_YOU_SURE".to_owned(),
                    description: Some("This is the description".to_owned()),
                    required: false,
                    type_ref: "simple_LONG_DescriptionFor-Me".to_owned()
                },
                QueryParam {
                    name: "just-required".to_owned(),
                    description: None,
                    required: true,
                    type_ref: "Another".to_owned()
                },
            ]
        }), @r###"
        pub mod example {
            use super::responses;
            use actix_swagger::{Answer, ContentType, StatusCode};
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Serialize)]
            #[serde(untagged)]
            pub enum Response {}
            impl Response {
                #[inline]
                pub fn to_answer(self) -> Answer<'static, Self> {
                    let status = match self {};
                    let content_type = match self {};
                    Answer::new(self).status(status).content_type(content_type)
                }
            }
            use super::parameters;
            #[derive(Debug, Deserialize)]
            pub struct QueryParams {
                #[serde(rename = "simple_LONG_DescriptionFor-Me")]
                pub simple_long_description_for_me: Option<parameters::SimpleLongDescriptionForMe>,
                #[doc = "This is the description"]
                #[serde(rename = "ARE_YOU_SURE")]
                pub are_you_sure: Option<parameters::SimpleLongDescriptionForMe>,
                #[serde(rename = "just-required")]
                pub just_required: parameters::Another,
            }
            pub type Query = actix_web::http::Query<QueryParams>;
        }
        "###);
    }
}
