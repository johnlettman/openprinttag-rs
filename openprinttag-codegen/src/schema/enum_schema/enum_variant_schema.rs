use crate::{
    de::{DeserializeWithContext, WithContext},
    schema::{
        context::LocalContext,
        gen::{GetAttributes, GetDoc, GetIdent, GetVariant},
    },
};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use syn::{parse_quote, Attribute, Variant};
use crate::schema::name;

pub type EnumVariantSchemas = Vec<EnumVariantSchema>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumVariantSchema {
    #[serde(skip)]
    pub parent_name: Option<String>,

    pub key: u32,

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

impl EnumVariantSchema {
    #[inline(always)]
    pub fn with_parent_name(mut self, parent_name: Option<String>) -> Self {
        self.parent_name = parent_name;
        self
    }

    #[inline(always)]
    pub fn has_parent(&self) -> bool {
        self.parent_name.is_some()
    }

    #[inline]
    pub fn get_hints_md(&self) -> Option<String> {
        name::to_enum_references_md(self.parent_name.as_ref()?, &self.hints)
    }

    #[inline]
    pub fn get_implies_md(&self) -> Option<String> {
        name::to_enum_references_md(self.parent_name.as_ref()?, &self.implies)
    }
}

impl GetIdent for EnumVariantSchema {
    fn get_name(&self) -> Option<String> {
        Some(name::to_camel(self.name.clone()?))
    }
}

impl GetDoc for EnumVariantSchema {
    fn get_doc(&self) -> Option<String> {
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

impl GetAttributes for EnumVariantSchema {
    //noinspection DuplicatedCode
    fn get_attributes(&self) -> Vec<Attribute> {
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

impl GetVariant for EnumVariantSchema {
    fn get_variant(&self) -> Option<Variant> {
        let ident = self.get_ident()?;
        let key = self.key;
        let attrs = self.get_attributes();

        Some(parse_quote! {
            #(#attrs)*
            #ident = #key
        })
    }
}

impl<'a> DeserializeWithContext<'a> for EnumVariantSchema {
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
        remap_names(&mut raw_map, local_context.name_map.clone());

        let mut schema: EnumVariantSchema =
            EnumVariantSchema::deserialize(raw_map).map_err(D::Error::custom)?;

        schema.parent_name = local_context.parent_name.clone();
        Ok(schema)
    }
}

impl<'a> DeserializeWithContext<'a> for EnumVariantSchemas {
    fn deserialize_with_context<'de, D>(
        deserializer: D,
        local_context: &'a LocalContext<'a>,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{SeqAccess, Visitor};

        struct VariantSeqVisitor<'a>(&'a LocalContext<'a>);

        impl<'de, 'a> Visitor<'de> for VariantSeqVisitor<'a> {
            type Value = Vec<EnumVariantSchema>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a sequence of EnumVariantSchema")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut items = Vec::with_capacity(seq.size_hint().unwrap_or(4));

                while let Some(variant) =
                    seq.next_element_seed(WithContext::<EnumVariantSchema>::new(self.0))?
                {
                    items.push(variant);
                }

                Ok(items)
            }
        }

        deserializer.deserialize_seq(VariantSeqVisitor(local_context))
    }
}
