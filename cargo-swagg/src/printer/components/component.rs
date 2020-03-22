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
    Type {
        name: String,
        description: Option<String>,
        type_value: FieldType,
    },
}

impl Component {
    fn description(&self) -> Option<String> {
        match self {
            Component::Object { description, .. } => description.clone(),
            Component::Enum { description, .. } => description.clone(),
            Component::Type { description, .. } => description.clone(),
        }
    }

    fn name(&self) -> String {
        match self {
            Component::Object { name, .. } => name.clone(),
            Component::Enum { name, .. } => name.clone(),
            Component::Type { name, .. } => name.clone(),
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
                    #[derive(Debug, Serialize, Deserialize)]
                    pub struct #name_ident {
                        #fields_stream
                    }
                }
            }
            Component::Enum { variants, .. } => {
                let variants_stream = variants.print();

                quote! {
                    #description
                    #[derive(Debug, Serialize, Deserialize)]
                    pub enum #name_ident {
                        #variants_stream
                    }
                }
            }
            Component::Type { type_value, .. } => {
                let type_stream = type_value.print();

                quote! {
                    #description
                    pub type #name_ident = #type_stream;
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

    /// Should be used with `x-rust-type: crate::app::MyType`
    /// MyType must implement Debug, Serialize, Deserialize
    Internal(String),
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
            FieldType::Internal(name) => path_to_stream(name.clone()),
        }
    }
}

/// TODO: use https://docs.rs/itertools/0.9.0/itertools/trait.Itertools.html#method.fold1
/// https://t.me/rust_beginners_ru/57578
/// https://t.me/rust_beginners_ru/57579
fn path_to_stream(path: String) -> proc_macro2::TokenStream {
    if path.contains("::") {
        let mut parts = path.split("::");

        let first = parts.next().expect("Path split to parts requires first element");
        let first_ident = format_ident!("{}", first);

        let rest = parts.map(|p| format_ident!("{}", p));

        quote! {
            #first_ident #(::#rest)*
        }
    } else {
        // Can panic if identifier is incorrect.
        // TODO: add regexp check for input
        let ident = format_ident!("{}", path);
        quote! { #ident }
    }
}

pub enum NativeType {
    // Add minimum and maximum ranges
    // https://swagger.io/docs/specification/data-models/data-types/#numbers
    Integer { format: FormatInteger },
    Float { format: FormatFloat },
    String { format: FormatString },
    Boolean,
}

impl Printable for NativeType {
    fn print(&self) -> proc_macro2::TokenStream {
        match self {
            NativeType::Integer { format } => format.print(),
            NativeType::Float { format } => format.print(),
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
    Int32,
    Int64,
}

impl Default for FormatInteger {
    fn default() -> FormatInteger {
        FormatInteger::Int32
    }
}

impl Printable for FormatInteger {
    fn print(&self) -> proc_macro2::TokenStream {
        match self {
            FormatInteger::Int32 => quote! { i32 },
            FormatInteger::Int64 => quote! { i64 },
        }
    }
}

pub enum FormatFloat {
    Float,
    Double,
}

impl Default for FormatFloat {
    fn default() -> FormatFloat {
        FormatFloat::Float
    }
}

impl Printable for FormatFloat {
    fn print(&self) -> proc_macro2::TokenStream {
        match self {
            FormatFloat::Float => quote! { f32 },
            FormatFloat::Double => quote! { f64 },
        }
    }
}
