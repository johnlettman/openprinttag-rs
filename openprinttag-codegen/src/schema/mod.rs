mod builtin;
mod config;
pub mod context;
mod enum_schema;
pub mod gen;
pub mod name;
mod resolve;
mod struct_schema;
mod type_schema;

use crate::schema::gen::{GetSchemaName, ToItem};
pub use builtin::*;
pub use config::*;
pub use enum_schema::*;
pub use resolve::*;
use std::sync::Arc;
pub use struct_schema::*;
use syn::Item;
pub use type_schema::*;

#[derive(Debug, Clone)]
pub enum Schema {
    Enum(Arc<EnumSchema>),
    Struct(Arc<StructSchema>),
}

impl GetSchemaName for Schema {
    #[inline]
    fn get_schema_name(&self) -> String {
        match self {
            Self::Enum(e) => e.get_schema_name(),
            Self::Struct(s) => s.get_schema_name(),
        }
    }
}

impl ToItem for Schema {
    #[inline]
    fn to_core_item(&self) -> Option<Item> {
        match self {
            Self::Enum(e) => e.to_core_item(),
            Self::Struct(s) => s.to_core_item(),
        }
    }
}
