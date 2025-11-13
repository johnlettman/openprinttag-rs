#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no such schema: {0}")]
    NoSchema(String),

    #[error("error generating Rust tokens: {0}")]
    SynError(#[from] syn::Error),

    #[error("failed to load schema: {0}")]
    LoaderError(#[from] crate::loader::LoaderError),

    #[error("schema deserialization error: {0}")]
    DeserializeError(String),
}
