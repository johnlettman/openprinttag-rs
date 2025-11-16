use crate::{
    de::DeserializeWithContext,
    schema::{
        context::LocalContext,
        gen::{
            util, GetAttributes, GetDoc, GetIdent, GetPubVisibility, GetSchemaName, GetSize,
            GetType, GetVariant, GetVariants, GetVisibility, ToItem,
        },
        EnumVariantSchemas,
    },
};
use serde::{de::Error, Deserializer};
use syn::{parse_quote, Arm, Attribute, Expr, Item, Type, Variant};
use crate::schema::builtin;
use crate::schema::gen::ToItems;

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
    fn get_core_type(&self) -> Option<Type> {
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
    fn get_core_attributes(&self) -> Vec<Attribute> {
        let mut attrs = vec![
            parse_quote!(#[repr(u16)]),
            parse_quote!(#[derive(
                Debug, Copy, Clone, PartialEq, Eq, minicbor_derive::Encode, minicbor_derive::Decode
            )]),
        ];

        if let Some(doc) = self.get_doc_attribute() {
            attrs.push(doc)
        }

        attrs
    }
}

impl GetVariants for EnumSchema {
    fn get_core_variants(&self) -> Vec<Variant> {
        self.variants.iter().filter_map(|v| v.get_core_variant()).collect()
    }
}

impl ToItems for EnumSchema {
    #[inline]
    fn to_core_items(&self) -> Vec<Item> {
        if let Some(ident) = self.get_core_ident() {
            let attrs = self.get_core_attributes();
            let vis = self.get_visibility();
            let variants = self.get_core_variants_punctuated();

            let mut items = vec![
                parse_quote! {
                    #(#attrs)*
                    #vis enum #ident {
                        #variants
                    }
                }
            ];

            let error_ident = builtin::Error.get_core_ident().expect("Error enum should have ident");



            let mut try_from_u16_arms: Vec<Arm> = self.variants.iter().filter_map(|v| {
                let v_key = v.key;
                let v_ident = v.get_core_ident()?;
                Some(parse_quote!(#v_key => Ok(Self::#v_ident),))
            }).collect();
            try_from_u16_arms.push(parse_quote!(other => Err(#error_ident::InvalidEnumDiscriminant(other)),));

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
                let ty_ident = util::make_type(ty).expect("should make int type");
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
                impl<'b, C> minicbor::Decode<'b, C> for MaterialType {
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

impl<'a> DeserializeWithContext<'a> for EnumSchema {
    #[cfg_attr(feature = "tracing", tracing::instrument(debug, skip(deserializer)))]
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
