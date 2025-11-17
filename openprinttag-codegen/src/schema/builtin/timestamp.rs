
use syn::{parse_quote, Item, Type};
use crate::emit::{EmitItems, EmitType, EmitIdent};

pub struct Timestamp;


impl EmitIdent for Timestamp {
    #[inline(always)]
    fn get_name(&self) -> Option<String> {
        Some("Timestamp".to_string())
    }
}

impl EmitType for Timestamp {
    #[inline]
    fn emit_core_type(&self) -> Option<Type> {
        self.rs_core_ident().map(|i| parse_quote!(#i))
    }
}

impl EmitItems for Timestamp {
    fn emit_core_items(&self) -> Vec<Item> {
        if let Some(ident) = self.rs_core_ident() {
            let mut items = vec![
                parse_quote! {
                    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
                    #[repr(transparent)]
                    pub struct #ident(pub u32);
                },
            ];

            items.extend(self.core_common_impls());
            items.extend(self.core_cbor_impls());
            items.extend(self.core_std_impls());
            items.extend(self.core_chrono_impls());

            items
        } else {
            Vec::new()
        }
    }
}
