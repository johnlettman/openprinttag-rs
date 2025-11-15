use crate::{
    de::{DeserializeWithContext, WithContext},
    loader::{Loader, LoaderResult, LoaderSeedExt},
    schema::{
        context::{
            registry::{ContextRegister, Register},
            Context, SharedContext,
        },
        EnumSchema, Schema, StructSchema,
    },
};
use serde_norway::Value;
use std::{collections::HashMap, io::Read, sync::Arc};

/// A transient, contextual frame used during deserialization.
///
/// It carries:
/// - the current schema name being resolved
/// - inherited parent name
/// - optional description used for documentation
/// - optional name remapping
///
/// A `LocalContext` is what makes recursive schema deserialization possible,
/// while keeping the full shared environment accessible through `context`.
#[derive(Debug, Clone)]
pub struct LocalContext<'a> {
    pub context: SharedContext,
    pub schema_name: &'a str,
    pub description: Option<String>,
    pub parent_name: Option<String>,
    pub name_map: Option<Arc<HashMap<String, String>>>,
}

impl<'a> LocalContext<'a> {
    /// Create a new local context frame for a schema.
    #[inline(always)]
    pub fn new(context: SharedContext, schema_name: &'a str) -> Self {
        Self { context, schema_name, description: None, parent_name: None, name_map: None }
    }

    /// Create a new context for a nested schema reference.
    ///
    /// Inherits:
    /// - the same shared context
    /// - parent name logic
    /// - any name remapping
    #[inline(always)]
    pub fn new_from(local: &'a LocalContext<'a>, schema_name: &'a str) -> Self {
        Self {
            context: local.context.clone(),
            schema_name,
            description: local.description.clone(),
            parent_name: local.parent_name.clone().or_else(|| Some(local.schema_name.to_string())),
            name_map: local.name_map.clone(),
        }
    }

    #[inline(always)]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[inline(always)]
    pub fn with_parent_name(mut self, parent: impl Into<String>) -> Self {
        self.parent_name = Some(parent.into());
        self
    }

    #[inline(always)]
    pub fn with_name_map(mut self, map: Option<Arc<HashMap<String, String>>>) -> Self {
        self.name_map = map;
        self
    }

    /// Deserialize the schema referenced by this context and return the loaded
    /// value.
    #[inline]
    pub fn seed_local<T>(&self) -> LoaderResult<T>
    where
        T: for<'de> DeserializeWithContext<'de> + ToOwned<Owned = T>,
    {
        let seed = WithContext::<T>::new(self);
        self.seed(self.schema_name, seed)
    }

    /// Insert a schema into the underlying registry using the current name.
    #[inline(always)]
    pub fn insert_local(&self, schema: Schema) -> Option<Schema> {
        self.insert(self.schema_name, schema)
    }

    /// Convenience for inserting enum schemas.c
    #[inline(always)]
    pub fn insert_enum_local(&self, enum_schema: Arc<EnumSchema>) -> Option<Schema> {
        self.insert_enum(self.schema_name, enum_schema)
    }

    /// Convenience for inserting struct schemas.
    #[inline(always)]
    pub fn insert_struct_local(&self, struct_schema: Arc<StructSchema>) -> Option<Schema> {
        self.insert_struct(self.schema_name, struct_schema)
    }

    /// Load and insert an enum schema into the registry.
    #[inline(always)]
    pub fn load_and_insert_enum(&self) -> crate::Result<String> {
        self.context.load_and_insert_enum_from(self)
    }

    /// Load and insert a struct schema into the registry.
    #[inline(always)]
    pub fn load_and_insert_struct(&self) -> crate::Result<String> {
        self.context.load_and_insert_struct_from(self)
    }
}

impl<'a> std::ops::Deref for LocalContext<'a> {
    type Target = Context;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

/// Allow calling context-registration APIs directly on [`LocalContext`].
impl<'a> ContextRegister for LocalContext<'a> {}

/// Allow [`LocalContext`] to function as a loader proxy.
impl<'a> Loader for LocalContext<'a> {
    #[inline(always)]
    fn load_string(&self, schema_name: &str) -> LoaderResult<String> {
        self.context.load_string(schema_name)
    }

    #[inline(always)]
    fn read(&self, schema_name: &str) -> LoaderResult<Box<dyn Read>> {
        self.context.read(schema_name)
    }

    #[inline(always)]
    fn load_value(&self, schema_name: &str) -> LoaderResult<Value> {
        self.context.load_value(schema_name)
    }
}

/// Allow [`LocalContext`] to function as a registry proxy.
impl<'a> Register for LocalContext<'a> {
    #[inline(always)]
    fn get(&self, schema_name: &str) -> Option<Schema> {
        self.context.get(schema_name)
    }

    #[inline(always)]
    fn contains(&self, schema_name: &str) -> bool {
        self.context.contains(schema_name)
    }

    #[inline(always)]
    fn insert(&self, schema_name: impl Into<String>, schema: Schema) -> Option<Schema> {
        self.context.insert(schema_name, schema)
    }

    #[inline(always)]
    fn iter(&self) -> std::vec::IntoIter<(String, Schema)> {
        self.context.iter()
    }

    #[inline(always)]
    fn keys(&self) -> std::vec::IntoIter<String> {
        self.context.keys()
    }

    #[inline(always)]
    fn values(&self) -> std::vec::IntoIter<Schema> {
        self.context.values()
    }
}
