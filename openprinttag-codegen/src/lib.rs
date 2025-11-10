mod config;
mod enum_variant;
mod field;
mod field_type;
pub mod name;
mod required;
pub mod de;
pub mod ser;
pub mod data;
mod error;
mod gen;

use std::path::{Path, PathBuf};
pub use config::*;
pub use enum_variant::*;
pub use field::*;
pub use field_type::*;
pub use required::*;
pub use error::*;




pub type Result<T> = std::result::Result<T, Error>;