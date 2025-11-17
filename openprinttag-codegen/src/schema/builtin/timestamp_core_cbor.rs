use syn::{parse_quote, Item};
use crate::emit::EmitIdent;
use crate::schema::Timestamp;

impl Timestamp {
    pub fn core_cbor_impls(&self) -> Vec<Item> {
        if let Some(ident) = self.rs_core_ident() {
            vec![
                parse_quote! {
                        impl<C> minicbor::Encode<C> for #ident {
                            fn encode<W>(&self, e: &mut minicbor::Encoder<W>, c: &mut C) -> Result<(), minicbor::encode::Error<W::Error>>
                            where
                                W: minicbor::encode::Write
                            {
                                minicbor::Encode::<C>::encode(&self.0, e, c)
                            }

                            fn is_nil(&self) -> bool {
                                minicbor::Encode::<C>::is_nil(&self.0)
                            }
                        }
                    },
                parse_quote! {
                        impl<'b, C> minicbor::Decode<'b, C> for #ident {
                            fn decode(d: &mut minicbor::Decoder<'b>, c: &mut C) -> core::result::Result<Self, minicbor::decode::Error> {
                                Ok(Self(minicbor::Decode::<C>::decode(d, c)?))
                            }

                            fn nil() -> Option<Self> {
                                minicbor::Decode::<C>::nil().map(Self)
                            }
                        }
                    }
            ]
        } else {
            Vec::new()
        }
    }
}
