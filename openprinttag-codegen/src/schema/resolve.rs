use crate::schema::{DataEnum, DataStruct};
use std::sync::Arc;

pub trait Resolve {
    fn resolve_enum(&self, schema_name: &str) -> crate::Result<Arc<DataEnum>>;
    fn resolve_struct(&self, schema_name: &str) -> crate::Result<Arc<DataStruct>>;
}
