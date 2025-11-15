mod crate_dir_loader;
mod dir_loader;
mod error;

pub use crate_dir_loader::*;
pub use dir_loader::*;
pub use error::*;
use serde::de::{DeserializeOwned, DeserializeSeed};
use serde_norway::{from_str, Value};
use std::fmt::Debug;

#[cfg(feature = "url-loader")]
mod url_loader;

#[cfg(feature = "url-loader")]
mod github_loader;

#[cfg(feature = "url-loader")]
pub use url_loader::*;

use crate::schema::name;
#[cfg(feature = "url-loader")]
pub use github_loader::*;

pub type LoaderResult<T> = Result<T, LoaderError>;

pub trait Loader: Debug {
    fn load_string(&self, schema_name: &str) -> LoaderResult<String>;

    fn read(&self, schema_name: &str) -> LoaderResult<Box<dyn std::io::Read>> {
        let str = self.load_string(schema_name)?;
        Ok(Box::new(std::io::Cursor::new(str)))
    }

    #[inline]
    fn load_value(&self, schema_name: &str) -> LoaderResult<Value> {
        let schema_name = name::normalize(schema_name);
        Ok(from_str(self.load_string(&schema_name)?.as_str())?)
    }
}

pub trait LoaderExt: Loader {
    #[inline]
    fn load<D: DeserializeOwned>(&self, schema_name: &str) -> LoaderResult<D> {
        Ok(from_str(self.load_string(schema_name)?.as_str())?)
    }
}

pub trait LoaderSeedExt: Loader {
    fn seed<'de, S>(&self, schema_name: &str, seed: S) -> LoaderResult<S::Value>
    where
        S: DeserializeSeed<'de>,
        S::Value: ToOwned<Owned = S::Value>,
    {
        let schema_name = name::normalize(schema_name);
        let obj = {
            let reader = self.read(&schema_name)?;
            let deserializer = serde_norway::Deserializer::from_reader(reader);
            seed.deserialize(deserializer)?
        };

        Ok(obj)
    }
}

impl<T: Loader + ?Sized> LoaderExt for T {}
impl<T: Loader + ?Sized> LoaderSeedExt for T {}
