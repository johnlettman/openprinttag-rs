use crate::{
    de::DeserializeWithContext,
    schema::{
        context::LocalContext,
        gen::{
            util, GetAttributes, GetDoc, GetDocAsAttributes, GetField, GetFields, GetIdent,
            GetPubVisibility, GetSchemaName, GetType, GetVisibility, ToItem,
        },
        StructFieldSchemas,
    },
};
use serde::{de::Error, Deserializer};
use syn::{parse_quote, Field, Fields, FieldsNamed, Item, Type};

#[derive(Debug, Clone)]
pub struct StructSchema {
    pub schema_name: String,

    pub description: Option<String>,
    pub fields: StructFieldSchemas,
}

impl GetSchemaName for StructSchema {
    fn get_schema_name(&self) -> String {
        self.schema_name.clone()
    }
}

impl GetPubVisibility for StructSchema {}

impl GetDoc for StructSchema {
    #[inline(always)]
    fn get_doc(&self) -> Option<String> {
        self.description.clone()
    }
}

impl GetDocAsAttributes for StructSchema {}

impl GetType for StructSchema {
    #[inline]
    fn get_core_type(&self) -> Option<Type> {
        Some(util::make_type(self.get_name()?).ok()?)
    }
}

impl GetFields for StructSchema {
    fn get_core_fields(&self) -> Vec<Field> {
        self.fields.iter().filter_map(|f| f.get_core_field()).collect()
    }
}

impl ToItem for StructSchema {
    #[inline]
    fn to_core_item(&self) -> Option<Item> {
        let ident = self.get_core_ident()?;
        let attrs = self.get_core_attributes();
        let vis = self.get_visibility();
        let fields = self.get_fields_punctuated();

        Some(parse_quote! {
            #(#attrs)*
            #vis struct #ident {
                #fields
            }
        })
    }
}

impl<'a> DeserializeWithContext<'a> for StructSchema {
    fn deserialize_with_context<'de, D>(
        _: D,
        local_context: &'a LocalContext<'a>,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut schema = StructSchema {
            schema_name: local_context.schema_name.to_string(),
            description: local_context.description.clone(),
            fields: vec![],
        };

        let mut variant_context = LocalContext::new_from(local_context, local_context.schema_name);
        variant_context.parent_name = schema.get_name();
        schema.fields = local_context
            .seed_local::<StructFieldSchemas>()
            .map_err(|e| D::Error::custom(e.to_string()))?;

        Ok(schema)
    }
}
