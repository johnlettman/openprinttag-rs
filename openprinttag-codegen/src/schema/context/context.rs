use crate::{
    loader::{Loader, LoaderResult},
    schema::{Schema,
    },
};
use serde_norway::Value;
use std::io::Read;
use crate::schema::context::registry::{ContextRegister, Register, Registry};

/// A global environment containing:
///
/// - a YAML loader ([`DirLoader`], [`CrateDirLoader`], [`URLLoader`], etc.)
/// - an in-memory registry caching loaded schemas
///
/// This is the primary state container for the codegen system.
///
/// [`DirLoader`]: crate::loader::DirLoader
/// [`CrateDirLoader`]: crate::loader::CrateDirLoader
/// [`URLLoader`]: crate::loader::URLLoader
#[derive(Debug)]
pub struct Context {
    pub loader: Box<dyn Loader>,
    pub registry: Registry,
}

impl Context {
    /// Construct a new context from any loader.
    #[inline(always)]
    pub fn new(loader: Box<dyn Loader>) -> Self {
        Self { loader, registry: Registry::new() }
    }
}

impl ContextRegister for Context {}

impl Loader for Context {
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

impl Register for Context {
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
