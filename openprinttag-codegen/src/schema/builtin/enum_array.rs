use crate::emit::{EmitAttributes, EmitIdent, EmitPubVisibility, EmitVisibility, EmitItems, EmitDocsAsAttributes};
use syn::{parse_quote, Item};
use crate::schema::HasDocs;

pub struct EnumArray;

impl EnumArray {
    pub const NAME: &'static str = "EnumArray";
    pub const DOC: &'static str = include_str!("../docs/enum_array.md");
}

impl EmitIdent for EnumArray {
    #[inline(always)]
    fn get_name(&self) -> Option<String> {
        Some(Self::NAME.to_string())
    }
}

impl EmitPubVisibility for EnumArray {}

impl HasDocs for EnumArray {
    #[inline(always)]
    fn docs(&self) -> Option<String> {
        Some(Self::DOC.to_string())
    }
}

impl EmitDocsAsAttributes for EnumArray {}

impl EmitItems for EnumArray {
    fn emit_core_items(&self) -> Vec<Item> {
        let ident = self.rs_core_ident();
        let vis = self.emit_visibility();
        let attrs = self.emit_core_attributes();

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
