use crate::schema::{builtin, fmt::term, name};
use proc_macro2::Span;
use syn::{parse_quote, Type};
use crate::emit::EmitType;
use crate::emit::util::emit_type;

#[derive(Debug, Clone)]
pub enum DataTy {
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

impl std::fmt::Display for DataTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Struct(s) => write!(f, "struct({})", term::schema_link(s)),
            Self::Enum(e) => write!(f, "enum({})", term::schema_link(e)),
            Self::EnumArray(e, size) => {
                write!(f, "enum_array({}, max_length: {})", term::schema_link(e), size)
            },
            Self::UUID => write!(f, "uuid"),
            Self::String(len) => write!(f, "string(max_length: {})", len),
            Self::Bytes(len) => write!(f, "bytes(max_length: {})", len),
            Self::Integer { unit, example } => {
                write!(f, "int")?;
                let mut params = Vec::new();

                if let Some(unit) = unit {
                    params.push(format!("unit: \"{}\"", unit));
                }

                if let Some(example) = example {
                    params.push(format!("example: {}", example));
                }

                if !params.is_empty() {
                    write!(f, "({})", params.join(", "))?;
                }

                Ok(())
            },
            Self::Number { unit, example } => {
                write!(f, "number")?;
                let mut params = Vec::new();

                if let Some(unit) = unit {
                    params.push(format!("unit: \"{}\"", unit));
                }

                if let Some(example) = example {
                    params.push(format!("example: {}", example));
                }

                if !params.is_empty() {
                    write!(f, "({})", params.join(", "))?;
                }

                Ok(())
            },
            Self::Timestamp => write!(f, "timestamp"),
            Self::None => write!(f, "none"),
        }
    }
}

impl EmitType for DataTy {
    fn emit_core_type(&self) -> Option<Type> {
        match self {
            Self::Struct(s) | Self::Enum(s) => Some(emit_type(name::to_camel(s))),
            Self::EnumArray(e, size) => {
                let lit = syn::LitInt::new(&size.to_string(), Span::call_site());
                let ident = name::to_camel_ident(e);
                Some(parse_quote!(EnumArray<#ident, #lit>))
            },
            Self::UUID => builtin::Uuid.emit_core_type(),
            Self::String(_) => Some(emit_type("String")),
            Self::Bytes(len) => {
                let lit = syn::LitInt::new(&len.to_string(), Span::call_site());
                Some(parse_quote! { [u8; #lit] })
            },
            Self::Integer { .. } => Some(emit_type("u32")),
            Self::Number { .. } => Some(emit_type("f32")),
            Self::Timestamp => builtin::Timestamp.emit_core_type(),
            Self::None => None,
        }
    }
}
