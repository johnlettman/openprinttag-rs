use crate::loader::{GitHubLoader, Loader, LoaderError, LoaderResult};
use std::io::Read;
use crate::schema::name;

#[derive(Debug, Clone)]
pub struct URLLoader(String);

impl URLLoader {
    pub fn new(base: impl Into<String>) -> Self {
        let mut base = base.into();
        if !base.ends_with('/') {
            base.push('/');
        }

        Self(base)
    }

    #[inline]
    pub fn url_for(&self, schema_name: &str) -> String {
        format!("{}{}", self.0, name::normalize(schema_name))
    }
}

impl Default for URLLoader {
    #[inline(always)]
    fn default() -> Self {
        Self::new(GitHubLoader::ROOT)
    }
}

impl Loader for URLLoader {
    fn load_string(&self, schema_name: &str) -> LoaderResult<String> {
        let url = self.url_for(schema_name);
        let response = ureq::get(&url)
            .call()
            .map_err(|source| LoaderError::URLIOError { url: url.clone(), source })?;
        let text = response
            .into_body()
            .read_to_string()
            .map_err(|source| LoaderError::URLParseError { url: url.clone(), source })?;
        Ok(text)
    }

    fn read(&self, schema_name: &str) -> LoaderResult<Box<dyn Read>> {
        let url = self.url_for(schema_name);
        let response = ureq::get(&url)
            .call()
            .map_err(|source| LoaderError::URLIOError { url: url.clone(), source })?;
        let reader = response.into_body().into_reader();
        Ok(Box::new(reader))
    }
}
