use crate::{
    loader::{Loader, LoaderResult},
    schema::{
        context::{
            registry::{ContextRegister, Register},
            Context,
        },
        Schema,
    },
};
use serde_norway::Value;
use std::{io::Read, sync::Arc};

/// Shared ownable context.
/// This is the type most callers should pass around.
///
/// A `SharedContext` internally is [`Arc<Context>`][Context].
pub type SharedContext = Arc<Context>;

/// Allow calling context-registration APIs directly on [`SharedContext`].
impl ContextRegister for SharedContext {}

/// Allow [`SharedContext`] to function as a loader proxy.
impl Loader for SharedContext {
    #[inline(always)]
    fn load_string(&self, schema_name: &str) -> LoaderResult<String> {
        self.loader.load_string(schema_name)
    }

    #[inline(always)]
    fn read(&self, schema_name: &str) -> LoaderResult<Box<dyn Read>> {
        self.loader.read(schema_name)
    }

    #[inline(always)]
    fn load_value(&self, schema_name: &str) -> LoaderResult<Value> {
        self.loader.load_value(schema_name)
    }
}

/// Allow [`SharedContext`] to function as a registry proxy.
impl Register for SharedContext {
    #[inline(always)]
    fn get(&self, schema_name: &str) -> Option<Schema> {
        self.registry.get(schema_name)
    }

    #[inline(always)]
    fn contains(&self, schema_name: &str) -> bool {
        self.registry.contains(schema_name)
    }

    #[inline(always)]
    fn insert(&self, schema_name: impl Into<String>, schema: Schema) -> Option<Schema> {
        self.registry.insert(schema_name, schema)
    }

    #[inline(always)]
    fn iter(&self) -> std::vec::IntoIter<(String, Schema)> {
        self.registry.iter()
    }

    #[inline(always)]
    fn keys(&self) -> std::vec::IntoIter<String> {
        self.registry.keys()
    }

    #[inline(always)]
    fn values(&self) -> std::vec::IntoIter<Schema> {
        self.registry.values()
    }
}
