use ahash::{HashMap, HashMapExt};

use crate::editor::{ElementConverter, HasElementName, UpdatableElement};
//TODO: Do the values need to be something other than strings?
//TODO: Ordering will be lost if we use a HashMap
/// Represents the properties of a pom file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Properties(pub HashMap<String, String>);
impl HasElementName for Properties {
    fn element_name() -> &'static str {
        "properties"
    }
}
impl ElementConverter for Properties {
    fn from_element(
        element: edit_xml::Element,
        document: &edit_xml::Document,
    ) -> Result<Self, crate::editor::XMLEditorError> {
        let mut properties = HashMap::new();
        for child in element.child_elements(document) {
            let name = child.name(document).to_owned();
            let value = child.text_content(document);
            properties.insert(name, value);
        }
        Ok(Properties(properties))
    }

    fn into_children(
        self,
        document: &mut edit_xml::Document,
    ) -> Result<Vec<edit_xml::Element>, crate::editor::XMLEditorError> {
        let mut children = vec![];
        for (name, value) in self.0 {
            let element = edit_xml::Element::new(document, name);
            element.set_text_content(document, value);
            children.push(element);
        }
        Ok(children)
    }
}

impl UpdatableElement for Properties {
    /// Updating a Properties element means all of the original children are removed and replaced with the new children.
    fn update_element(
        &self,
        element: edit_xml::Element,
        document: &mut edit_xml::Document,
    ) -> Result<(), crate::editor::XMLEditorError> {
        element.clear_children(document);
        for (key, value) in self.0.iter() {
            let child = edit_xml::Element::new(document, key);
            child.set_text_content(document, value);
            element.push_child(document, child.into())?;
        }
        Ok(())
    }
}
