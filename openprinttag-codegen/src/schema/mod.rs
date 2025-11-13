mod builtin;
mod config;
pub mod context;
mod enum_schema;
pub mod gen;
mod resolve;
mod struct_schema;
mod type_schema;
pub mod name;

pub use builtin::*;
use crate::{
    schema::gen::AsItem,
};
pub use enum_schema::*;
pub use resolve::*;
use std::sync::Arc;
pub use struct_schema::*;
use syn::Item;
pub use type_schema::*;
pub use config::*;

#[derive(Debug, Clone)]
pub enum Schema {
    Enum(Arc<EnumSchema>),
    Struct(Arc<StructSchema>),
}

impl AsItem for Schema {
    #[inline]
    fn is_item(&self) -> bool {
        match self {
            Self::Enum(e) => e.is_item(),
            Self::Struct(s) => s.is_item()
        }
    }

    #[inline]
    fn as_item(&self) -> Option<Item> {
        match self {
            Self::Enum(e) => e.as_item(),
            Self::Struct(s) => s.as_item()
        }
    }
}
