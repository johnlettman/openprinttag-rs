use crate::{de, EnumVariants, Error};
use serde::de::DeserializeOwned;
use serde_norway::Value;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Constructs an absolute path to a schema file located under the crate’s `data` directory.
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
/// - `N`: Type of the schema name (typically `str` or [`String`]).
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
pub fn get_data_path<N: AsRef<str>>(name: N) -> PathBuf {
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

/// Loads and parses a YAML schema file from the crate’s built-in `data` directory
/// into a generic [`Value`].
///
/// This is a convenience wrapper around [`load_value_from_path`], which
/// automatically resolves the file path relative to the crate’s `data`
/// directory using [`get_data_path`].
///
/// # Type Parameters
/// - `N`: Type of the schema name (typically `str` or [`String`]).
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
/// - [`get_data_path`] for path resolution
/// - [`load_value_from_path`] for loading from arbitrary locations
#[inline]
pub fn load_value<N: AsRef<str>>(name: N) -> crate::Result<Value> {
    load_value_from_path(get_data_path(name))
}

pub fn load_from_path<P, D>(path: P) -> crate::Result<D>
where
    P: AsRef<Path>,
    D: DeserializeOwned,
{
    serde_norway::from_value(load_value_from_path(path.as_ref())?).map_err(|source| {
        Error::YAMLParseError { path: path.as_ref().to_path_buf(), source }
    })
}

pub fn load<S, D>(name: S) -> crate::Result<D>
where
    S: AsRef<str>,
    D: DeserializeOwned,
{
    load_from_path(get_data_path(name))
}

pub fn maybe_load<S, D>(name: Option<S>) -> crate::Result<D>
where
    S: AsRef<str>,
    D: DeserializeOwned,
{
    if let Some(name) = name {
        return load(name);
    }

    Err(crate::Error::NoDataPath)
}

pub fn load_mapped_from_path<P, M, K, V, D>(path: P, name_map: &M) -> crate::Result<D>
where
    P: AsRef<Path>,
    for<'a> &'a M: IntoIterator<Item = (&'a K, &'a V)>,
    K: AsRef<str>,
    V: AsRef<str>,
    D: DeserializeOwned,
{
    let mut value = load_value_from_path(path.as_ref())?;
    de::rename_fields(&mut value, name_map);
    serde_norway::from_value(value).map_err(|source| crate::Error::YAMLParseError {
        path: path.as_ref().to_path_buf(),
        source,
    })
}

pub fn load_mapped<S, M, K, V, D>(name: S, name_map: &M) -> crate::Result<D>
where
    S: AsRef<str>,
    for<'a> &'a M: IntoIterator<Item = (&'a K, &'a V)>,
    K: AsRef<str>,
    V: AsRef<str>,
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

pub fn load_enum_variants<I, NF, DNF>(
    name: I,
    name_field: Option<NF>,
    display_name_field: Option<DNF>,
) -> crate::Result<EnumVariants>
where
    I: AsRef<str>,
    NF: AsRef<str>,
    DNF: AsRef<str>,
{
    load_enum_variants_from_path(get_data_path(name), name_field, display_name_field)
}
