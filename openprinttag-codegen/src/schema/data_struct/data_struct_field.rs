use crate::{
    de::DeserializeWithContext,
    schema::{
        context::{registry::Register, LocalContext},

        DataEnum, Schema, DataTy,
    },
    Required,
};
use std::{collections::HashMap, sync::Arc};
use syn::{parse_quote, Attribute, Expr, Field, Type};
use crate::emit::{EmitAttributes, EmitField, EmitPubVisibility, EmitType, EmitVisibility, EmitIdent};
use crate::schema::{DataSize, HasDocs, SchemaName};

#[derive(Debug, Clone)]
pub struct DataStructField {
    parent_name: Option<String>,

    pub key: u32,
    pub name: Option<String>,
    pub description: Option<String>,

    pub required: Option<Required>,
    pub deprecated: bool,

    pub field_type: DataTy,
}

impl EmitIdent for DataStructField {
    #[inline]
    fn get_name(&self) -> Option<String> {
        self.name.clone()
    }
}

impl EmitPubVisibility for DataStructField {}

impl EmitType for DataStructField {
    #[inline]
    fn emit_core_type(&self) -> Option<Type> {
        self.field_type.emit_core_type()
    }
}

impl HasDocs for DataStructField {
    fn docs(&self) -> Option<String> {
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

impl EmitAttributes for DataStructField {
    //noinspection DuplicatedCode
    fn emit_core_attributes(&self) -> Vec<Attribute> {
        let mut attrs = Vec::new();

        if let Some(doc) = self.doc_attribute() {
            attrs.push(doc);
        }

        if self.deprecated {
            attrs.push(parse_quote!(#[deprecated]));
        }

        attrs
    }
}

impl EmitField for DataStructField {
    fn emit_core_field(&self) -> Option<Field> {
        let attrs = self.emit_core_attributes();
        let vis = self.emit_visibility();
        let ident = self.rs_core_ident();
        let ty = self.emit_core_type()?;

        Some(parse_quote! {
            #(#attrs)*
            #vis #ident: #ty
        })
    }
}


#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct RawStructureField {
    pub(crate) key: u32,

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

impl<'a> DeserializeWithContext<'a> for DataStructField {
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "debug", fields(%schema = local_context.schema_name), skip(deserializer, local_context)))]
    fn deserialize_with_context<'de, D>(
        deserializer: D,
        local_context: &'a LocalContext<'a>,
    ) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use crate::de;
        use serde::{de::Error, Deserialize, Deserializer};
        use serde_norway::Value;

        let mut raw_map: Value = Value::deserialize(deserializer)?;
        de::remap_names(&mut raw_map, local_context.name_map.clone());

        let RawStructureField {
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
        } = RawStructureField::deserialize(raw_map).map_err(|e| Error::custom(e.to_string()))?;

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
        ) -> Result<DataEnum, D::Error>
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
            Some("uuid") => Ok(DataTy::UUID),
            Some("string") => Ok(DataTy::String(max_length.unwrap_or(255) as usize)),
            Some("bytes") => Ok(DataTy::Bytes(max_length.unwrap_or(255) as usize)),
            Some("int" | "integer") => {
                let example = example.as_ref().and_then(|v| v.as_u64()).map(|v| v as u32);
                Ok(DataTy::Integer { unit: unit.clone(), example })
            },

            Some("number") => {
                let example = example.as_ref().and_then(|v| v.as_f64()).map(|v| v as f32);
                Ok(DataTy::Number { unit: unit.clone(), example })
            },

            Some("timestamp") => Ok(DataTy::Timestamp),
            Some("enum") => {
                let e = seed_enum::<D>(local_context, items_file, name_field, display_name_field)
                    .map_err(|e| D::Error::custom(e))?;

                let schema_name = e.get_schema_name();
                let _ = local_context.insert(schema_name.clone(), Schema::Enum(Arc::new(e)));
                Ok(DataTy::Enum(schema_name))
            },
            Some("enum_array") => {
                let e = seed_enum::<D>(local_context, items_file, name_field, display_name_field)?;

                let len = e.size();
                let schema_name = e.get_schema_name();
                let _ = local_context.insert(schema_name.clone(), Schema::Enum(Arc::new(e)));
                Ok(DataTy::EnumArray(schema_name, len))
            },

            Some(t) => Err(D::Error::custom(format!("unknown type: {}", t))),
            None => Ok(DataTy::None),
        }?;

        Ok(DataStructField {
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
