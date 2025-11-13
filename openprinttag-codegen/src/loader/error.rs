#[derive(Debug, thiserror::Error)]
pub enum LoaderError {
    #[error("failed to load schema: {0}")]
    IOError(#[from] std::io::Error),

    #[error("failed to load schema from file {path}: {source}")]
    FileIOError {
        path: std::path::PathBuf,

        #[source]
        source: std::io::Error,
    },

    #[cfg(feature = "url-loader")]
    #[error("failed to load schema from URL {url}: {source}")]
    URLIOError {
        url: String,

        #[source]
        source: ureq::Error,
    },

    #[error("failed to parse schema YAML: {0}")]
    ParseError(#[from] serde_norway::Error),

    #[error("failed to parse schema YAML from file {path}: {source}")]
    FileParseError {
        path: std::path::PathBuf,

        #[source]
        source: serde_norway::Error,
    },

    #[cfg(feature = "url-loader")]
    #[error("failed to parse schema from URL {url}: {source}")]
    URLParseError {
        url: String,

        #[source]
        source: ureq::Error,
    },

    #[error("{0}")]
    Custom(String),
}

impl LoaderError {
    #[inline]
    pub fn custom<E: std::fmt::Display>(e: E) -> Self {
        Self::Custom(e.to_string())
    }
}
