use crate::{data, Fields};
use serde::{Deserialize, Deserializer, Serialize};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Config {
    pub mime_type: String,

    pub root: Option<String>,

    /// The meta section allows defining of region offsets (within the `NDEF` payload) and sizes.
    ///
    /// See: https://specs.openprinttag.org/#/nfc_data_format?id=_4-meta-section
    pub meta_fields: Fields,

    /// The main section contains material information that does not change during the package
    /// instance lifetime.
    ///
    /// See: https://specs.openprinttag.org/#/nfc_data_format?id=_5-main-section
    pub main_fields: Fields,

    /// The auxiliary section is intended for dynamic data - typically usage tracking.
    ///
    /// See: https://specs.openprinttag.org/#/nfc_data_format?id=_6-auxiliary-section
    pub aux_fields: Fields,
}

impl Config {
    #[inline(always)]
    pub fn load<N: AsRef<str>>(name: N) -> crate::Result<Self> {
        data::load(name)
    }

    #[inline(always)]
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        data::load_from_path(path)
    }
}

impl<'de> Deserialize<'de> for Config {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub(crate) struct RawConfig {
            pub(crate) mime_type: String,
            pub(crate) root: Option<String>,
            pub(crate) meta_fields: Option<String>,
            pub(crate) main_fields: Option<String>,
            pub(crate) aux_fields: Option<String>,
        }

        let raw = RawConfig::deserialize(deserializer)?;
        Ok(Config {
            mime_type: raw.mime_type,
            root: raw.root,
            meta_fields: data::maybe_load(raw.meta_fields).unwrap_or(Vec::new()),
            main_fields: data::maybe_load(raw.main_fields).unwrap_or(Vec::new()),
            aux_fields: data::maybe_load(raw.aux_fields).unwrap_or(Vec::new()),
        })
    }
}
