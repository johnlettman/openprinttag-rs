use crate::{
    emit::EmitItems,
    schema::{context::registry::Register, Schema},
};
use crate::lock::RwLock;
use std::{collections::HashMap, fmt};
use syn::Item;


pub type SchemaMap = RwLock<HashMap<String, Schema>>;

/// A thread-safe registry of [`Schema`] objects.
///
/// The [`Registry`] maps schema names to their corresponding [`Schema`] values.
/// Internally, the storage is protected by an [`RwLock`], which resolves to.
///
/// The registry supports:
/// - Schema lookup ([`get`], [`contains`])
/// - Schema insertion ([`insert`])
/// - Lazy initialization ([`get_or_insert_with`])
/// - Iteration over keys, values, or entries
/// - Emitting Rust code via [`EmitItems`]
///
/// [`get`]: Registry::get
/// [`contains`]: Registry::contains
/// [`insert`]: Registry::insert
/// [`get_or_insert_with`]: Registry::get_or_insert_with
pub struct Registry {
    map: SchemaMap,
}

impl Registry {
    pub fn new() -> Self {
        Self { map: RwLock::new(HashMap::new()) }
    }
}

impl fmt::Debug for Registry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let map = self.map.read();
        let mut debug = f.debug_struct("Registry");

        for (key, schema) in map.iter() {
            debug.field(key, schema);
        }

        debug.finish()
    }
}

impl EmitItems for Registry {
    /// Collects all code-generation items emitted by contained schemas.
    ///
    /// This flattens the output of [`Schema::emit_core_items`] for each schema
    /// stored in the registry. The registry therefore acts as the root assembly
    /// point for code generation.
    fn emit_core_items(&self) -> Vec<Item> {
        self.values().flat_map(|v| v.emit_core_items()).collect()
    }
}

impl Register for Registry {
    fn get(&self, schema_name: &str) -> Option<Schema> {
        self.map.read().get(schema_name).cloned()
    }

    fn contains(&self, schema_name: &str) -> bool {
        self.map.read().contains_key(schema_name)
    }

    fn insert(&self, schema_name: impl Into<String>, schema: Schema) -> Option<Schema> {
        self.map.write().insert(schema_name.into(), schema)
    }

    fn iter(&self) -> std::vec::IntoIter<(String, Schema)> {
        let guard = self.map.read();
        guard.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>().into_iter()
    }

    fn keys(&self) -> std::vec::IntoIter<String> {
        self.map.read().keys().map(|k| k.clone()).collect::<Vec<_>>().into_iter()
    }

    fn values(&self) -> std::vec::IntoIter<Schema> {
        self.map.read().values().map(|v| v.clone()).collect::<Vec<_>>().into_iter()
    }

    fn get_or_insert_with<F>(&self, key: &str, f: F) -> crate::Result<Schema>
    where
        F: FnOnce() -> crate::Result<Schema>,
    {
        let mut guard = self.map.write();
        if !guard.contains_key(key) {
            guard.insert(key.to_string(), f()?);
        }
        guard.get(key).cloned().ok_or_else(|| crate::Error::NoSchema(key.to_string()))
    }
}

impl<'a> IntoIterator for &Registry {
    type Item = (String, Schema);
    type IntoIter = std::vec::IntoIter<(String, Schema)>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
