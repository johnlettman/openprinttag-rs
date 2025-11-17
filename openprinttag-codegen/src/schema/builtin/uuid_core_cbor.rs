use syn::{parse_quote, Item};
use crate::emit::EmitIdent;
use crate::schema::Uuid;

impl Uuid {
    pub fn core_cbor_impls(&self) -> Vec<Item> {
        if let Some(ident) = self.rs_core_ident() {
            vec![
                parse_quote! {
                    impl<C> minicbor::Encode<C> for #ident {
                        fn encode<W>(&self, e: &mut minicbor::Encoder<W>, c: &mut C) -> core::result::Result<(), minicbor::encode::Error<W::Error>>
                        where
                            W: minicbor::encode::Write
                        {
                            let bytes = self.as_bytes();
                            minicbor::Encode::<C>::encode(bytes, e, c)
                        }
                    }
                },

                parse_quote! {
                    impl<'b, C> minicbor::Decode<'b, C> for #ident {
                        fn decode(d: &mut minicbor::Decoder<'b>, c: &mut C) -> core::result::Result<Self, minicbor::decode::Error> {
                            let bytes: [u8; 16] = minicbor::Decode::decode(d, c)?;
                            Ok(Self(uuid::Uuid::from_bytes(bytes)))
                        }

                        fn nil() -> core::option::Option<Self> {
                            <[u8; 16] as minicbor::Decode<C>>::nil()
                                .map(|bytes| Self(uuid::Uuid::from_bytes(bytes)))
                        }
                    }
                }
            ]
        } else {
            Vec::new()
        }
    }

}
