pub fn schema_link<N: AsRef<str>>(schema_name: N) -> String {
    let name = schema_name.as_ref();

    #[cfg(not(feature = "terminal-links"))]
    return name.to_string();

    #[cfg(feature = "terminal-links")]
    {
        if supports_hyperlinks::on(supports_hyperlinks::Stream::Stdout) {
            let url = crate::loader::GitHubLoader::new().url_for(name);
            let link = terminal_link::Link::new(name, url.as_str());
            return link.to_string();
        } else {
            return name.to_string();
        }
    }
}
