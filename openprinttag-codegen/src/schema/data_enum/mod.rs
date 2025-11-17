mod data_enum_variant;
mod data_enum_variants;

pub use data_enum_variant::*;
pub use data_enum_variants::*;

use crate::{
    de::DeserializeWithContext,
    emit::{
        util, EmitAttributes, EmitIdent, EmitPubVisibility,
        EmitType, EmitVariant, EmitVariants, EmitVisibility, EmitItems,
    },
    schema::{
        builtin,HasDocs,SchemaName, DataSize,
        context::LocalContext,

    },
    tracing::trace,
};
use serde::{de::Error, Deserializer};
use std::fmt;
use syn::{parse_quote, Arm, Attribute, Expr, Item, Type, Variant};
use crate::emit::util::emit_type;

#[derive(Clone, derive_new::new)]
pub struct DataEnum {
    pub schema_name: String,

    pub description: Option<String>,
    pub variants: DataEnumVariants,
}

impl fmt::Debug for DataEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EnumSchema")
            .field("schema_name", &self.schema_name)
            .field("description", &self.description)
            .field("variants", &format_args!("EnumVariantSchemas(len={})", self.variants.len()))
            .finish()
    }
}

impl DataEnum {
    pub fn has_display_names(&self) -> bool {
        self.variants.iter().any(|v| v.display_name.is_some())
    }

    pub fn has_implies(&self) -> bool {
        self.variants.iter().any(|v| !v.implies.is_empty())
    }

    pub fn has_hints(&self) -> bool {
        self.variants.iter().any(|v| !v.hints.is_empty())
    }
}

impl SchemaName for DataEnum {
    #[inline]
    fn get_schema_name(&self) -> String {
        self.schema_name.clone()
    }
}

impl DataSize for DataEnum {
    #[inline(always)]
    fn size(&self) -> usize {
        self.variants.len()
    }
}

impl EmitPubVisibility for DataEnum {}

impl EmitType for DataEnum {
    #[inline]
    fn emit_core_type(&self) -> Option<Type> {
        Some(emit_type(self.get_name()?))
    }
}

impl HasDocs for DataEnum {
    #[inline]
    fn docs(&self) -> Option<String> {
        self.description.clone()
    }
}

impl EmitAttributes for DataEnum {
    fn emit_core_attributes(&self) -> Vec<Attribute> {
        let mut attrs = vec![
            parse_quote!(#[repr(u16)]),
            parse_quote!(#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]),
        ];

        if let Some(doc) = self.doc_attribute() {
            attrs.push(doc)
        }

        attrs
    }
}

impl EmitVariants for DataEnum {
    fn emit_core_variants(&self) -> Vec<Variant> {
        self.variants.iter().filter_map(|v| v.emit_core_variant()).collect()
    }
}

impl EmitItems for DataEnum {
    #[inline]
    fn emit_core_items(&self) -> Vec<Item> {
        if let Some(ident) = self.rs_core_ident() {
            let attrs = self.emit_core_attributes();
            let vis = self.emit_visibility();
            let variants = self.get_core_variants_punctuated();

            let mut items = vec![parse_quote! {
                #(#attrs)*
                #vis enum #ident {
                    #variants
                }
            }];

            let error_ident =
                builtin::Error.rs_core_ident().expect("Error enum should have ident");

            let mut try_from_u16_arms: Vec<Arm> = self
                .variants
                .iter()
                .filter_map(|v| {
                    let v_key = v.key;
                    let v_ident = v.rs_core_ident()?;
                    Some(parse_quote!(#v_key => Ok(Self::#v_ident),))
                })
                .collect();
            try_from_u16_arms
                .push(parse_quote!(other => Err(#error_ident::InvalidEnumDiscriminant(other)),));

            items.push(parse_quote! {
                impl TryFrom<u16> for #ident {
                    type Error = #error_ident;

                    fn try_from(value: u16) -> Result<Self, Self::Error> {
                        match value {
                            #(#try_from_u16_arms)*
                        }
                    }
                }
            });

            items.extend(["u8", "u32", "u64", "usize", "i8", "i16", "i32", "i64", "isize"].iter().map(|ty| {
                let ty_ident = emit_type(ty);
                let ty_range_bounds_check: Expr = if ty.starts_with('u') {
                    parse_quote!(value > u16::MAX as #ty_ident)
                } else {
                    parse_quote!(value < 0 || value > u16::MAX as #ty_ident)
                };

                parse_quote! {
                    impl TryFrom<#ty_ident> for #ident {
                        type Error = #error_ident;

                        //noinspection DuplicatedCode
                        fn try_from(value: #ty_ident) -> Result<Self, Self::Error> {
                            if #ty_range_bounds_check {
                                return Err(#error_ident::InvalidEnumDiscriminant(value as u16));
                            }
                            Self::try_from(value as u16)
                        }
                    }
                }
            }));

            items.push(parse_quote! {
                impl<'b, C> minicbor::Decode<'b, C> for #ident {
                    fn decode(d: &mut minicbor::Decoder<'b>, _ctx: &mut C) -> Result<#ident, minicbor::decode::Error> {
                        let pos = d.position();
                        let n = d.i64()?;
                        #ident::try_from(n).map_err(|_| minicbor::decode::Error::unknown_variant(n).at(pos))
                    }
                }
            });

            items
        } else {
            Vec::new()
        }
    }
}

impl<'a> DeserializeWithContext<'a> for DataEnum {
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "debug", ret, fields(%schema = local_context.schema_name), skip(local_context)))]
    fn deserialize_with_context<'de, D>(
        _: D,
        local_context: &'a LocalContext<'a>,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut schema = DataEnum {
            schema_name: local_context.schema_name.to_string(),
            description: local_context.description.clone(),
            variants: vec![],
        };

        let mut variant_context = LocalContext::new_from(local_context, local_context.schema_name);
        variant_context.parent_name = schema.get_name();
        trace!("Built local context for enum variants: {:?}", variant_context);

        schema.variants = local_context
            .seed_local::<DataEnumVariants>()
            .map_err(|e| D::Error::custom(e.to_string()))?;

        Ok(schema)
    }
}
