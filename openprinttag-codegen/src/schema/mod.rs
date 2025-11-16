mod builtin;
mod config;
pub mod context;
mod enum_schema;
pub mod gen;
pub mod name;
mod resolve;
mod struct_schema;
mod type_schema;

use crate::schema::gen::{GetSchemaName, ToItem, ToItems};
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

pub trait GetSchema {
    fn get_schema(&self) -> Schema;
}

pub trait GetSchemas {
    fn get_schemas(&self) -> Vec<Schema>;
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

impl ToItems for Schema {
    #[inline]
    fn to_core_items(&self) -> Vec<Item> {
        match self {
            Self::Enum(e) => e.to_core_items(),
            Self::Struct(s) => s.to_core_items(),
        }
    }
}
