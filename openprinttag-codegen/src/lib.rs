mod config;
pub mod data;
pub mod de;
mod enum_variant;
mod error;
mod field;
mod field_type;
mod gen;
pub mod name;
mod required;
pub mod ser;

pub use config::*;
pub use enum_variant::*;
pub use error::*;
pub use field::*;
pub use field_type::*;
pub use required::*;
use std::path::{Path, PathBuf};

pub type Result<T> = std::result::Result<T, Error>;
