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

impl FieldType {
    pub fn native_float(format: FormatFloat) -> Self {
        Self::Native(NativeType::Float { format })
    }

    pub fn native_integer(format: FormatInteger) -> Self {
        Self::Native(NativeType::Integer { format })
    }

    pub fn native_string(format: FormatString) -> Self {
        Self::Native(NativeType::String { format })
    }

    pub fn native_boolean() -> Self {
        Self::Native(NativeType::Boolean)
    }

    pub fn array(item: FieldType) -> Self {
        Self::Array(Box::new(item))
    }
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

        let first = parts
            .next()
            .expect("Path split to parts requires first element");
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::shot;
    use insta::assert_snapshot;

    struct TestType<T>(T);

    impl<T: crate::printer::Printable> crate::printer::Printable for TestType<T> {
        fn print(&self) -> proc_macro2::TokenStream {
            let t = self.0.print();
            quote::quote! {
                type Test = #t;
            }
        }
    }

    #[test]
    fn format_float() {
        assert_snapshot!(shot(TestType(FormatFloat::default())), @r"type Test = f32;");
        assert_snapshot!(shot(TestType(FormatFloat::Float)), @r"type Test = f32;");
        assert_snapshot!(shot(TestType(FormatFloat::Double)), @r"type Test = f64;");
    }

