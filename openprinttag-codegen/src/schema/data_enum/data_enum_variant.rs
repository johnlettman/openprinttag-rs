use crate::{
    de::{DeserializeWithContext, WithContext},
    emit::{EmitAttributes, EmitIdent, EmitVariant},
    schema::{
        context::LocalContext,
        HasDocs,
        name,
    },
    tracing::trace,
};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use syn::{parse_quote, Attribute, Variant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataEnumVariant {
    #[serde(skip)]
    pub parent_name: Option<String>,

    pub key: u16,

    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    pub display_name: Option<String>,

    #[serde(default)]
    pub deprecated: bool,

    #[serde(
        default,
        deserialize_with = "crate::de::description",
        serialize_with = "crate::ser::description"
    )]
    pub description: Option<String>,

    /// Category for the enumeration value. Used in `material_type`.
    #[serde(default)]
    pub category: Option<String>,

    /// Hint for potentially relevant enumeration values. Used in `tags`.
    #[serde(default)]
    pub hints: Vec<String>,

    /// Implications of other enumeration values. Used in `tags`.
    #[serde(default)]
    pub implies: Vec<String>,
}

impl DataEnumVariant {
    #[inline(always)]
    pub fn with_parent_name(mut self, parent_name: Option<String>) -> Self {
        self.parent_name = parent_name;
        self
    }

    #[inline(always)]
    pub fn has_parent(&self) -> bool {
        self.parent_name.is_some()
    }

    pub fn get_hints(&self) -> Vec<String> {
        if let Some(parent_name) = &self.parent_name {
            name::to_enum_references(parent_name, &self.hints)
        } else {
            Vec::new()
        }
    }

    #[inline]
    pub fn get_hints_md(&self) -> Option<String> {
        name::to_enum_references_md(self.parent_name.as_ref()?, &self.hints)
    }

    #[inline]
    pub fn get_implies_md(&self) -> Option<String> {
        name::to_enum_references_md(self.parent_name.as_ref()?, &self.implies)
    }

    pub fn get_implies(&self) -> Vec<String> {
        if let Some(parent_name) = &self.parent_name {
            name::to_enum_references(parent_name, &self.implies)
        } else {
            Vec::new()
        }
    }
}

impl EmitIdent for DataEnumVariant {
    fn get_name(&self) -> Option<String> {
        Some(name::to_camel(self.name.clone()?))
    }
}

impl HasDocs for DataEnumVariant {
    fn docs(&self) -> Option<String> {
        use std::fmt::Write;
        let mut doc = String::new();

        if let Some(display_name) = &self.display_name {
            writeln!(doc, "**{}**", display_name).ok();
        }

        if let Some(description) = &self.description {
            doc.push_str(description);
        }

        if self.has_parent() {
            let mut extra = Vec::new();

            if let Some(hints_md) = self.get_hints_md() {
                extra.push(format!("**Hints:** {}", hints_md));
            }

            if let Some(implies_md) = self.get_implies_md() {
                extra.push(format!("**Implies:** {}", implies_md));
            }

            if !extra.is_empty() {
                doc.push_str("\n\n");
                doc.push_str(&extra.join("\n"))
            }
        }

        (!doc.is_empty()).then_some(doc)
    }
}

impl EmitAttributes for DataEnumVariant {
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

impl EmitVariant for DataEnumVariant {
    fn emit_core_variant(&self) -> Option<Variant> {
        let ident = self.rs_core_ident()?;
        let key = self.key;
        let attrs = self.emit_core_attributes();

        Some(parse_quote! {
            #(#attrs)*
            #ident = #key
        })
    }
}

impl<'a> DeserializeWithContext<'a> for DataEnumVariant {
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "debug", ret, fields(%schema = local_context.schema_name), skip(deserializer, local_context)))]
    fn deserialize_with_context<'de, D>(
        deserializer: D,
        local_context: &'a LocalContext<'a>,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::de::remap_names;
        use serde::{de::Error, Deserialize};
        use serde_norway::Value;

        let mut raw_map: Value = Value::deserialize(deserializer)?;
        trace!("Raw enum variant map: {:?}", raw_map);

        remap_names(&mut raw_map, local_context.name_map.clone());

        let mut schema: DataEnumVariant =
            DataEnumVariant::deserialize(raw_map).map_err(D::Error::custom)?;

        schema.parent_name = local_context.parent_name.clone();
        Ok(schema)
    }
}
