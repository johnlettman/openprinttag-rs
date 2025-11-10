use crate::{de, EnumVariants};
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub type NameMap = BTreeMap<String, String>;

pub fn get_data_path<S: AsRef<str>>(name: S) -> PathBuf {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let file_name = name.as_ref();
    let file_name =
        if file_name.ends_with(".yaml") { file_name } else { &*format!("{}.yaml", file_name) };

    crate_dir.join("data").join(file_name)
}

pub fn load_value_from_path<P: AsRef<Path>>(path: P) -> crate::Result<serde_norway::Value> {
    let data_string = fs::read_to_string(path.as_ref()).map_err(|source| {
        crate::Error::DataLoadError { path: path.as_ref().to_path_buf(), source }
    })?;

    Ok(serde_norway::from_str(&data_string).map_err(|source| crate::Error::YAMLParseError {
        path: path.as_ref().to_path_buf(),
        source,
    })?)
}

pub fn load_value<S: AsRef<str>>(name: S) -> crate::Result<serde_norway::Value> {
    load_value_from_path(get_data_path(name))
}

pub fn load_from_path<P, D>(path: P) -> crate::Result<D>
where
    P: AsRef<Path>,
    D: DeserializeOwned,
{
    serde_norway::from_value(load_value_from_path(path.as_ref())?).map_err(|source| {
        crate::Error::YAMLParseError { path: path.as_ref().to_path_buf(), source }
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

pub fn load_enum_variants_from_path<P, N>(
    path: P,
    name_field: Option<N>,
) -> crate::Result<EnumVariants>
where
    P: AsRef<Path>,
    N: AsRef<str>,
{
    if let Some(name_field) = name_field {
        let mut name_map = BTreeMap::new();
        name_map.insert("name", name_field.as_ref());

        // infer full name field
        // https://github.com/prusa3d/OpenPrintTag/issues/78
        name_map.insert("full_name", "name");

        load_mapped_from_path(path, &name_map)
    } else {
        load_from_path(path)
    }
}

pub fn load_enum_variants<I, N>(name: I, name_field: Option<N>) -> crate::Result<EnumVariants>
where
    I: AsRef<str>,
    N: AsRef<str>,
{
    load_enum_variants_from_path(get_data_path(name), name_field)
}
