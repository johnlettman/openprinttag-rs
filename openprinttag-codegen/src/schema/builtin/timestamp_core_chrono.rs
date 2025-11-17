use syn::{parse_quote, Item};
use crate::emit::EmitIdent;
use crate::schema::Timestamp;

impl Timestamp {
    pub fn core_chrono_impls(&self) -> Vec<Item> {
        if let Some(ident) = self.rs_core_ident() {
            vec![
                parse_quote! {
                    #[cfg(all(feature = "chrono", feature = "std"))]
                    impl From<#ident> for chrono::DateTime<chrono::Utc> {
                        fn from(ts: #ident) -> Self {
                            chrono::DateTime::from_timestamp(ts.0 as i64, 0).expect("invalid Unix timestamp")
                        }
                    }
                },
                parse_quote! {
                    #[cfg(all(feature = "chrono", feature = "std"))]
                    impl TryFrom<chrono::DateTime<chrono::Utc>> for #ident {
                        type Error = std::num::TryFromIntError;

                        fn try_from(dt: chrono::DateTime<chrono::Utc>) -> Result<Self, Self::Error> {
                            let secs = dt.timestamp();
                            u32::try_from(secs).map(Self)
                        }
                    }
                }
            ]
        } else {
            Vec::new()
        }
    }
}
