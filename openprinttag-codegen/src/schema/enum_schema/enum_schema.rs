use crate::{
    de::DeserializeWithContext,
    schema::{
        context::LocalContext,
        gen::{
            util, GetAttributes, GetDoc, GetDocAsAttributes, GetIdent, GetPubVisibility,
            GetSchemaName, GetSize, GetType, GetVariant, GetVariants, GetVisibility, ToItem,
        },
        EnumVariantSchemas,
    },
};
use serde::{de::Error, Deserializer};
use syn::{parse_quote, Attribute, Item, Type, Variant};

#[derive(Debug, Clone, derive_new::new)]
pub struct EnumSchema {
    pub schema_name: String,

    pub description: Option<String>,
    pub variants: EnumVariantSchemas,
}

impl GetSchemaName for EnumSchema {
    #[inline]
    fn get_schema_name(&self) -> String {
        self.schema_name.clone()
    }
}

impl GetSize for EnumSchema {
    #[inline(always)]
    fn get_size(&self) -> usize {
        self.variants.len()
    }
}

impl GetPubVisibility for EnumSchema {}

impl GetType for EnumSchema {
    #[inline]
    fn get_type(&self) -> Option<Type> {
        Some(util::make_type(self.get_name()?).ok()?)
    }
}

impl GetDoc for EnumSchema {
    #[inline]
    fn get_doc(&self) -> Option<String> {
        self.description.clone()
    }
}

impl GetAttributes for EnumSchema {
    fn get_attributes(&self) -> Vec<Attribute> {
        let mut attrs = vec![parse_quote!(#[repr(u32)])];

        if let Some(doc) = self.get_doc_attribute() {
            attrs.push(doc)
        }

        attrs
    }
}

impl GetVariants for EnumSchema {
    fn get_variants(&self) -> Vec<Variant> {
        self.variants.iter().filter_map(|v| v.get_variant()).collect()
    }
}

impl ToItem for EnumSchema {
    #[inline]
    fn to_item(&self) -> Option<Item> {
        let attrs = self.get_attributes();
        let vis = self.get_visibility();
        let ident = self.get_ident()?;
        let variants = self.get_variants_punctuated();

        Some(parse_quote! {
            #(#attrs)*
            #vis enum #ident {
                #variants
            }
        })
    }
}

impl<'a> DeserializeWithContext<'a> for EnumSchema {
    fn deserialize_with_context<'de, D>(
        _: D,
        local_context: &'a LocalContext<'a>,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut schema = EnumSchema {
            schema_name: local_context.schema_name.to_string(),
            description: local_context.description.clone(),
            variants: vec![],
        };

        let mut variant_context = LocalContext::new_from(local_context, local_context.schema_name);
        variant_context.parent_name = schema.get_name();
        schema.variants = local_context
            .seed_local::<EnumVariantSchemas>()
            .map_err(|e| D::Error::custom(e.to_string()))?;

        Ok(schema)
    }
}
