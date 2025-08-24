use base64::{Engine as _, engine::general_purpose::STANDARD};
use std::{borrow::Cow, fmt::Debug};

use reqwest::header::HeaderValue;
use url::Url;

use crate::settings::Server;

pub trait ResolvableRepository: Debug {
    /// The repository ID
    fn id(&self) -> Option<&str> {
        None
    }
    /// The base URL of the repository
    fn base_url(&self) -> &str;

    // TODO: Add Authentication Logic

    fn has_authentication(&self) -> bool {
        false
    }

    fn authentication_header(&self) -> Option<HeaderValue> {
        None
    }
    fn create_url_with_path(&self, path: &str) -> Result<Url, url::ParseError> {
        let base = self.base_url();
        if base.ends_with('/') {
            Url::parse(&format!("{}{}", base, path))
        } else {
            Url::parse(&format!("{}/{}", base, path))
        }
    }
}
impl<T> ResolvableRepository for &T
where
    T: ResolvableRepository,
{
    fn base_url(&self) -> &str {
        (*self).base_url()
    }
    fn authentication_header(&self) -> Option<HeaderValue> {
        (*self).authentication_header()
    }
    fn has_authentication(&self) -> bool {
        (*self).has_authentication()
    }
    fn id(&self) -> Option<&str> {
        (*self).id()
    }
    fn create_url_with_path(&self, path: &str) -> Result<Url, url::ParseError> {
        (*self).create_url_with_path(path)
    }
}
impl<T: ResolvableRepository + Clone> ResolvableRepository for Cow<'_, T> {
    fn base_url(&self) -> &str {
        self.as_ref().base_url()
    }
    fn authentication_header(&self) -> Option<HeaderValue> {
        self.as_ref().authentication_header()
    }
    fn has_authentication(&self) -> bool {
        self.as_ref().has_authentication()
    }
    fn id(&self) -> Option<&str> {
        self.as_ref().id()
    }
    fn create_url_with_path(&self, path: &str) -> Result<Url, url::ParseError> {
        self.as_ref().create_url_with_path(path)
    }
}

impl ResolvableRepository for &str {
    fn base_url(&self) -> &str {
        self
    }
}

impl ResolvableRepository for str {
    fn base_url(&self) -> &str {
        self
    }
}

impl ResolvableRepository for String {
    fn base_url(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Default)]
pub struct FullMavenRepository {
    pub id: Option<String>,
    pub url: String,
    pub authentication: Option<HeaderValue>,
}
impl FullMavenRepository {
    pub fn new_with_config_server(url: String, server: Server) -> Self {
        let authentication = if let Some((username, password)) = server.username_and_password() {
            Some(Self::basic_authentication_header(username, password))
        } else {
            None
        };
        Self {
            id: Some(server.id),
            url,
            authentication,
        }
    }
    fn basic_authentication_header(username: &str, password: &str) -> HeaderValue {
        let auth = format!("{}:{}", username, password);
        let encoded = STANDARD.encode(auth.as_bytes());
        let header_value = format!("Basic {}", encoded);
        HeaderValue::from_str(&header_value).unwrap()
    }
}
impl ResolvableRepository for FullMavenRepository {
    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }
    fn base_url(&self) -> &str {
        self.url.as_str()
    }
    fn has_authentication(&self) -> bool {
        self.authentication.is_some()
    }
    fn authentication_header(&self) -> Option<HeaderValue> {
        self.authentication.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::ResolvableRepository;

    fn test_url_create(repo: impl ResolvableRepository, path: &str, expected_url: &str) {
        let url = repo.create_url_with_path(path).unwrap();
        assert_eq!(url.as_str(), expected_url);
        println!("URL: {:?}", url);
    }
    #[test]
    pub fn test_resolable_repository() {
        test_url_create(
            "https://repo1.maven.org/maven2/",
            "com/google/code/gson/gson/2.11.0/",
            "https://repo1.maven.org/maven2/com/google/code/gson/gson/2.11.0/",
        );
    }
}
