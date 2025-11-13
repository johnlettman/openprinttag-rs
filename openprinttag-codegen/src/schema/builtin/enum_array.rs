use syn::{parse_quote, Item};
use crate::schema::gen::{AsItems, GetAttributes, GetDoc, GetDocAsAttributes, GetIdent, GetPubVisibility, GetVisibility};

pub struct EnumArray;

impl EnumArray {
    pub const NAME: &'static str = "EnumArray";
    pub const DOC: &'static str = include_str!("../docs/enum_array.md");
}

impl GetIdent for EnumArray {
    #[inline(always)]
    fn get_name(&self) -> Option<String> {
        Some(Self::NAME.to_string())
    }
}

impl GetPubVisibility for EnumArray {}

impl GetDoc for EnumArray {
    #[inline(always)]
    fn get_doc(&self) -> Option<String> {
        Some(Self::DOC.to_string())
    }
}

impl GetDocAsAttributes for EnumArray {}

impl AsItems for EnumArray {
    fn as_items(&self) -> Vec<Item> {
        let ident = self.get_ident();
        let vis = self.get_visibility();
        let attrs = self.get_attributes();

        vec![
            parse_quote! {
                #[cfg(feature = "std")]
                #(#attrs)*
                #vis type #ident<T, const N: usize> = std::vec::Vec<T>;
            },
            parse_quote! {
                #[cfg(not(feature = "std"))]
                #(#attrs)*
                #vis type #ident<T, const N: usize> = heapless::Vec<T, N>;
            },
        ]
    }
}
