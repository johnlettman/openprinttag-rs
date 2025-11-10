use crate::{gen, name};
use proc_macro2::Span;
use serde::{Deserialize, Serialize};
use std::fmt::Write;

pub type EnumVariants = Vec<EnumVariant>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumVariant {
    pub key: u32,

    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    pub full_name: Option<String>,

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

impl EnumVariant {
    #[inline(always)]
    pub fn get_variant_name(&self) -> Option<String> {
        self.name.as_ref().map(|n| name::to_camel(n.clone()))
    }

    #[inline(always)]
    pub fn make_ident(&self) -> Option<syn::Ident> {
        self.get_variant_name().map(gen::make_ident)
    }

    pub fn get_doc<EN: AsRef<str>>(&self, name: EN) -> Option<String> {
        let mut doc = String::new();

        if let Some(full_name) = &self.full_name {
            writeln!(doc, "**{}**", full_name).ok();
        }

        if let Some(description) = &self.description {
            doc.push_str(description);
        }

        let mut extra_details = Vec::new();
        if let Some(hints_md) = self.get_hints_md(name.as_ref()) {
            extra_details.push(format!("**Hints:** {}", hints_md));
        }

        if let Some(implies_md) = self.get_implies_md(name.as_ref()) {
            extra_details.push(format!("**Implies:** {}", implies_md));
        }

        if !extra_details.is_empty() {
            doc.push_str("\n\n");
            doc.push_str(&extra_details.join("\n"))
        }

        (!doc.is_empty()).then_some(doc)
    }

    pub fn make_doc_attribute<EN: AsRef<str>>(&self, name: EN) -> Option<syn::Attribute> {
        self.get_doc(name).map(gen::make_doc_attribute)?.ok()
    }

    pub fn make_discriminant(&self) -> syn::Expr {
        use syn::{ExprLit, Lit, LitInt};

        syn::Expr::Lit(ExprLit {
            attrs: Vec::new(),
            lit: Lit::Int(LitInt::new(&self.key.to_string(), Span::call_site())),
        })
    }

    pub fn make_variant<EN: AsRef<str>>(&self, name: EN) -> Option<syn::Variant> {
        let ident = self.make_ident()?;

        let attrs = self.make_doc_attribute(name).map(|a| vec![a]).unwrap_or_default();

        Some(syn::Variant {
            attrs,
            ident,
            fields: syn::Fields::Unit,
            discriminant: Some((Default::default(), self.make_discriminant())),
        })
    }

    #[inline(always)]
    pub fn has_description(&self) -> bool {
        self.description != None || self.full_name != None
    }

    #[inline(always)]
    pub fn has_category(&self) -> bool {
        self.category != None
    }

    #[inline(always)]
    pub fn has_hints(&self) -> bool {
        !self.hints.is_empty()
    }

    #[inline(always)]
    pub fn get_hints_md<EN: AsRef<str>>(&self, name: EN) -> Option<String> {
        name::to_enum_references_md(name, &self.hints)
    }

    #[inline(always)]
    pub fn has_implies(&self) -> bool {
        !self.implies.is_empty()
    }

    #[inline(always)]
    pub fn get_implies_md<EN: AsRef<str>>(&self, name: EN) -> Option<String> {
        name::to_enum_references_md(name, &self.implies)
    }
}
