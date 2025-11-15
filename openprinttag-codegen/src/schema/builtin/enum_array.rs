use crate::schema::gen::{
    GetAttributes, GetDoc, GetDocAsAttributes, GetIdent, GetPubVisibility, GetVisibility, ToItems,
};
use syn::{parse_quote, Item};

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

impl ToItems for EnumArray {
    fn to_items(&self) -> Vec<Item> {
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
                #[cfg(all(not(feature = "std"), feature = "alloc"))]
                #(#attrs)*
                #[doc = "(no `std`, using `alloc`)"]
                #vis type #ident<T, const N: usize> = alloc::vec::Vec<T>;
            },
            parse_quote! {

                #[cfg(all(not(feature = "std"), not(feature = "alloc")))]
                #(#attrs)*
                #[doc = "(no `std` and no `alloc`)"]
                #vis type #ident<T, const N: usize> = heapless::Vec<T, N>;
            },
        ]
    }
}
