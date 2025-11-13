use std::sync::Arc;
use crate::schema::{EnumSchema, Schema, StructSchema};

pub trait Register {
    fn get(&self, schema_name: &str) -> Option<Schema>;
    fn contains(&self, schema_name: &str) -> bool;
    fn insert(&self, schema_name: impl Into<String>, schema: Schema) -> Option<Schema>;
    fn iter(&self) -> std::vec::IntoIter<(String, Schema)>;
    fn keys(&self) -> std::vec::IntoIter<String>;
    fn values(&self) -> std::vec::IntoIter<Schema>;

    fn insert_enum(
        &self,
        schema_name: impl Into<String>,
        enum_schema: Arc<EnumSchema>,
    ) -> Option<Schema> {
        self.insert(schema_name, Schema::Enum(enum_schema))
    }

    fn insert_struct(
        &self,
        schema_name: impl Into<String>,
        struct_schema: Arc<StructSchema>,
    ) -> Option<Schema> {
        self.insert(schema_name, Schema::Struct(struct_schema))
    }

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
