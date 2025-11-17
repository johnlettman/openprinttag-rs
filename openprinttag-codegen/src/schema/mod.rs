mod builtin;
mod data_print_tag;
pub mod context;
mod data_enum;
mod data_struct;
pub mod fmt;
pub mod name;
mod resolve;
mod data_ty;

pub use builtin::*;
pub use data_print_tag::*;
pub use data_enum::*;
pub use data_struct::*;
pub use resolve::*;
use std::sync::Arc;
use syn::{parse_quote, Attribute, Item};
pub use data_ty::*;
use crate::emit::EmitItems;

pub trait SchemaName {
    fn get_schema_name(&self) -> String;
}

pub trait HasDocs {
    fn docs(&self) -> Option<String>;

    fn doc_attribute(&self) -> Option<Attribute> {
        let doc = self.docs()?;
        Some(parse_quote!(#[doc = #doc]))
    }
}

pub trait DataSize {
    fn size(&self) -> usize;
}


#[derive(Debug, Clone)]
pub enum Schema {
    Enum(Arc<DataEnum>),
    Struct(Arc<DataStruct>),
}

pub trait GetSchema {
    fn get_schema(&self) -> Schema;
}

pub trait GetSchemas {
    fn get_schemas(&self) -> Vec<Schema>;

    fn find_schema_by_name(&self, name: &str) -> Option<Schema> {
        let name = name::normalize(name);
        self.get_schemas().into_iter().find(|s| s.get_schema_name() == name)
    }
}

impl SchemaName for Schema {
    #[inline]
    fn get_schema_name(&self) -> String {
        match self {
            Self::Enum(e) => e.get_schema_name(),
            Self::Struct(s) => s.get_schema_name(),
        }
    }
}

impl EmitItems for Schema {
    #[inline]
    fn emit_core_items(&self) -> Vec<Item> {
        match self {
            Self::Enum(e) => e.emit_core_items(),
            Self::Struct(s) => s.emit_core_items(),
        }
    }
}
