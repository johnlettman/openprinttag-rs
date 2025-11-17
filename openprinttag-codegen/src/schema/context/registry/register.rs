use crate::schema::{DataEnum, DataStruct, Schema};
use std::sync::Arc;

pub trait Register {
    /// Returns a schema by name.
    fn get(&self, schema_name: &str) -> Option<Schema>;
    fn contains(&self, schema_name: &str) -> bool;
    fn insert(&self, schema_name: impl Into<String>, schema: Schema) -> Option<Schema>;

    /// Returns an iterator over `(name, schema)` pairs.
    ///
    /// This clones the data out of the lock so that iteration does not
    /// hold the lock during the caller's loop.
    fn iter(&self) -> std::vec::IntoIter<(String, Schema)>;

    /// Returns an iterator over schema names.
    fn keys(&self) -> std::vec::IntoIter<String>;

    /// Returns an iterator over schema values.
    fn values(&self) -> std::vec::IntoIter<Schema>;

    fn insert_enum(
        &self,
        schema_name: impl Into<String>,
        enum_schema: Arc<DataEnum>,
    ) -> Option<Schema> {
        self.insert(schema_name, Schema::Enum(enum_schema))
    }

    fn insert_struct(
        &self,
        schema_name: impl Into<String>,
        struct_schema: Arc<DataStruct>,
    ) -> Option<Schema> {
        self.insert(schema_name, Schema::Struct(struct_schema))
    }

    /// Retrieves a schema by name, inserting it lazily if not present.
    fn get_or_insert_with<F>(&self, key: &str, f: F) -> crate::Result<Schema>
    where
        F: FnOnce() -> crate::Result<Schema>,
    {
        if self.contains(key) {
            return self.get(key).ok_or_else(|| crate::Error::NoSchema(key.to_string()));
        }
        let schema = f()?;
        self.insert(key.to_string(), schema);
        self.get(key).ok_or_else(|| crate::Error::NoSchema(key.to_string()))
    }
}
