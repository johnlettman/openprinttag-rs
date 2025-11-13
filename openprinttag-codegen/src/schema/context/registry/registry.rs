use std::collections::hash_map::{Iter, Keys, Values};
use std::collections::HashMap;
use parking_lot::{RwLock, RwLockReadGuard};
use syn::Item;
use crate::schema::{Schema};
use crate::schema::context::registry::{Register};
use crate::schema::gen::{AsItem, AsItems};

#[derive(Debug)]
pub struct Registry {
    map: RwLock<HashMap<String, Schema>>,
}

impl Registry {
    pub fn new() -> Self {
        Self { map: RwLock::new(HashMap::new()) }
    }
}

impl AsItems for Registry {
    #[inline]
    fn makes_items(&self) -> bool {
        self.values().any(|v| v.is_item())
    }

    #[inline]
    fn as_items(&self) -> Vec<Item> {
        self.values().flat_map(|v| v.as_items()).collect()
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
