use crate::{
    de::{DeserializeWithContext, WithContext},
    schema::{
        context::{registry::Register, LocalContext},
        gen::{
            GetAttributes, GetDoc, GetField, GetIdent, GetPubVisibility, GetSchemaName, GetSize,
            GetType, GetVisibility,
        },
        EnumSchema, EnumVariantSchema, Schema, TypeSchema,
    },
    Required,
};
use proc_macro2::Span;
use serde::{Deserialize, Deserializer};
use std::{collections::HashMap, fmt, sync::Arc};
use syn::{parse_quote, Attribute, Field, Type};

pub type StructFieldSchemas = Vec<StructFieldSchema>;

#[derive(Debug, Clone)]
pub struct StructFieldSchema {
    parent_name: Option<String>,

    key: Option<u32>,
    name: Option<String>,
    description: Option<String>,

    required: Option<Required>,
    deprecated: bool,

    field_type: TypeSchema,
}

impl GetIdent for StructFieldSchema {
    #[inline]
    fn get_name(&self) -> Option<String> {
        self.name.clone()
    }
}

impl GetPubVisibility for StructFieldSchema {}

impl GetType for StructFieldSchema {
    #[inline]
    fn get_core_type(&self) -> Option<Type> {
        self.field_type.get_core_type()
    }
}

impl GetDoc for StructFieldSchema {
    fn get_doc(&self) -> Option<String> {
        let mut doc = String::new();

        if let Some(description) = &self.description {
            doc.push_str(description);
        }

        if self.deprecated {
            doc.push_str("\n**Deprecated.**");
        }

        (!doc.is_empty()).then_some(doc)
    }
}

