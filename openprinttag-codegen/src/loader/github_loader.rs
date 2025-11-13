use crate::loader::{Loader, LoaderResult, URLLoader};
use std::io::Read;

#[derive(Debug, Clone)]
pub struct GitHubLoader(URLLoader);

impl GitHubLoader {
    pub const ROOT: &'static str =
        "https://raw.githubusercontent.com/prusa3d/OpenPrintTag/refs/heads/main/data";

    #[inline(always)]
    pub fn new() -> Self {
        Self(URLLoader::new(Self::ROOT))
    }
}

impl Default for GitHubLoader {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl Loader for GitHubLoader {
    #[inline(always)]
    fn load_string(&self, schema_name: &str) -> LoaderResult<String> {
        self.0.load_string(schema_name)
    }

    #[inline(always)]
    fn read(&self, schema_name: &str) -> LoaderResult<Box<dyn Read>> {
        self.0.read(schema_name)
    }
}
