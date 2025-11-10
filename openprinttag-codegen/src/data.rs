use crate::{de, EnumVariants, Error};
use serde::de::DeserializeOwned;
use serde_norway::Value;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Constructs an absolute path to a schema file located under the crate’s
/// `data` directory.
///
/// The function resolves paths relative to the crate root using
/// `CARGO_MANIFEST_DIR` environment variable at compile time and automatically
/// appends a `.yaml` extension if one is not already present.
///
/// This is used for resolving schema or enum data files, such as
/// - `data/material_type_enum.yaml`
/// - `data/config_nfcv.yaml`
///
/// # Type Parameters
/// - `S`: Type of the schema name (typically `str` or [`String`]).
///
/// # Arguments
/// - `name`: Name of the schema to load.
///
/// # Examples
/// ```rust
/// use openprinttag_codegen::data::get_data_path;
///
/// let path = get_data_path("material_type_enum");
/// assert!(path.ends_with("data/material_type_enum.yaml"));
///
/// let path = get_data_path("config_nfcv.yaml");
/// assert!(path.ends_with("data/config_nfcv.yaml"));
/// ```
pub fn get_data_path<S: AsRef<str>>(name: S) -> PathBuf {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let file_name = name.as_ref();
    let file_name =
        if file_name.ends_with(".yaml") { file_name } else { &*format!("{}.yaml", file_name) };

    crate_dir.join("data").join(file_name)
}

/// Loads and parses a YAML schema file from the disk into a generic [`Value`].
///
/// This function reads the file at the specified path, converts it to a UTF-8
/// string, and then deserializes it into a dynamically typed [`Value`]. It
/// forms the low-level foundation for higher-level loaders such as
/// [`load_mapped_from_path`] or [`load_enum_variants_from_path`].
///
/// # Type Parameters
/// - `P`: Type of the path (typically [`Path`][std::path::Path]).
///
/// # Arguments
/// - `path`: Path to the schema.
///
/// # Errors
/// - [`Error::DataLoadError`]: if the file cannot be read from disk.
/// - [`Error::YAMLParseError`]: if the YAML is invalid or cannot be parsed.
///
/// Both error variants include the full path to the file.
///
/// # Example
/// ```rust
/// use openprinttag_codegen::data::{get_data_path, load_value_from_path};
///
/// let path = get_data_path("config_nfcv");
/// let value = load_value_from_path(path).expect("should load YAML path");
///
/// assert!(value.is_sequence() || value.is_mapping());
/// ```
pub fn load_value_from_path<P: AsRef<Path>>(path: P) -> crate::Result<Value> {
    let data_string = fs::read_to_string(path.as_ref())
        .map_err(|source| Error::DataLoadError { path: path.as_ref().to_path_buf(), source })?;

    Ok(serde_norway::from_str(&data_string)
        .map_err(|source| Error::YAMLParseError { path: path.as_ref().to_path_buf(), source })?)
}

