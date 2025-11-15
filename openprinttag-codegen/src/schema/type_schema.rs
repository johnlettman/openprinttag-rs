use crate::schema::{
    gen::{util::make_type, GetType},
    name,
};
use proc_macro2::Span;
use syn::{parse_quote, Type};

#[derive(Debug, Clone)]
pub enum TypeSchema {
    Struct(String),
    Enum(String),
    EnumArray(String, usize),
    UUID,
    String(usize),
    Bytes(usize),
    Integer { unit: Option<String>, example: Option<u32> },
    Number { unit: Option<String>, example: Option<f32> },
    Timestamp,
    None,
}

impl GetType for TypeSchema {
    fn get_type(&self) -> Option<Type> {
        match self {
            Self::Struct(s) | Self::Enum(s) => Some(make_type(s).ok()?),
            Self::EnumArray(e, size) => {
                let lit = syn::LitInt::new(&size.to_string(), Span::call_site());
                let ident = name::to_ident(e);
                Some(parse_quote!(EnumArray<#ident, #lit>))
            },
            Self::UUID => Some(make_type("uuid::Uuid").ok()?),
            Self::String(_) => Some(make_type("String").ok()?),
            Self::Bytes(len) => {
                let lit = syn::LitInt::new(&len.to_string(), Span::call_site());
                Some(parse_quote! { [u8; #lit] })
            },
            Self::Integer { .. } => Some(make_type("u32").ok()?),
            Self::Number { .. } => Some(make_type("f32").ok()?),
            Self::Timestamp => Some(parse_quote! { chrono::DateTime<chrono::Utc> }),
            Self::None => None,
        }
    }
}
