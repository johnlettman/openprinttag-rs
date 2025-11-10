#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no data path provided")]
    NoDataPath,

    #[error("failed to load data file from {path}: {source}")]
    DataLoadError {
        path: std::path::PathBuf,

        #[source]
        source: std::io::Error,
    },

    #[error("failed to parse data YAML from {path}: {source}")]
    YAMLParseError {
        path: std::path::PathBuf,

        #[source]
        source: serde_norway::Error,
    },

    #[error("failed to generate code: {0}")]
    GenError(#[from] syn::Error),

    #[error("failed to generate type: {0}")]
    GenTypeError(#[source] syn::Error),
}