    #[test]
    fn format_integer() {
        assert_snapshot!(shot(TestType(FormatInteger::default())), @r###"type Test = i32;"###);
        assert_snapshot!(shot(TestType(FormatInteger::Int32)), @r###"type Test = i32;"###);
        assert_snapshot!(shot(TestType(FormatInteger::Int64)), @r###"type Test = i64;"###);
    }

    #[test]
    fn format_string() {
        assert_snapshot!(shot(TestType(FormatString::default())), @"type Test = String;
");
        assert_snapshot!(shot(TestType(FormatString::None)), @"type Test = String;
");
        assert_snapshot!(shot(TestType(FormatString::Binary)), @"type Test = String;
");
        assert_snapshot!(shot(TestType(FormatString::Byte)), @"type Test = String;
");
        assert_snapshot!(shot(TestType(FormatString::Date)), @"type Test = String;
");
        assert_snapshot!(shot(TestType(FormatString::DateTime)), @"type Test = String;
");
        assert_snapshot!(shot(TestType(FormatString::Email)), @"type Test = String;
");
        assert_snapshot!(shot(TestType(FormatString::Hostname)), @"type Test = String;
");
        assert_snapshot!(shot(TestType(FormatString::Ipv4)), @"type Test = String;
");
        assert_snapshot!(shot(TestType(FormatString::Ipv6)), @"type Test = String;
");
        assert_snapshot!(shot(TestType(FormatString::Password)), @"type Test = String;
");
        assert_snapshot!(shot(TestType(FormatString::Url)), @"type Test = String;
");
        assert_snapshot!(shot(TestType(FormatString::Uuid)), @"type Test = String;
");
        assert_snapshot!(shot(TestType(FormatString::Pattern(regex::Regex::new(".*").unwrap()))), @"type Test = String;
");
    }

    #[test]
    fn native_type_string() {
        assert_snapshot!(shot(TestType(NativeType::String { format: Default::default() })), @"type Test = String;
");
    }

    #[test]
    fn native_type_float() {
        assert_snapshot!(shot(TestType(NativeType::Float { format: Default::default() })), @"type Test = f32;
");
    }

    #[test]
    fn native_type_integer() {
        assert_snapshot!(shot(TestType(NativeType::Integer { format: Default::default() })), @"type Test = i32;
");
    }

    #[test]
    fn native_type_boolean() {
        assert_snapshot!(shot(TestType(NativeType::Boolean)), @"type Test = bool;
");
    }

    #[test]
    fn convert_path_to_stream() {
        assert_snapshot!(path_to_stream("crate::app::Type".to_owned()).to_string(), @"crate :: app :: Type");
        assert_snapshot!(path_to_stream("Type".to_owned()).to_string(), @"Type");
        assert_snapshot!(path_to_stream("super::super::app::Type".to_owned()).to_string(), @"super :: super :: app :: Type");
    }

    #[test]
    fn field_type_native() {
        assert_snapshot!(shot(TestType(FieldType::Native(NativeType::Boolean))), @r"type Test = bool;");
        assert_snapshot!(shot(TestType(FieldType::Native(NativeType::Float { format: FormatFloat::Double }))), @r"type Test = f64;");
        assert_snapshot!(shot(TestType(FieldType::Native(NativeType::String { format: FormatString::Email }))), @r"type Test = String;");
    }

    #[test]
    fn field_type_custom() {
        assert_snapshot!(shot(TestType(FieldType::Custom("MySuperType".to_owned()))), @r"type Test = MySuperType;");
        assert_snapshot!(shot(TestType(FieldType::Custom("i32".to_owned()))), @r"type Test = i32;");
    }

    #[test]
    #[should_panic]
    fn field_type_custom_disallow_path() {
        assert_snapshot!(shot(TestType(FieldType::Custom("some::Example".to_owned()))), @r"");
    }

    #[test]
    fn field_type_internal_allow_path() {
        assert_snapshot!(shot(TestType(FieldType::Internal("some::Example".to_owned()))), @r"type Test = some::Example;");
    }

    #[test]
    fn field_type_array() {
        assert_snapshot!(shot(TestType(FieldType::Array(Box::new(FieldType::Native(NativeType::Boolean))))), @r"type Test = Vec<bool>;");
        assert_snapshot!(shot(TestType(FieldType::Array(Box::new(FieldType::Array(Box::new(FieldType::Native(NativeType::Boolean))))))), @r"type Test = Vec<Vec<bool>>;");
        assert_snapshot!(shot(TestType(FieldType::Array(Box::new(FieldType::Array(Box::new(FieldType::Custom("Super".to_owned()))))))), @r"type Test = Vec<Vec<Super>>;");
        assert_snapshot!(shot(TestType(FieldType::Array(Box::new(FieldType::Array(Box::new(FieldType::Internal("crate::Super".to_owned()))))))), @r"type Test = Vec<Vec<crate::Super>>;");
    }

    #[test]
    fn component_type() {
        assert_snapshot!(shot(Component::Type {
            name: "snake_case_name".to_owned(),
            description: None,
            type_value: FieldType::Internal("super::another::Type".to_owned()),
        }), @"pub type SnakeCaseName = super::another::Type;");

        assert_snapshot!(shot(Component::Type {
            name: "UPPER_CASE_NAME".to_owned(),
            description: Some("Example description for test type export".to_owned()),
            type_value: FieldType::Internal("super::another::Type".to_owned()),
        }), @r###"
        #[doc = "Example description for test type export"]
        pub type UpperCaseName = super::another::Type;
        "###);
    }

    #[test]
    fn component_object() {
        assert_snapshot!(shot(Component::Object {
            name: "snake_case_name".to_owned(),
            description: None,
            fields: vec![],
        }), @r###"
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SnakeCaseName {}
        "###);

        assert_snapshot!(shot(Component::Object {
            name: "UPPER_CASE_NAME".to_owned(),
            description: Some("My super long description.\nOr not".to_owned()),
            fields: vec![],
        }), @r###"
        #[doc = "My super long description.\nOr not"]
        #[derive(Debug, Serialize, Deserialize)]
        pub struct UpperCaseName {}
        "###);

        assert_snapshot!(shot(Component::Object {
            name: "THIS-IS-FIELDS".to_owned(),
            description: None,
            fields: vec![Field {
                name: "UPPER_CASE_FIELD".to_owned(),
                description: Some("Description".to_owned()),
                required: true,
                field_type: FieldType::Native(NativeType::String { format: Default::default() })
            },
            Field {
                name: "snake_case_field".to_owned(),
                description: None,
                required: true,
                field_type: FieldType::Native(NativeType::Integer { format: FormatInteger::Int64 })
            },
            Field {
                name: "superCase".to_owned(),
                description: None,
                required: false,
                field_type: FieldType::Internal("super::super::app::Type".to_owned()),
            },
            Field {
                name: "JustAnother".to_owned(),
                description: Some("".to_owned()),
                required: false,
                field_type: FieldType::Array(Box::new(FieldType::Internal("i128".to_owned())))
            }],
        }), @r###"
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ThisIsFields {
            #[doc = "Description"]
            #[serde(rename = "UPPER_CASE_FIELD")]
            pub upper_case_field: String,
            pub snake_case_field: i64,
            #[serde(rename = "superCase")]
            pub super_case: Option<super::super::app::Type>,
            #[doc = ""]
            #[serde(rename = "JustAnother")]
            pub just_another: Option<Vec<i128>>,
        }
        "###);
    }

    #[test]
    fn component_enum() {
        assert_snapshot!(shot(Component::Enum {
            name: "snake_case_name".to_owned(),
            description: None,
            variants: vec![],
        }), @r###"
        #[derive(Debug, Serialize, Deserialize)]
        pub enum SnakeCaseName {}
        "###);

        assert_snapshot!(shot(Component::Enum {
            name: "UPPER_CASE_NAME".to_owned(),
            description: Some("My super long description.\nOr not".to_owned()),
            variants: vec![],
        }), @r###"
        #[doc = "My super long description.\nOr not"]
        #[derive(Debug, Serialize, Deserialize)]
        pub enum UpperCaseName {}
        "###);

        assert_snapshot!(shot(Component::Enum {
            name: "THIS-IS-FIELDS".to_owned(),
            description: None,
            variants: vec![EnumVariant {
                name: "UPPER_CASE_FIELD".to_owned(),
                description: Some("Description".to_owned()),
            },
            EnumVariant {
                name: "snake_case_field".to_owned(),
                description: None,
            },
            EnumVariant {
                name: "superCase".to_owned(),
                description: None,
            },
            EnumVariant {
                name: "JustAnother".to_owned(),
                description: Some("".to_owned()),
            }],
        }), @r###"
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ThisIsFields {
            #[doc = "Description"]
            #[serde(rename = "UPPER_CASE_FIELD")]
            UpperCaseField,
            #[serde(rename = "snake_case_field")]
            SnakeCaseField,
            #[serde(rename = "superCase")]
            SuperCase,
            #[doc = ""]
            JustAnother,
        }
        "###);
    }
}
