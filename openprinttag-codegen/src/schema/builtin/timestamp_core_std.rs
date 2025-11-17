use syn::{parse_quote, Item};
use crate::emit::EmitIdent;
use crate::schema::Timestamp;

impl Timestamp {
    pub fn core_std_impls(&self) -> Vec<Item> {
        if let Some(ident) = self.rs_core_ident() {
            vec![
                parse_quote! {
                    #[cfg(feature = "std")]
                    impl From<#ident> for std::time::SystemTime {
                        #[inline]
                        fn from(ts: #ident) -> std::time::SystemTime {
                            use std::time::{Duration, UNIX_EPOCH};
                            UNIX_EPOCH + Duration::from_secs(ts.0 as u64)
                        }
                    }
                },
            ]
        } else {
            Vec::new()
        }
    }
}
