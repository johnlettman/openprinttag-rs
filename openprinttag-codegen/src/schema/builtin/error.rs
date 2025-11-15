use crate::schema::gen::{ToItem, ToItems};
use syn::{parse_quote, Item};

pub struct Error;

impl ToItems for Error {
    fn to_items(&self) -> Vec<Item> {
        vec![
            parse_quote! {
                #[derive(Debug, thiserror::Error)]
                pub enum Error {
                    #[error("unexpected type")]
                    UnexpectedType,

                    #[error("invalid enum discriminant: {0}")]
                    InvalidEnumDiscriminant(u32),

                    #[error("too many items in the array")]
                    TooManyItems,

                    #[error("missing field: {0}")]
                    MissingField(&'static str),

                    #[error("CBOR decode error")]
                    DecodeError(minicbor::decode::Error)
                }
            },
            parse_quote! {
                impl From<minicbor::decode::Error> for Error {
                    #[inline(always)]
                    fn from(e: minicbor::decode::Error) -> Self {
                        Self::DecodeError(e)
                    }
                }
            },
        ]
    }
}
