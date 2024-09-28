use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::editor::{
    utils::{add_if_present, from_element_using_builder, sync_element},
    ElementConverter, HasElementName, UpdatableElement,
};
#[derive(Debug, Clone, Copy, Error)]
pub enum SCMError {
    #[error("The scm did not start with scm")]
    DidNotStartWithScm,
    #[error("The scm did not have a provider")]
    MissingProvider,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Scm {
    /// URL should be formatted as `scm:{provider}:{provider_specific}`
    #[builder(setter(into, strip_option), default)]
    pub url: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub connection: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub tag: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub developer_connection: Option<String>,
}
impl Scm {
    /// Gets the provider of the scm
    /// ```rust
    /// use maven_rs::pom::ScmBuilder;
    /// let scm = ScmBuilder::default().connection("scm:git:https://github.com/wyatt-herkamp/maven-rs").build().unwrap();
    /// assert_eq!(scm.get_provider_for_connection().unwrap(), Some("git".to_owned()));
    /// ```
    pub fn get_provider_for_connection(&self) -> Result<Option<String>, SCMError> {
        let Some((_, provider, _)) = self.split_connection()? else {
            return Ok(None);
        };
        Ok(Some(provider.to_owned()))
    }
    /// Gets the provider specific part of the scm
    ///
    /// ```rust
    /// use maven_rs::pom::ScmBuilder;
    /// let scm = ScmBuilder::default().connection("scm:git:https://github.com/wyatt-herkamp/maven-rs").build().unwrap();
    ///
    /// assert_eq!(scm.get_provider_specific_for_connection().unwrap(), Some("https://github.com/wyatt-herkamp/maven-rs".to_owned()));
    ///
    /// ```
    pub fn get_provider_specific_for_connection(&self) -> Result<Option<String>, SCMError> {
        let Some((_, _, url)) = self.split_connection()? else {
            return Ok(None);
        };
        Ok(Some(url.join(":")))
    }
    fn split_connection(&self) -> Result<Option<(&str, &str, Vec<&str>)>, SCMError> {
        let Some(url) = self.connection.as_deref() else {
            return Err(SCMError::MissingProvider);
        };
        let mut parts = url.split(':');
        let part_one = parts.next().ok_or(SCMError::DidNotStartWithScm)?;
        if part_one != "scm" {
            return Err(SCMError::DidNotStartWithScm);
        }
        let part_two = parts.next().ok_or(SCMError::MissingProvider)?;
        let part_three = parts.collect::<Vec<&str>>();
        Ok(Some((part_one, part_two, part_three)))
    }
}

impl UpdatableElement for Scm {
    fn update_element(
        &self,
        element: edit_xml::Element,
        document: &mut edit_xml::Document,
    ) -> Result<(), crate::editor::XMLEditorError> {
        let Self {
            url,
            connection,
            tag,
            developer_connection,
        } = self;
        sync_element(document, element, "url", url.as_deref());
        sync_element(document, element, "connection", connection.as_deref());
        sync_element(document, element, "tag", tag.as_deref());
        sync_element(
            document,
            element,
            "developerConnection",
            developer_connection.as_deref(),
        );
        Ok(())
    }
}
impl HasElementName for Scm {
    fn element_name() -> &'static str {
        "scm"
    }
}
impl ElementConverter for Scm {
    from_element_using_builder!(
        ScmBuilder,
        element,
        document,
        "url" => url,
        "connection" => connection,
        "tag" => tag,
        "developerConnection" => developer_connection
    );

    fn into_children(
        self,
        document: &mut edit_xml::Document,
    ) -> Result<Vec<edit_xml::Element>, crate::editor::XMLEditorError> {
        let Self {
            url,
            connection,
            tag,
            developer_connection,
        } = self;
        let mut children = vec![];
        add_if_present!(document, children, url, "url");
        add_if_present!(document, children, connection, "connection");
        add_if_present!(document, children, tag, "tag");
        add_if_present!(
            document,
            children,
            developer_connection,
            "developerConnection"
        );
        Ok(children)
    }
}
