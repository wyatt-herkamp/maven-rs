use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::editor::{
    utils::{add_if_present, from_element_using_builder, sync_element},
    ElementConverter, HasElementName, UpdatableElement,
};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Builder)]
pub struct Parent {
    #[serde(rename = "groupId")]
    #[builder(setter(into, strip_option), default)]
    pub group_id: Option<String>,
    #[serde(rename = "artifactId")]
    #[builder(setter(into, strip_option), default)]
    pub artifact_id: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub version: Option<String>,
    #[serde(rename = "relativePath")]
    #[builder(setter(into, strip_option), default)]
    pub relative_path: Option<String>,
}
impl HasElementName for Parent {
    fn element_name() -> &'static str {
        "parent"
    }
}
impl ElementConverter for Parent {
    from_element_using_builder!(
        ParentBuilder,
        element,
        document,
        "groupId" => group_id,
        "artifactId" => artifact_id,
        "version" => version,
        "relativePath" => relative_path
    );

    fn into_children(
        self,
        document: &mut edit_xml::Document,
    ) -> Result<Vec<edit_xml::Element>, crate::editor::XMLEditorError> {
        let Self {
            group_id,
            artifact_id,
            version,
            relative_path,
        } = self;
        let mut children = vec![];
        add_if_present!(document, children, group_id, "groupId");
        add_if_present!(document, children, artifact_id, "artifactId");
        add_if_present!(document, children, version, "version");
        add_if_present!(document, children, relative_path, "relativePath");
        Ok(children)
    }
}

impl UpdatableElement for Parent {
    fn update_element(
        &self,
        element: edit_xml::Element,
        document: &mut edit_xml::Document,
    ) -> Result<(), crate::editor::XMLEditorError> {
        sync_element(document, element, "version", self.version.as_deref());
        sync_element(
            document,
            element,
            "relativePath",
            self.relative_path.as_deref(),
        );
        Ok(())
    }

    fn is_same_item(&self, other: &Self) -> bool {
        self.group_id == other.group_id && self.artifact_id == other.artifact_id
    }
}
#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::editor::utils::test_utils;

    pub use super::*;

    #[test]
    pub fn test_element_parse() -> anyhow::Result<()> {
        let test_value = r#"
            <parent>
                <groupId>dev.wyatt-herkamp</groupId>
                <artifactId>test</artifactId>
                <version>1.0.0</version>
            </parent>
        "#;
        let dep = test_utils::create_xml_to_element::<Parent>(test_value)?;
        assert_eq!(
            dep,
            Parent {
                group_id: Some("dev.wyatt-herkamp".to_string()),
                artifact_id: Some("test".to_string()),
                version: Some("1.0.0".to_string()),
                ..Default::default()
            }
        );
        println!("{:#?}", dep);
        Ok(())
    }
    #[test]
    pub fn test_is_same_item() {
        let parent = Parent {
            group_id: Some("dev.wyatt-herkamp".to_string()),
            artifact_id: Some("test".to_string()),
            version: Some("1.0.0".to_string()),
            ..Default::default()
        };
        let other = Parent {
            group_id: Some("dev.wyatt-herkamp".to_string()),
            artifact_id: Some("test".to_string()),
            version: Some("0.2".to_string()),
            relative_path: Some("../pom.xml".to_string()),
        };

        assert!(parent.is_same_item(&other));
    }
}
