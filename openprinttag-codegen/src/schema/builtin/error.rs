use syn::{parse_quote, Attribute, Item};
use crate::emit::{EmitAttributes, EmitItems, EmitPubVisibility, EmitIdent};
use crate::emit::util::emit_doc_attribute;

pub struct Error;

impl EmitIdent for Error {
    #[inline(always)]
    fn get_name(&self) -> Option<String> {
        Some("Error".to_string())
    }
}

impl EmitAttributes for Error {
    fn emit_core_attributes(&self) -> Vec<Attribute> {
        vec![
            parse_quote!(#[derive(Debug, thiserror::Error)]),
            emit_doc_attribute("OpenPrintTag errors.")
        ]
    }
}

impl EmitPubVisibility for Error {}

impl EmitItems for Error {
    fn emit_core_items(&self) -> Vec<Item> {
        if let Some(ident) = self.rs_core_ident() {
            let attrs = self.emit_core_attributes();

            vec![
                parse_quote! {
                    #(#attrs)*
                    pub enum #ident {
                        #[error("unexpected type")]
                        UnexpectedType,

                        #[error("invalid enum discriminant: {0}")]
                        InvalidEnumDiscriminant(u16),

                        #[error("too many items in the array")]
                        TooManyItems,

                        #[error("missing field: {0}")]
                        MissingField(&'static str),

                        #[error("CBOR decode error")]
                        DecodeError(minicbor::decode::Error)
                    }
                },
                parse_quote! {
                    impl From<minicbor::decode::Error> for #ident {
                        #[inline(always)]
                        fn from(e: minicbor::decode::Error) -> Self {
                            Self::DecodeError(e)
                        }
                    }
                },
            ]
        } else {
            Vec::new()
        }
    }
}
