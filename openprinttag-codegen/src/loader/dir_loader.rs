use crate::{
    loader::{Loader, LoaderError, LoaderResult},
    schema::name,
    tracing::debug,
};
use serde_norway::{from_str, Value};
use std::{
    fs::read_to_string,
    ops::Deref,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct DirLoader(PathBuf);

impl DirLoader {
    /// Creates a new directory loader rooted at the given directory.
    #[inline]
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "debug", ret, fields(?dir = dir.as_ref()), skip(dir)))]
    pub fn new<P: AsRef<Path>>(dir: P) -> Self {
        Self(dir.as_ref().to_path_buf())
    }

    /// Builds the full path to a schema within the directory.
    #[inline]
    pub fn path_for<N: AsRef<str>>(&self, schema: N) -> PathBuf {
        self.0.join(name::normalize(schema.as_ref()))
    }
}

impl<P> From<P> for DirLoader
where
    P: AsRef<Path>,
{
    /// Creates a new directory loader rooted at the given directory.
    #[inline(always)]
    fn from(path: P) -> Self {
        Self::new(path)
    }
}

impl Deref for DirLoader {
    type Target = PathBuf;

    /// Dereferences to the root directory path.
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Loader for DirLoader {
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "debug", ret))]
    fn load_string(&self, schema: &str) -> LoaderResult<String> {
        let path = self.path_for(schema);
        debug!("Resolved path: {path:?}");
        read_to_string(&path).map_err(|source| LoaderError::FileIOError { path, source })
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(level = "debug", ret))]
    fn load_value(&self, schema: &str) -> LoaderResult<Value> {
        from_str(self.load_string(schema.as_ref())?.as_str())
            .map_err(|source| LoaderError::FileParseError { path: self.path_for(schema), source })
    }
}
