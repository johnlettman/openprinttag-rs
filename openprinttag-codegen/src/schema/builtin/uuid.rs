use syn::{parse_quote, Item, Type};
use crate::emit::{EmitIdent, EmitItems, EmitType};

pub struct Uuid;

impl Uuid {

}

impl EmitIdent for Uuid {
    fn get_name(&self) -> Option<String> {
        Some("Uuid".to_string())
    }
}

impl EmitType for Uuid {
    #[inline]
    fn emit_core_type(&self) -> Option<Type> {
        self.rs_core_ident().map(|i| parse_quote!(#i))
    }
}



impl EmitItems for Uuid {
    fn emit_core_items(&self) -> Vec<Item> {
        if let Some(ident) = self.rs_core_ident() {
            let mut items = vec![
                parse_quote! {
                    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
                    #[repr(transparent)]
                    pub struct #ident(pub uuid::Uuid);
                },
            ];

            items.extend(self.core_common_impls());
            items.extend(self.core_from_impls());
            items.extend(self.core_cbor_impls());

            items
        } else {
            Vec::new()
        }
    }

}
