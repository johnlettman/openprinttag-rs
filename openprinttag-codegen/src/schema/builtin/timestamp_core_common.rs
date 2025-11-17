use syn::{parse_quote, Item};
use crate::emit::EmitIdent;
use crate::schema::Timestamp;

impl Timestamp {
    pub fn core_common_impls(&self) -> Vec<Item> {
        if let Some(ident) = self.rs_core_ident() {
            vec![
                parse_quote! {
                    impl #ident {
                        #[inline(always)]
                        pub const fn new(secs: u32) -> Self {
                            Self(secs)
                        }

                        #[inline(always)]
                        pub const fn as_secs(self) -> u32 {
                            self.0
                        }

                        #[cfg(feature = "std")]
                        pub fn now() -> Self {
                            use core::cmp::min;
                            use std::time::{SystemTime, UNIX_EPOCH};

                            let dur = SystemTime::now().duration_since(UNIX_EPOCH).expect("system time is before Unix epoch");
                            let secs = min(dur.as_secs(), u32::MAX as u64) as u32;
                            Self(secs)
                        }
                    }
                },

                parse_quote! {
                    impl core::fmt::Display for #ident {
                        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                            #[cfg(all(feature = "chrono", feature = "std"))]
                            {
                                let dt: chrono::DateTime<chrono::Utc> = (*self).into();
                                return write!(f, "{}", dt.to_rfc3339());
                            }

                            #[cfg(all(feature = "time", not(feature = "chrono")))]
                            {
                                let dt: time::OffsetDateTime = (*self).into();
                                return write!(f, "{}", dt.format(&time::format_description::well_known::Rfc3339).unwrap());
                            }

                            #[cfg(all(feature = "std", not(feature = "chrono"), not(feature = "time")))]
                            {
                                use std::time::{Duration, UNIX_EPOCH};
                                let systime = UNIX_EPOCH + Duration::from_secs(self.0 as u64);
                                return write!(f, "{} (system)", self.0);
                            }

                            #[cfg(all(not(feature = "std"), not(feature = "chrono"), not(feature = "time")))]
                            {
                                return write!(f, "{}", self.0);
                            }
                        }
                    }
                },

                parse_quote! {
                    impl Default for #ident {
                        #[cfg(feature = "std")]
                        #[inline(always)]
                        fn default() -> Self {
                            Self::now()
                        }

                        #[cfg(not(feature = "std"))]
                        #[inline(always)]
                        fn default() -> Self {
                            Self(0)
                        }
                    }
                },

                parse_quote! {
                    impl From<#ident> for u32 {
                        #[inline(always)]
                        fn from(ts: #ident) -> u32 {
                            ts.0
                        }
                    }
                },

                parse_quote! {
                    impl From<u32> for #ident {
                        #[inline(always)]
                        fn from(secs: u32) -> Self {
                            Self(secs)
                        }
                    }
                }
            ]
        } else {
            Vec::new()
        }
    }
}
