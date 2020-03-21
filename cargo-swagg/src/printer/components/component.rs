pub enum Component {
    Object {
        name: String,
        fields: Vec<Field>,
    },
    Enum {
        name: String,
        variants: Vec<EnumVariant>,
    },
}

pub struct EnumVariant {
    pub name: String,
}

/// Field in object definition
pub struct Field {
    /// field name in any case
    pub name: String,

    pub required: bool,

    pub description: Option<String>,

    pub field_type: FieldType,
}

pub enum FieldType {
    Native(NativeType),

    /// Name of the custom type
    Custom(String),
}

pub enum NativeType {
    Integer { format: FormatInteger },
    String { format: FormatString },
    Boolean,
}

pub enum FormatString {
    None,
    Url,
}

pub enum FormatInteger {
    None,
    Int32,
}
