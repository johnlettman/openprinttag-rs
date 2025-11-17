mod data_struct_field;
mod data_struct_fields;
mod data_struct_cbor;

pub use data_struct_field::*;
pub use data_struct_fields::*;

use crate::{
    de::DeserializeWithContext,
    emit::{
        util, EmitAttributes, EmitField, EmitFields, EmitIdent, EmitPubVisibility,
        EmitType, EmitVisibility, EmitItem,
    },
    schema::{
        context::LocalContext,
        HasDocs,
        SchemaName
    },
};
use serde::{de::Error, Deserializer};
use std::fmt;
use syn::{parse_quote, Attribute, Field, Item, Type};
use crate::emit::EmitItems;
use crate::schema::name;

#[derive(Clone)]
pub struct DataStruct {
    pub schema_name: String,

    pub description: Option<String>,
    pub fields: DataStructFields,
}

impl fmt::Debug for DataStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DataStruct")
            .field("schema_name", &self.schema_name)
            .field("description", &self.description)
            .field("fields", &format_args!("DataStructFields(len={})", self.fields.len()))
            .finish()
    }
}

impl SchemaName for DataStruct {
    fn get_schema_name(&self) -> String {
        self.schema_name.clone()
    }
}

impl EmitPubVisibility for DataStruct {}

impl HasDocs for DataStruct {
    #[inline(always)]
    fn docs(&self) -> Option<String> {
        self.description.clone()
    }
}

impl EmitAttributes for DataStruct {
    fn emit_core_attributes(&self) -> Vec<Attribute> {
        let mut attrs = vec![parse_quote!(#[derive(Debug, Clone)])];

        if let Some(doc_attr) = self.doc_attribute() {
            attrs.push(doc_attr);
        }

        attrs
    }
}

impl EmitType for DataStruct {
    #[inline]
    fn emit_core_type(&self) -> Option<Type> {
        Some(util::emit_type(self.get_name()?))
    }
}

impl EmitFields for DataStruct {
    fn emit_core_fields(&self) -> Vec<Field> {
        self.fields.iter().filter_map(|f| f.emit_core_field()).collect()
    }
}

impl EmitItems for DataStruct {
    #[inline]
    fn emit_core_items(&self) -> Vec<Item> {
        if let Some(ident) = self.rs_core_ident() {
            let attrs = self.emit_core_attributes();
            let vis = self.emit_visibility();
            let fields = self.emit_core_fields();

            let mut items: Vec<Item> = vec![
                parse_quote! {
                    #(#attrs)*
                    #vis struct #ident {
                        #(#fields),*
                    }
                }
            ];

            items.extend(self.core_cbor_impls());

            items
        } else {
            Vec::new()
        }
    }
}

impl<'a> DeserializeWithContext<'a> for DataStruct {
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "debug", fields(%schema = local_context.schema_name
    ), skip(local_context)))]
    fn deserialize_with_context<'de, D>(
        _: D,
        local_context: &'a LocalContext<'a>,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut schema = DataStruct {
            schema_name: local_context.schema_name.to_string(),
            description: local_context.description.clone(),
            fields: vec![],
        };

        let mut variant_context = LocalContext::new_from(local_context, local_context.schema_name);
        variant_context.parent_name = schema.get_name();
        schema.fields = local_context
            .seed_local::<DataStructFields>()
            .map_err(|e| D::Error::custom(e.to_string()))?;

        Ok(schema)
    }
}