/// Loads and parses a YAML schema file from the crate’s built-in `data`
/// directory into a generic [`Value`].
///
/// This is a convenience wrapper around [`load_value_from_path`], which
/// automatically resolves the file path relative to the crate’s `data`
/// directory using [`get_data_path`].
///
/// # Type Parameters
/// - `S`: Type of the schema name (typically `str` or [`String`]).
///
/// # Arguments
/// - `name`: Name of the schema to load.
///
/// # Errors
/// Propagates the same errors as [`load_value_from_path`]:
///
/// - [`Error::DataLoadError`]: if the file cannot be read from disk.
/// - [`Error::YAMLParseError`]: if the YAML is invalid or cannot be parsed.
///
/// Both errors include the full absolute path for debugging.
///
/// # Example
/// ```rust
/// use openprinttag_codegen::data::load_value;
///
/// let value = load_value("material_type_enum").expect("should load YAML file");
///
/// assert!(value.is_sequence() || value.is_mapping());
/// ```
///
/// # See also
/// - [`get_data_path`] for path resolution.
/// - [`load_value_from_path`] for loading into untyped [`Value`].
#[inline]
pub fn load_value<S: AsRef<str>>(name: S) -> crate::Result<Value> {
    load_value_from_path(get_data_path(name))
}
/// Loads and deserializes a schema from a YAML file on disk.
///
/// This is a generic helper built on top of [`load_value_from_path`]. It reads
/// a YAML schema file, parses it into a [`Value`], and then deserializes that
/// value into any type `D` that implements [`DeserializeOwned`].
///
/// This function is used as the backbone of all higher-level data-loading
/// utilities, such as [`load_enum_variants_from_path`], to automatically
/// populate Rust structs or enums from YAML schema definitions.
///
/// # Type Parameters
/// - `P`: Type of the path (typically [`Path`][std::path::Path]).
/// - `D`: The target deserializable type implementing [`DeserializeOwned`].
///
/// # Errors
/// - [`Error::DataLoadError`]: if the file cannot be read from disk.
/// - [`Error::YAMLParseError`]: if the YAML is invalid or cannot be parsed.
///
/// Both errors include the full absolute path for debugging.
///
/// # Example
/// ```rust
/// use openprinttag_codegen::data::{get_data_path, load_from_path};
///
/// #[derive(Debug, Clone, serde::Deserialize)]
/// struct MyConfig {
///     #[serde(default)]
///     mime_type: Option<String>,
/// }
///
/// let path = get_data_path("config_nfcv");
/// let config: MyConfig = load_from_path(path).expect("should load and deserialize config_nfcv");
///
/// assert_eq!(config.mime_type, Some("application/vnd.openprinttag".to_string()));
/// ```
///
/// # See also
/// - [`load_value_from_path`] for loading into untyped [`Value`].
pub fn load_from_path<P, D>(path: P) -> crate::Result<D>
where
    P: AsRef<Path>,
    D: DeserializeOwned,
{
    serde_norway::from_value(load_value_from_path(path.as_ref())?)
        .map_err(|source| Error::YAMLParseError { path: path.as_ref().to_path_buf(), source })
}

/// Loads and deserializes a schema from a built-in YAML schema file located in
/// the crate’s `data` directory.
///
/// This is a convenience wrapper around [`load_from_path`]. It resolves the
/// full file path automatically using [`get_data_path`], reads the YAML schema
/// file, then deserializes that value into any type `D` that implements
/// [`DeserializeOwned`].
///
/// # Type Parameters
/// - `S`: Type of the file name (typically `str` or [`String`]).
/// - `D`: The target deserializable type implementing [`DeserializeOwned`].
///
/// # Arguments
/// - `name`: Name of the schema to load.
///
/// # Errors
/// Propagates the same errors as [`load_from_path`]:
///
/// - [`Error::DataLoadError`]: if the file cannot be read from disk.
/// - [`Error::YAMLParseError`]: if the YAML is invalid or cannot be parsed.
///
/// Both errors include the full absolute path for debugging.
///
/// # Example
/// ```rust
/// use openprinttag_codegen::data::load;
///
/// #[derive(Debug, Clone, serde::Deserialize)]
/// struct MyConfig {
///     #[serde(default)]
///     mime_type: Option<String>,
/// }
///
/// let config: MyConfig = load("config_nfcv").expect("should load and deserialize config_nfcv");
///
/// assert_eq!(config.mime_type, Some("application/vnd.openprinttag".to_string()));
/// ```
///
/// # See also
/// - [`get_data_path`] for path resolution.
/// - [`load_from_path`] for loading from arbitrary paths.
pub fn load<S, D>(name: S) -> crate::Result<D>
where
    S: AsRef<str>,
    D: DeserializeOwned,
{
    load_from_path(get_data_path(name))
}

