use crate::loader::{Loader, LoaderResult, URLLoader};
use std::fmt;

/// Loader for interacting with the official GitHub-hosted [OpenPrintTag]
/// schemas.
///
/// [`GitHubLoader`] is a thin wrapper around the [`URLLoader`] which allows
/// for loading data from GitHub endpoints.
///
/// # Example
/// ```rust
/// use openprinttag_codegen::loader::GitHubLoader;
/// use openprinttag_codegen::schema::DataEnumVariants;
///
/// // use loader extensions for convenience methods
/// use openprinttag_codegen::loader::LoaderExt;
///
/// // construct the loader
/// let loader = GitHubLoader::new();
///
/// // get the URL for a schema
/// let schema = "material_class_enum";
/// let url = loader.url_for(schema);
///
/// // the loader will automatically prepend the GitHub raw content URL and
/// // normalize the name of the schema
/// assert_eq!(url, "https://raw.githubusercontent.com/prusa3d/OpenPrintTag/refs/heads/main/data/material_class_enum.yaml");
///
/// // load schemas
/// let variants: DataEnumVariants = loader.load(schema).expect("should load schema from GitHub");
/// assert!(variants.len() > 0);
/// ```
#[derive(Clone)]
pub struct GitHubLoader(URLLoader);

impl GitHubLoader {
    /// The root of the official OpenPrintTag schemas on GitHub.
    pub const ROOT: &'static str =
        "https://raw.githubusercontent.com/prusa3d/OpenPrintTag/refs/heads/main/data";

    #[inline(always)]
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", ret))]
    pub fn new() -> Self {
        Self(URLLoader::new(Self::ROOT))
    }

    #[inline(always)]
    pub fn url_for(&self, schema_name: &str) -> String {
        self.0.url_for(schema_name)
    }
}

impl fmt::Debug for GitHubLoader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GitHubLoader").finish()
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
    fn read(&self, schema_name: &str) -> LoaderResult<Box<dyn std::io::Read>> {
        self.0.read(schema_name)
    }
}
