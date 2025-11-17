use syn::{parse_quote, Item};
use crate::emit::EmitIdent;
use crate::schema::Uuid;

impl Uuid {
    pub fn core_from_impls(&self) -> Vec<Item> {
        if let Some(ident) = self.rs_core_ident() {
            vec![
                parse_quote! {
                    impl From<[u8; 16]> for #ident {
                        #[inline]
                        fn from(bytes: [u8; 16]) -> Self {
                            Self(uuid::Uuid::from_bytes(bytes))
                        }
                    }
                },

                parse_quote! {
                    impl From<#ident> for [u8; 16] {
                        #[inline(always)]
                        fn from(u: #ident) -> Self {
                            *u.as_bytes()
                        }
                    }
                },

                parse_quote! {
                    impl core::str::FromStr for #ident {
                        type Err = uuid::Error;

                        #[inline]
                        fn from_str(s: &str) -> Result<Self, Self::Err> {
                            uuid::Uuid::parse_str(s).map(Self)
                        }
                    }
                },

                parse_quote! {
                    impl From<#ident> for uuid::Uuid {
                        #[inline(always)]
                        fn from(v: #ident) -> uuid::Uuid {
                            v.0
                        }
                    }
                },

                parse_quote! {
                    impl From<uuid::Uuid> for #ident {
                        #[inline(always)]
                        fn from(v: uuid::Uuid) -> Self {
                            Self(v)
                        }
                    }
                }
            ]
        } else {
            Vec::new()
        }
    }
}