/// Conditionally loads and deserializes a schema from a built-in YAML schema
/// file.
///
/// This helper wraps [`load`], returning an error if no file name is provided.
/// It is useful in cases where certain data files are optional -- for example,
/// when a field in a larger configuration may or may not reference an external
/// YAML schema.
///
/// # Type Parameters
/// - `S`: Type of the optional file name (typically `str` or [`String`]).
/// - `D`: The target deserializable type implementing [`DeserializeOwned`].
///
/// # Arguments
/// - `name`: Name of the schema to load.
///
/// # Returns
/// - `Ok` if a file name is provided and successfully loaded.
/// - `Err` of [`Error::NoDataPath`] — if `name` is [`None`].
/// - Other variants of [`Error`] if the file cannot be read or parsed.
///
/// # Example
/// ```rust
/// use openprinttag_codegen::{data::maybe_load, Error};
///
/// #[derive(Debug, Clone, serde::Deserialize)]
/// struct MyConfig {
///     #[serde(default)]
///     mime_type: Option<String>,
/// }
///
/// let config: MyConfig =
///     maybe_load(Some("config_nfcv")).expect("should load and deserialize config_nfcv");
///
/// let missing: Result<MyConfig, _> = maybe_load::<&str, _>(None);
/// assert!(matches!(missing, Err(Error::NoDataPath)));
/// ```
///
/// # See also
/// - [`load`] for unconditional loading.
/// - [`get_data_path`] for path resolution.
pub fn maybe_load<S, D>(name: Option<S>) -> crate::Result<D>
where
    S: AsRef<str>,
    D: DeserializeOwned,
{
    match name {
        Some(name) => load(name),
        None => Err(Error::NoDataPath),
    }
}

/// Loads, renames, and deserializes a schema from a YAML schema file on disk.
///
/// This function extends [`load_from_path`] by first applying key remapping to
/// the parsed YAML data before deserializing it into type `D`. It is useful for
/// cases where field names in the external YAML files _differ_ from the
/// expected Rust struct field names, or when aliases must be normalized for
/// compatibility.
///
/// Internally, it:
/// 1. Reads and parses the YAML file into a [`Value`].
/// 2. Applies the provided [`de::NameMap`] (or any iterable of key-value pairs)
///    to rename top-level and nested keys using [`de::rename_fields`].
/// 3. Deserializes the transformed value into the specified type `D`.
///
/// # Type Parameters
/// - `P`: Type of the file path (typically [`Path`]).
/// - `NM`: A mapping type implementing `IntoIterator<Item = (&N, &O)>`.
/// - `N`: Type of each *new* key in the mapping (usually `&str` or [`String`]).
/// - `O`: Type of each *old* key in the mapping (usually `&str` or [`String`]).
/// - `D`: The target deserializable type implementing [`DeserializeOwned`].
///
/// # Arguments
/// - `path`: Path to the YAML schema file to be loaded.
/// - `name_map`: A key-renaming map, typically a [`NameMap`] or
///   [`BTreeMap<String, String>`][BTreeMap], where each `(new_name, old_name)`
///   pair defines a field rename rule.
///
/// # Errors
/// Propagates the same errors as [`load_value_from_path`]:
///
/// - [`Error::DataLoadError`]: if the file cannot be read from disk.
/// - [`Error::YAMLParseError`]: if the YAML is invalid or cannot be parsed.
///
/// Both errors include the full absolute path for debugging.
///
/// # Example
/// ```rust
/// use openprinttag_codegen::{
///     data::{get_data_path, load_mapped_from_path},
///     de,
/// };
///
/// #[derive(Debug, Clone, serde::Deserialize)]
/// struct MyConfig {
///     #[serde(default)]
///     real_mime_type: Option<String>,
/// }
///
/// let mut name_map = de::NameMap::new();
/// name_map.insert("real_mime_type".into(), "mime_type".into());
///
/// let path = get_data_path("config_nfcv");
/// let config: MyConfig = load_mapped_from_path(path, &name_map)
///     .expect("should load and rename fields before deserialization");
///
/// assert_eq!(config.real_mime_type, Some("application/vnd.openprinttag".to_string()))
/// ```
///
/// # See also
/// - [`load_from_path`] for simple typed loading.
/// - [`de::rename_fields`] for renaming logic.
/// - [`de::NameMap`] for key mapping type.
pub fn load_mapped_from_path<P, NM, N, O, D>(path: P, name_map: &NM) -> crate::Result<D>
where
    P: AsRef<Path>,
    for<'a> &'a NM: IntoIterator<Item = (&'a N, &'a O)>,
    N: AsRef<str>,
    O: AsRef<str>,
    D: DeserializeOwned,
{
    let mut value = load_value_from_path(path.as_ref())?;
    de::rename_fields(&mut value, name_map);
    serde_norway::from_value(value)
        .map_err(|source| Error::YAMLParseError { path: path.as_ref().to_path_buf(), source })
}

