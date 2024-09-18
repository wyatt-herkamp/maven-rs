use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    editor::{
        utils::{create_basic_text_element, find_element_or_err},
        ChildOfListElement, ElementConverter, HasElementName, PomValue, UpdatableElement,
        XMLEditorError,
    },
    types::StringOrVariable,
};

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder)]
pub struct Build {
    #[serde(rename = "sourceDirectory")]
    pub source_directory: Option<String>,
    #[serde(default)]
    pub plugins: Plugins,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Plugins {
    #[serde(default, rename = "plugin")]
    pub plugins: Vec<Plugin>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Builder)]
pub struct Plugin {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub version: StringOrVariable,
    // TODO Add configuration
}
impl Plugin {
    /// Checks if the plugin is the same as the other plugin.
    ///
    /// Basically, it checks if the group id and artifact id are the same.
    pub fn is_same_plugin(&self, other: &Plugin) -> bool {
        self.group_id == other.group_id && self.artifact_id == other.artifact_id
    }
}
impl HasElementName for Plugin {
    fn element_name() -> &'static str {
        "plugin"
    }
}
impl ElementConverter for Plugin {
    fn from_element(
        element: edit_xml::Element,
        document: &edit_xml::Document,
    ) -> Result<Self, XMLEditorError> {
        let group_id = find_element_or_err(element, "groupId", document)?.text_content(document);
        let artifact_id =
            find_element_or_err(element, "artifactId", document)?.text_content(document);
        let version = StringOrVariable::from_element(
            find_element_or_err(element, "version", document)?,
            document,
        )?;
        Ok(Self {
            group_id,
            artifact_id,
            version,
        })
    }

    fn into_children(
        self,
        document: &mut edit_xml::Document,
    ) -> Result<Vec<edit_xml::Element>, XMLEditorError> {
        let mut result = vec![];
        let group_id = create_basic_text_element(document, "groupId", self.group_id);
        let artifact_id = create_basic_text_element(document, "artifactId", self.artifact_id);
        let version = create_basic_text_element(document, "version", self.version);
        result.push(group_id);
        result.push(artifact_id);
        result.push(version);
        Ok(result)
    }
}
impl ChildOfListElement for Plugin {
    fn parent_element_name() -> &'static str {
        "plugins"
    }
}
impl UpdatableElement for Plugin {
    fn is_same_item(&self, other: &Self) -> bool {
        self.is_same_plugin(other)
    }

    fn update_element(
        &self,
        element: edit_xml::Element,
        document: &mut edit_xml::Document,
    ) -> Result<(), XMLEditorError> {
        let version = find_element_or_err(element, "version", document)?;
        version.set_text_content(document, self.version.to_string());
        Ok(())
    }
}
