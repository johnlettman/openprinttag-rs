use crate::{
    de::WithContext,
    loader::{Loader, LoaderSeedExt},
    schema::{
        context::{registry::Register, LocalContext},
        EnumSchema, Schema, StructSchema,
    },
};
use std::sync::Arc;

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
