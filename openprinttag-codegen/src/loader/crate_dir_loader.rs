use crate::loader::{dir_loader::DirLoader, Loader, LoaderExt, LoaderResult};
use serde_norway::Value;
use std::{
    fmt::{Debug, Formatter},
    path::Path,
};

#[derive(Debug, Clone)]
pub struct CrateDirLoader(DirLoader);

impl CrateDirLoader {
    /// Creates a new crate loader rooted at the given project directory.
    pub fn new<P: AsRef<Path>>(dir: P) -> Self {
        let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join(dir.as_ref());
        Self(DirLoader::new(crate_dir))
    }
}

impl Default for CrateDirLoader {
    #[inline(always)]
    fn default() -> Self {
        Self::new("data")
    }
}

impl Loader for CrateDirLoader {
    #[inline(always)]
    fn load_string(&self, schema: &str) -> LoaderResult<String> {
        self.0.load_string(schema)
    }

    #[inline(always)]
    fn load_value(&self, schema: &str) -> LoaderResult<Value> {
        self.0.load_value(schema)
    }
}
