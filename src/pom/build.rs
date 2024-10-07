use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    editor::{
        utils::{
            add_if_present, create_basic_text_element, find_element, find_element_or_err,
            sync_element,
        },
        ChildOfListElement, ComparableElement, ElementConverter, HasElementName, PomValue,
        UpdatableElement, XMLEditorError,
    },
    types::Property,
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
    pub group_id: Option<String>,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub version: Option<Property>,
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
        let group_id = find_element(element, "groupId", document)
            .map(|group_id| String::from_element(group_id, document))
            .transpose()?;
        let artifact_id =
            find_element_or_err(element, "artifactId", document)?.text_content(document);
        let version = find_element(element, "version", document)
            .map(|element| Property::from_element(element, document))
            .transpose()?;

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
        let Self {
            group_id,
            artifact_id,
            version,
        } = self;
        let mut result = vec![];
        add_if_present!(document, result, group_id, "groupId");
        result.push(create_basic_text_element(
            document,
            "artifactId",
            artifact_id,
        ));
        add_if_present!(document, result, version, "version");

        Ok(result)
    }
}
impl ChildOfListElement for Plugin {
    fn parent_element_name() -> &'static str {
        "plugins"
    }
}
impl ComparableElement for Plugin {
    fn is_same_item(&self, other: &Self) -> bool {
        self.is_same_plugin(other)
    }
}
impl UpdatableElement for Plugin {
    fn update_element(
        &self,
        element: edit_xml::Element,
        document: &mut edit_xml::Document,
    ) -> Result<(), XMLEditorError> {
        sync_element(
            document,
            element,
            "version",
            self.version.as_ref().map(|v| v.to_string()),
        );
        Ok(())
    }
}
