pub mod de;
mod error;
pub mod loader;
mod required;
pub mod schema;
pub mod ser;

pub use error::*;
pub use required::*;

pub type Result<T> = std::result::Result<T, Error>;
