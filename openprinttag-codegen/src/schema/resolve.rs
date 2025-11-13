use crate::schema::{EnumSchema, StructSchema};
use std::sync::Arc;

pub trait Resolve {
    fn resolve_enum(&self, schema_name: &str) -> crate::Result<Arc<EnumSchema>>;
    fn resolve_struct(&self, schema_name: &str) -> crate::Result<Arc<StructSchema>>;
}
