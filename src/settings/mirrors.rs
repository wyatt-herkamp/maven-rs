use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::editor::{
    ChildOfListElement, ElementConverter, HasElementName, UpdatableElement,
    utils::{
        create_basic_text_element, find_or_create_then_set_text_content, from_element_using_builder,
    },
};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mirrors {
    #[serde(rename = "mirror")]
    pub mirrors: Vec<Mirror>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Mirror {
    pub id: String,
    pub name: String,
    pub url: String,
    pub mirror_of: String,
}
impl HasElementName for Mirror {
    fn element_name() -> &'static str {
        "mirror"
    }
}

impl ElementConverter for Mirror {
    from_element_using_builder!(
        MirrorBuilder,
        element,
        document,
        "id" => id,
        "name" => name,
        "url" => url,
        "mirrorOf" => mirror_of
    );
    fn into_children(
        self,
        document: &mut edit_xml::Document,
    ) -> Result<Vec<edit_xml::Element>, crate::editor::XMLEditorError> {
        let children = vec![
            create_basic_text_element(document, "id", self.id),
            create_basic_text_element(document, "name", self.name),
            create_basic_text_element(document, "url", self.url),
            create_basic_text_element(document, "mirrorOf", self.mirror_of),
        ];
        Ok(children)
    }
}
impl ChildOfListElement for Mirror {
    fn parent_element_name() -> &'static str {
        "mirrors"
    }
}

impl UpdatableElement for Mirror {
    fn update_element(
        &self,
        element: edit_xml::Element,
        document: &mut edit_xml::Document,
    ) -> Result<(), crate::editor::XMLEditorError> {
        let Mirror {
            name,
            url,
            mirror_of,
            ..
        } = self;
        find_or_create_then_set_text_content(document, element, "name", name);
        find_or_create_then_set_text_content(document, element, "url", url);
        find_or_create_then_set_text_content(document, element, "mirrorOf", mirror_of);
        Ok(())
    }
}
