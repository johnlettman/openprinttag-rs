use crate::gen;

#[derive(Debug, Clone)]
pub enum FieldType {
    UUID,
    String(usize),
    Bytes(usize),

    Integer { unit: Option<String>, example: Option<u32> },

    Number { unit: Option<String>, example: Option<f32> },

    Timestamp,

    Enum { name: String, description: Option<String>, variants: crate::EnumVariants },

    EnumArray { name: String, description: Option<String>, variants: crate::EnumVariants },
}

impl FieldType {
    pub fn make_type(&self) -> crate::Result<syn::Type> {
        match self {
            Self::UUID => gen::make_type("uuid::Uuid"),
            Self::String(_) => gen::make_type("String"),
            Self::Bytes(len) => gen::make_array_type("u8", *len),
            Self::Integer { .. } => gen::make_type("u32"),
            Self::Number { .. } => gen::make_type("f32"),
            Self::Timestamp => gen::make_type("chrono::DateTime<chrono::Utc>"),
            Self::EnumArray { name, .. } | Self::Enum { name, .. } => gen::make_type(name),
        }
    }

    pub fn make_definitions(&self) -> Vec<syn::Item> {
        use syn::{punctuated::Punctuated, Generics, Item, ItemEnum};
        let mut definitions = Vec::new();

        match self {
            Self::UUID
            | Self::String(_)
            | Self::Bytes(_)
            | Self::Integer { .. }
            | Self::Number { .. }
            | Self::Timestamp => {},

            Self::Enum { name, description, variants }
            | Self::EnumArray { name, description, variants } => {
                let attrs = description
                    .as_ref()
                    .and_then(|d| gen::make_doc_attribute(d).ok())
                    .into_iter()
                    .collect::<Vec<_>>();

                let vis = gen::make_pub_visibility();
                let ident = gen::make_ident(name);

                let variants: Punctuated<_, _> =
                    variants.iter().filter_map(|v| v.make_variant(name)).collect();

                definitions.push(Item::Enum(ItemEnum {
                    attrs,
                    vis,
                    enum_token: Default::default(),
                    ident,
                    generics: Generics::default(),
                    variants,
                    brace_token: Default::default(),
                }));
            },
        }

        definitions
    }
}