impl GetAttributes for StructFieldSchema {
    //noinspection DuplicatedCode
    fn get_core_attributes(&self) -> Vec<Attribute> {
        let mut attrs = Vec::new();

        if let Some(doc) = self.get_doc_attribute() {
            attrs.push(doc);
        }

        if self.deprecated {
            attrs.push(parse_quote!(#[deprecated]));
        }

        attrs
    }
}

impl GetField for StructFieldSchema {
    fn get_core_field(&self) -> Option<Field> {
        let attrs = self.get_core_attributes();
        let vis = self.get_visibility();
        let ident = self.get_core_ident();
        let ty = self.get_core_type()?;

        Some(parse_quote! {
            #(#attrs)*
            #vis #ident: #ty
        })
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct RawStructFieldSchema {
    pub(crate) key: Option<u32>,

    #[serde(default)]
    pub(crate) name: Option<String>,

    #[serde(default)]
    pub(crate) description: Option<serde_norway::Value>,

    #[serde(default)]
    pub(crate) required: Option<Required>,

    #[serde(default)]
    pub(crate) deprecated: bool,

    #[serde(default)]
    pub(crate) r#type: Option<String>,

    #[serde(default)]
    pub(crate) max_length: Option<u64>,

    #[serde(default)]
    pub(crate) unit: Option<String>,

    #[serde(default)]
    pub(crate) items_file: Option<String>,

    #[serde(default)]
    pub(crate) name_field: Option<String>,

    // https://github.com/prusa3d/OpenPrintTag/issues/78
    // https://github.com/prusa3d/OpenPrintTag/pull/81
    #[serde(default)]
    pub(crate) display_name_field: Option<String>,

    #[serde(default)]
    pub(crate) example: Option<serde_norway::Value>,
}

impl<'a> DeserializeWithContext<'a> for StructFieldSchema {
    fn deserialize_with_context<'de, D>(
        deserializer: D,
        local_context: &'a LocalContext<'a>,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::de;
        use serde::{de::Error, Deserialize};
        use serde_norway::Value;

        let mut raw_map: Value = Value::deserialize(deserializer)?;
        de::remap_names(&mut raw_map, local_context.name_map.clone());

        let RawStructFieldSchema {
            key,
            name,
            description: raw_description,
            required,
            deprecated,
            r#type,
            max_length,
            unit,
            items_file,
            name_field,
            display_name_field,
            example,
        } = RawStructFieldSchema::deserialize(raw_map).map_err(|e| Error::custom(e.to_string()))?;

        // description
        let description = match raw_description {
            Some(val) => de::description(val.clone()).map_err(D::Error::custom)?,
            None => None,
        };

        fn seed_enum<'de, D>(
            local_context: &LocalContext,
            items_file: Option<String>,
            name_field: Option<String>,
            display_name_field: Option<String>,
        ) -> Result<EnumSchema, D::Error>
        where
            D: Deserializer<'de>,
        {
            let schema_name =
                items_file.as_ref().ok_or_else(|| D::Error::missing_field("items_file"))?.as_str();

            let mut name_map = HashMap::new();

            if let Some(n) = name_field {
                name_map.insert(n.clone(), "name".to_string());
            }

            if let Some(d) = display_name_field {
                name_map.insert(d.clone(), "display_name".to_string());
            }

            let name_map = (!name_map.is_empty()).then_some(Arc::new(name_map));
            let mut enum_context = LocalContext::new_from(local_context, schema_name);
            enum_context.name_map = name_map;
            enum_context.seed_local().map_err(|e| D::Error::custom(e.to_string()))
        }

        let field_type = match r#type.as_deref() {
            Some("uuid") => Ok(TypeSchema::UUID),
            Some("string") => Ok(TypeSchema::String(max_length.unwrap_or(255) as usize)),
            Some("bytes") => Ok(TypeSchema::Bytes(max_length.unwrap_or(255) as usize)),
            Some("int" | "integer") => {
                let example = example.as_ref().and_then(|v| v.as_u64()).map(|v| v as u32);
                Ok(TypeSchema::Integer { unit: unit.clone(), example })
            },

            Some("number") => {
                let example = example.as_ref().and_then(|v| v.as_f64()).map(|v| v as f32);
                Ok(TypeSchema::Number { unit: unit.clone(), example })
            },

            Some("timestamp") => Ok(TypeSchema::Timestamp),
            Some("enum") => {
                let e = seed_enum::<D>(local_context, items_file, name_field, display_name_field)
                    .map_err(|e| D::Error::custom(e))?;

                let schema_name = e.get_schema_name();
                let _ = local_context
                    .insert(schema_name.clone(), Schema::Enum(Arc::new(e)))
                    .ok_or(D::Error::custom("could not insert schema"))?;
                Ok(TypeSchema::Enum(schema_name))
            },
            Some("enum_array") => {
                let e = seed_enum::<D>(local_context, items_file, name_field, display_name_field)?;

                let len = e.get_size();
                let schema_name = e.get_schema_name();
                let _ = local_context
                    .insert(schema_name.clone(), Schema::Enum(Arc::new(e)))
                    .ok_or(D::Error::custom("could not insert schema"))?;
                Ok(TypeSchema::EnumArray(schema_name, len))
            },

            Some(t) => Err(D::Error::custom(format!("unknown type: {}", t))),
            None => Ok(TypeSchema::None),
        }?;

        Ok(StructFieldSchema {
            parent_name: local_context.parent_name.clone(),

            key,
            name,
            description,
            required,
            deprecated,
            field_type,
        })
    }
}

impl<'a> DeserializeWithContext<'a> for StructFieldSchemas {
    fn deserialize_with_context<'de, D>(
        deserializer: D,
        local_context: &'a LocalContext<'a>,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{SeqAccess, Visitor};

        struct FieldSeqVisitor<'a>(&'a LocalContext<'a>);

        impl<'de, 'a> Visitor<'de> for FieldSeqVisitor<'a> {
            type Value = Vec<StructFieldSchema>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a sequence of StructFieldSchema")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut items = Vec::with_capacity(seq.size_hint().unwrap_or(4));

                while let Some(variant) =
                    seq.next_element_seed(WithContext::<StructFieldSchema>::new(self.0))?
                {
                    items.push(variant);
                }

                Ok(items)
            }
        }

        deserializer.deserialize_seq(FieldSeqVisitor(local_context))
    }
}
