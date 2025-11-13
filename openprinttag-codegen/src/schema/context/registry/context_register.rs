use std::sync::Arc;
use crate::de::WithContext;
use crate::loader::{Loader, LoaderSeedExt};
use crate::schema::{EnumSchema, Schema, StructSchema};
use crate::schema::context::LocalContext;
use crate::schema::context::registry::Register;

pub trait ContextRegister: Register + Loader {
    fn load_and_insert_enum_from(&self, local: &LocalContext) -> crate::Result<String> {
        self.get_or_insert_with(local.schema_name, || {
            let seed = WithContext::<EnumSchema>::new(local);
            let enum_schema = self.seed(local.schema_name, seed).map_err(crate::Error::from)?;
            Ok(Schema::Enum(Arc::new(enum_schema)))
        })?;
        Ok(local.schema_name.to_string())
    }

    fn load_and_insert_struct_from(&self, local: &LocalContext) -> crate::Result<String> {
        self.get_or_insert_with(local.schema_name, || {
            let seed = WithContext::<StructSchema>::new(local);
            let struct_schema = self.seed(local.schema_name, seed).map_err(crate::Error::from)?;
            Ok(Schema::Struct(Arc::new(struct_schema)))
        })?;
        Ok(local.schema_name.to_string())
    }
}
