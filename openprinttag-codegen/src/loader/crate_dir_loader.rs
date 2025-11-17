use crate::loader::{dir_loader::DirLoader, Loader, LoaderResult};
use serde_norway::Value;
use std::{fmt, ops::Deref, path::Path};

#[derive(Clone)]
pub struct CrateDirLoader(DirLoader);

impl CrateDirLoader {
    /// Creates a new crate loader rooted at the given project directory.
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "debug", ret, fields(?dir = dir.as_ref()), skip(dir)))]
    pub fn new<P: AsRef<Path>>(dir: P) -> Self {
        let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join(dir.as_ref());
        Self(DirLoader::new(crate_dir))
    }
}

impl Default for CrateDirLoader {
    /// Creates a new crate loader rooted at the default "data" directory.
    #[inline(always)]
    fn default() -> Self {
        Self::new("data")
    }
}

impl<P> From<P> for CrateDirLoader
where
    P: AsRef<Path>,
{
    /// Creates a new crate loader rooted at the given project directory.
    #[inline(always)]
    fn from(path: P) -> Self {
        Self::new(path)
    }
}

impl Deref for CrateDirLoader {
    type Target = DirLoader;

    /// Dereferences to the underlying directory loader.
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for CrateDirLoader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CrateDirLoader").field(self.0.deref()).finish()
    }
}

impl Loader for CrateDirLoader {
    #[inline(always)]
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "debug"))]
    fn load_string(&self, schema: &str) -> LoaderResult<String> {
        self.0.load_string(schema)
    }

    #[inline(always)]
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "debug"))]
    fn load_value(&self, schema: &str) -> LoaderResult<Value> {
        self.0.load_value(schema)
    }
}