/// Loads, renames, and deserializes a schema from a built-in YAML schema file
/// located in the crate’s `data` directory.
///
/// This is a convenience wrapper around [`load_mapped_from_path`].
///
/// # Type Parameters
/// - `P`: Type of the file path (typically [`Path`]).
/// - `NM`: A mapping type implementing `IntoIterator<Item = (&N, &O)>`.
/// - `N`: Type of each *new* key in the mapping (usually `&str` or [`String`]).
/// - `O`: Type of each *old* key in the mapping (usually `&str` or [`String`]).
/// - `D`: The target deserializable type implementing [`DeserializeOwned`].
///
/// # Arguments
/// - `path`: Path to the YAML schema file to be loaded.
/// - `name_map`: A key-renaming map, typically a [`NameMap`] or
///   [`BTreeMap<String, String>`][BTreeMap], where each `(new_name, old_name)`
///   pair defines a field rename rule.
///
/// # Errors
/// Propagates the same errors as [`load_mapped_from_path`]:
///
/// - [`Error::DataLoadError`]: if the file cannot be read from disk.
/// - [`Error::YAMLParseError`]: if the YAML is invalid or cannot be parsed.
///
/// Both errors include the full absolute path for debugging.
///
/// # Example
/// ```rust
/// use openprinttag_codegen::{
///     data::{get_data_path, load_mapped},
///     de,
/// };
///
/// #[derive(Debug, Clone, serde::Deserialize)]
/// struct MyConfig {
///     #[serde(default)]
///     real_mime_type: Option<String>,
/// }
///
/// let mut name_map = de::NameMap::new();
/// name_map.insert("real_mime_type".into(), "mime_type".into());
///
/// let config: MyConfig = load_mapped("config_nfcv", &name_map)
///     .expect("should load and rename fields before deserialization");
///
/// assert_eq!(config.real_mime_type, Some("application/vnd.openprinttag".to_string()))
/// ```
///
/// # See also
/// - [`load_mapped_from_path`] for loading from arbitrary paths.
/// - [`de::rename_fields`] for renaming logic.
/// - [`de::NameMap`] for key mapping type.
/// - [`get_data_path`] for path resolution.
#[inline]
pub fn load_mapped<S, NM, N, O, D>(name: S, name_map: &NM) -> crate::Result<D>
where
    S: AsRef<str>,
    for<'a> &'a NM: IntoIterator<Item = (&'a N, &'a O)>,
    N: AsRef<str>,
    O: AsRef<str>,
    D: DeserializeOwned,
{
    load_mapped_from_path(get_data_path(name), name_map)
}

pub fn load_enum_variants_from_path<P, NF, DNF>(
    path: P,
    name_field: Option<NF>,
    display_name_field: Option<DNF>,
) -> crate::Result<EnumVariants>
where
    P: AsRef<Path>,
    NF: AsRef<str>,
    DNF: AsRef<str>,
{
    let mut name_map = de::NameMap::new();

    if let Some(name_field) = name_field {
        name_map.insert("name".to_string(), name_field.as_ref().to_string());
    }

    if let Some(display_name_field) = display_name_field {
        name_map.insert("display_name".to_string(), display_name_field.as_ref().to_string());
    }

    if name_map.is_empty() {
        load_from_path(path)
    } else {
        load_mapped_from_path(path, &name_map)
    }
}

pub fn load_enum_variants<S, NF, DNF>(
    name: S,
    name_field: Option<NF>,
    display_name_field: Option<DNF>,
) -> crate::Result<EnumVariants>
where
    S: AsRef<str>,
    NF: AsRef<str>,
    DNF: AsRef<str>,
{
    load_enum_variants_from_path(get_data_path(name), name_field, display_name_field)
}
