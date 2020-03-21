use crate::printer::Printable;
use inflections::Inflect;
use quote::{format_ident, quote};

pub enum Component {
    Object {
        name: String,
        description: Option<String>,
        fields: Vec<Field>,
    },
    Enum {
        name: String,
        description: Option<String>,
        variants: Vec<EnumVariant>,
    },
}

impl Component {
    fn description(&self) -> Option<String> {
        match self {
            Component::Object { description, .. } => description.clone(),
            Component::Enum { description, .. } => description.clone(),
        }
    }

    fn name(&self) -> String {
        match self {
            Component::Object { name, .. } => name.clone(),
            Component::Enum { name, .. } => name.clone(),
        }
    }
}

impl Printable for Component {
    fn print(&self) -> proc_macro2::TokenStream {
        let name_ident = format_ident!("{}", self.name().to_pascal_case());
        let description = match self.description() {
            Some(description) => quote! { #[doc = #description] },
            None => quote! {},
        };

        // implement validator::Validate

        match self {
            Component::Object { fields, .. } => {
                let fields_stream = fields.print();

                quote! {
                    #description
                    #[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
                    pub struct #name_ident {
                        #fields_stream
                    }
                }
            }
            Component::Enum { variants, .. } => {
                let variants_stream = variants.print();

                quote! {
                    #description
                    #[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
                    pub enum #name_ident {
                        #variants_stream
                    }
                }
            }
        }
    }
}

pub struct EnumVariant {
    pub name: String,
    pub description: Option<String>,
}

impl Printable for EnumVariant {
    fn print(&self) -> proc_macro2::TokenStream {
        let name_original = self.name.clone();
        let name_pascal = name_original.to_pascal_case();
        let name_ident = format_ident!("{}", name_pascal);

        let is_pascal_diffs = name_pascal != name_original;
        let rename = match is_pascal_diffs {
            true => quote! { #[serde(rename = #name_original)] },
            false => quote! {},
        };

        let description = match &self.description {
            Some(descr) => quote! { #[doc = #descr]},
            None => quote! {},
        };

        quote! {
            #description
            #rename
            #name_ident,
        }
    }
}

/// Field in object definition
pub struct Field {
    /// field name in any case
    pub name: String,

    pub required: bool,

    // Add support for nullable values
    // https://swagger.io/docs/specification/data-models/data-types/#null
    // pub nullable: bool,
    pub description: Option<String>,

    pub field_type: FieldType,
}

impl Printable for Field {
    fn print(&self) -> proc_macro2::TokenStream {
        let name_original = self.name.clone();
        let name_snake = name_original.to_snake_case();
        let name_ident = format_ident!("{}", name_snake);

        let is_snake_diffs = name_snake != name_original;
        let rename = match is_snake_diffs {
            true => quote! { #[serde(rename = #name_original)] },
            false => quote! {},
        };

        let description = match &self.description {
            Some(descr) => quote! { #[doc = #descr]},
            None => quote! {},
        };
        let type_stream = self.field_type.print();
        let type_value = match self.required {
            false => quote! { Option<#type_stream> },
            true => type_stream,
        };

        quote! {
            #description
            #rename
            pub #name_ident: #type_value,
        }
    }
}

pub enum FieldType {
    Native(NativeType),

    /// Name of the custom type
    Custom(String),

    Array(Box<FieldType>),
}

impl Printable for FieldType {
    fn print(&self) -> proc_macro2::TokenStream {
        match self {
            FieldType::Native(native) => native.print(),
            FieldType::Custom(name) => {
                let name_ident = format_ident!("{}", name);
                quote! { #name_ident }
            }
            FieldType::Array(inner_type) => {
                let inner_type_stream = inner_type.print();
                quote! { Vec<#inner_type_stream> }
            }
        }
    }
}

pub enum NativeType {
    Integer { format: FormatInteger },
    String { format: FormatString },
    Boolean,
}

impl Printable for NativeType {
    fn print(&self) -> proc_macro2::TokenStream {
        match self {
            NativeType::Integer { format } => format.print(),
            NativeType::String { format } => format.print(),
            NativeType::Boolean => quote! { bool },
        }
    }
}

pub enum FormatString {
    None,
    Binary,
    Byte,
    Date,
    DateTime,
    Email,
    Hostname,
    Ipv4,
    Ipv6,
    Password,
    Url,
    Uuid,
    Pattern(regex::Regex),
}

impl Default for FormatString {
    fn default() -> FormatString {
        FormatString::None
    }
}

impl Printable for FormatString {
    fn print(&self) -> proc_macro2::TokenStream {
        // Any string format now compiles to String
        quote! { String }
    }
}

pub enum FormatInteger {
    None,
    Int32,
    Int64,
}

impl Printable for FormatInteger {
    fn print(&self) -> proc_macro2::TokenStream {
        match self {
            FormatInteger::None => quote! { i32 },
            FormatInteger::Int32 => quote! { i32 },
            FormatInteger::Int64 => quote! { i64 },
        }
    }
}
