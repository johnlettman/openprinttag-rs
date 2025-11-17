pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GIT_VERSION: &str = git_version::git_version!(fallback = "unknown");
pub const VERSION: &str = const_format::concatcp!(CRATE_VERSION, "-", GIT_VERSION);
