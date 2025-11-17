use crate::{
    loader::{GitHubLoader, Loader, LoaderError, LoaderResult},
    schema::name,
    tracing::debug,
};
use std::fmt;
use ureq::Agent;

#[derive(Clone)]
pub struct URLLoader {
    base: String,
    agent: Agent,
}

impl URLLoader {
    pub const USER_AGENT: &'static str =
        const_format::concatcp!("openprinttag-codegen/", crate::VERSION);

    /// Creates a new URL loader rooted at the given base URL.
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", ret, fields(?base = base.as_ref()), skip(base)))]
    pub fn new<B: AsRef<str>>(base: B) -> Self {
        let mut base = base.as_ref().to_string();
        if !base.ends_with('/') {
            base.push('/');
        }

        let agent = Self::new_agent();
        Self { base, agent }
    }

    pub(crate) fn new_agent() -> Agent {
        let config = Agent::config_builder().user_agent(Self::USER_AGENT).build();
        Agent::new_with_config(config)
    }

    /// Builds the full URL to a schema within the base URL.
    #[inline]
    pub fn url_for(&self, schema_name: &str) -> String {
        format!("{}{}", self.base, name::normalize(schema_name))
    }
}

impl Default for URLLoader {
    /// Creates a new URL loader rooted at the GitHub raw content URL.
    #[inline(always)]
    fn default() -> Self {
        Self::new(GitHubLoader::ROOT)
    }
}

impl fmt::Debug for URLLoader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("UrlLoader").field(&self.base).finish()
    }
}

impl Loader for URLLoader {
    /// Loads the schema as a string from the constructed URL.
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "debug", ret))]
    fn load_string(&self, schema_name: &str) -> LoaderResult<String> {
        let url = self.url_for(schema_name);

        let response = self
            .agent
            .get(&url)
            .call()
            .map_err(|source| LoaderError::URLIOError { url: url.clone(), source })?;
        debug!("Received response: {response:#?}");

        let text = response
            .into_body()
            .read_to_string()
            .map_err(|source| LoaderError::URLParseError { url: url.clone(), source })?;

        Ok(text)
    }

    /// Loads the schema as a reader from the constructed URL.
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "debug"))]
    fn read(&self, schema_name: &str) -> LoaderResult<Box<dyn std::io::Read>> {
        let url = self.url_for(schema_name);

        let response = self
            .agent
            .get(&url)
            .call()
            .map_err(|source| LoaderError::URLIOError { url: url.clone(), source })?;
        debug!("Received response status: {}", response.status());

        Ok(Box::new(response.into_body().into_reader()))
    }
}
