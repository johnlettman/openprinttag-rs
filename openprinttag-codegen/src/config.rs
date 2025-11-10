use crate::{data, Fields};
use serde::{Deserialize, Deserializer, Serialize};
use std::path::Path;

/// Represents an [OpenPrintTag] configuration schema.
///
/// A `Config` defines the structure of an [OpenPrintTag data layout], composed
/// of three major field groups:
///
/// - **Meta section** (`meta_fields`): Defines region offsets and sizes within
///   the NDEF payload.
/// - **Main section** (`main_fields`): Contains static material information.
/// - **Auxiliary section** (`aux_fields`): Contains dynamic runtime or
///   usage-tracking data.
///
/// This struct corresponds to the `config_*` schema files found in the crate’s
/// `data` directory. These files define the field layout, metadata, and
/// auxiliary data formats used by [OpenPrintTag]-compliant devices and
/// software.
///
/// # Example
/// ```rust
/// use openprinttag_codegen::Config;
///
/// // load a built-in configuration such as `config_nfcv.yaml`
/// let config = Config::load("config_nfcv").expect("should load and deserialize configuration");
///
/// println!("MIME type: {}", config.mime_type);
/// println!("Main fields: {} entries", config.main_fields.len());
/// ```
///
/// # Structure
/// | Section       | Purpose                                                   | Reference           |
/// |---------------|-----------------------------------------------------------|---------------------|
/// | **Meta**      | Defines offsets, sizes, and low-level NFC layout details. | [Meta section]      |
/// | **Main**      | Defines static, per-material properties.                  | [Main section]      |
/// | **Auxiliary** | Defines dynamic or runtime tracking data.                 | [Auxiliary section] |
///
/// [Meta section]: https://specs.openprinttag.org/#/nfc_data_format?id=_4-meta-section
/// [Main section]: https://specs.openprinttag.org/#/nfc_data_format?id=_5-main-section
/// [Auxiliary section]: https://specs.openprinttag.org/#/nfc_data_format?id=_6-auxiliary-section
/// [OpenPrintTag]: https://openprinttag.org/
/// [OpenPrintTag data layout]: https://specs.openprinttag.org/#/nfc_data_format
#[derive(Debug, Clone)]
pub struct Config {
    /// The MIME type of this configuration (e.g.
    /// `"application/vnd.openprinttag"`).
    pub mime_type: String,

    /// The optional root section for hierarchical configurations.
    pub root: Option<String>,

    /// The **meta section**, defining offsets and sizes within the `NDEF`
    /// payload.
    ///
    /// # See also
    /// - https://specs.openprinttag.org/#/nfc_data_format?id=_4-meta-section
    pub meta_fields: Fields,

    /// The **main section** contains material information that does not change
    /// during the package instance lifetime.
    ///
    /// # See also
    /// - https://specs.openprinttag.org/#/nfc_data_format?id=_5-main-section
    pub main_fields: Fields,

    /// The **auxiliary section** is intended for dynamic data, typically usage
    /// tracking.
    ///
    /// # See also
    /// - https://specs.openprinttag.org/#/nfc_data_format?id=_6-auxiliary-section
    pub aux_fields: Fields,
}

impl Config {
    /// Loads a [`Config`] definition from the crate’s `data` directory.
    ///
    /// This is a convenience wrapper around [`data::load`], which automatically
    /// resolves the file path.
    ///
    /// # Type Parameters
    /// - `S`: Type of the schema name (typically `str` or [`String`]).
    ///
    /// # Arguments
    /// - `name`: The base file name (with or without `.yaml`) of the
    ///   configuration.
    ///
    /// # Errors
    /// - [`crate::Error::DataLoadError`]: if the file cannot be read from disk.
    /// - [`crate::Error::YAMLParseError`]: if the YAML is invalid or cannot be
    ///   parsed.
    ///
    /// # Example
    /// ```rust
    /// use openprinttag_codegen::config::Config;
    ///
    /// let config = Config::load("config_nfcv").expect("should load config_nfcv.yaml");
    /// assert_eq!(config.mime_type, "application/vnd.openprinttag");
    /// ```
    #[inline]
    pub fn load<S: AsRef<str>>(name: S) -> crate::Result<Self> {
        data::load(name)
    }

    /// Loads a [`Config`] definition directly from an explicit file path.
    ///
    /// This is equivalent to [`Config::load`] but allows specifying an
    /// arbitrary file path.
    ///
    /// # Type Parameters
    /// - `P`: Type of the path (typically [`Path`][std::path::Path]).
    ///
    /// # Arguments
    /// - `path`: Path to a YAML schema file.
    ///
    /// # Errors
    /// - [`crate::Error::DataLoadError`]: if the file cannot be read from disk.
    /// - [`crate::Error::YAMLParseError`]: if the YAML is invalid or cannot be
    ///   parsed.
    ///
    /// # Example
    /// ```rust
    /// use openprinttag_codegen::{config::Config, data::get_data_path};
    ///
    /// let path = get_data_path("config_nfcv");
    /// let config = Config::load_from_path(path).expect("should load from path");
    /// assert_eq!(config.mime_type, "application/vnd.openprinttag");
    /// ```
    #[inline]
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
