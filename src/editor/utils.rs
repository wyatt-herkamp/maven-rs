use super::{
    ChildOfListElement, ComparableElement, ElementConverter, HasElementName, PomValue,
    UpdatableElement, XMLEditorError,
};
use edit_xml::{Document, Element};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Missing Element {0}")]
pub struct MissingElementError(pub &'static str);
pub fn find_element(element: Element, name: &'static str, document: &Document) -> Option<Element> {
    element
        .child_elements(document)
        .into_iter()
        .find(|x| x.name(document) == name)
}
pub fn find_element_or_err(
    element: Element,
    name: &'static str,
    document: &Document,
) -> Result<Element, MissingElementError> {
    element
        .child_elements(document)
        .into_iter()
        .find(|x| x.name(document) == name)
        .ok_or(MissingElementError(name))
}
pub fn find_to_string_or_none(
    element: Element,
    name: &'static str,
    document: &Document,
) -> Option<String> {
    element
        .child_elements(document)
        .into_iter()
        .find(|x| x.name(document) == name)
        .map(|x| x.text_content(document))
}
pub fn create_basic_text_element(
    document: &mut Document,
    name: impl Into<String>,
    value: impl PomValue,
) -> Element {
    let element = Element::new(document, name);
    element.set_text_content(document, value.to_string_for_editor());
    element
}

pub fn get_or_create_top_level_element(
    name: &'static str,
    document: &mut Document,
    parent: Element,
) -> Element {
    if let Some(element) = find_element(parent, name, document) {
        return element;
    }
    let element = Element::new(document, name);
    parent
        .push_child(document, element.into())
        .expect("Failed to add element");
    element
}
/// Finds an Element with the name of name.
///
/// If it does not exist, it will be created.
///
/// Then the text content of the element will be set to value.
///
/// The children of the element will be cleared.
pub(crate) fn find_or_create_then_set_text_content(
    document: &mut Document,
    parent: Element,
    name: &'static str,
    value: impl Into<String>,
) {
    let element = get_or_create_top_level_element(name, document, parent);
    element.clear_children(document);
    element.set_text_content(document, value);
}
/// Syncs an element with the name of name.
///
/// If the value is Some, the element will be created or updated with the value.
///
/// If the value is None, the element will be removed.
pub(crate) fn sync_element<V: Into<String>>(
    document: &mut Document,
    parent: Element,
    name: &'static str,
    value: Option<V>,
) {
    if let Some(value) = value {
        let element = get_or_create_top_level_element(name, document, parent);
        element.clear_children(document);
        element.set_text_content(document, value);
    } else {
        let element = find_element(parent, name, document);
        if let Some(element) = element {
            element.detach(document).expect("Failed to remove element");
        }
    }
}

/// Adds or updates an element in a parent element. All elements in the parent element must be of the same type.
///
/// If the parent element does not exist, it will be created. and the element will be added to it.
/// If the element already exists, it will be updated. Uses [UpdatableElement::is_same_item] to check if the element is the same.
/// If the element does not exist, it will be added to the parent element.
pub(crate) fn add_or_update_item<I>(
    document: &mut Document,
    parent_element: Option<Element>,
    insert_into: Element,
    item: I,
) -> Result<Option<I>, XMLEditorError>
where
    I: UpdatableElement
        + ElementConverter
        + ChildOfListElement
        + HasElementName
        + ComparableElement,
{
    let Some(parent_container) = parent_element else {
        // No parent element found element found, create it and add the dependency
        let dependencies = Element::new(document, I::parent_element_name());
        let value = item.into_element(document)?;
        dependencies.push_child(document, value.into())?;
        insert_into.push_child(document, dependencies.into())?;
        return Ok(None);
    };
    let elements_in_parent = get_all_children_of_element::<I>(document, parent_container)?;
    for (current_value, element) in elements_in_parent {
        // A dependency with the same group_id and artifact_id is already present
        // Update the version and return the old dependency
        if current_value.is_same_item(&item) {
            item.update_element(element, document)?;
            return Ok(Some(current_value));
        }
    }
    // No dependency with the same group_id and artifact_id is present
    let value = item.into_element(document)?;
    parent_container.push_child(document, value.into())?;
    Ok(None)
}
/// Gets all children of an element and converts them to a specific type.
pub(crate) fn get_all_children_of_element<E>(
    document: &Document,
    element: Element,
) -> Result<Vec<(E, Element)>, XMLEditorError>
where
    E: ElementConverter + HasElementName,
{
    let mut result = vec![];

    for raw_element in element.child_elements(document) {
        let element_name = raw_element.name(document);
        if element_name != E::element_name() {
            return Err(XMLEditorError::UnexpectedElementType {
                expected: E::element_name(),
                found: element_name.to_owned(),
            });
        }
        let value = E::from_element(raw_element, document)?;
        result.push((value, raw_element));
    }
    Ok(result)
}

macro_rules! add_if_present {
    (
        $document:ident,
        $children:ident,
        $element:ident,
        $name:literal
    ) => {
        if let Some(value) = $element {
            $children.push(crate::editor::utils::create_basic_text_element(
                $document, $name, value,
            ));
        }
    };
}

pub(crate) use add_if_present;
macro_rules! from_element_using_builder {
    (
        $builder:ident,
        $element:ident,
        $document:ident,
        $(
            $name:literal => $set_func:ident
        ),*
    ) => {
        fn from_element(
            element: edit_xml::Element,
            document: &edit_xml::Document,
        ) -> Result<Self, crate::editor::XMLEditorError> {
        let mut builder = $builder::default();
        for child in element.child_elements(document) {
            match child.name(document) {
                    $(
                    $name => {
                        builder.$set_func(child.text_content(document));
                    }
                    )*
                    _ => {}
                }
            }
            let result = builder.build()?;
            return Ok(result);
        }
    };
}
pub(crate) use from_element_using_builder;

macro_rules! typed_from_element_using_builder {
    (
        $builder:ident,
        $element:ident,
        $document:ident,
        $(
            $name:literal($element_type:ty) => $set_func:ident
        ),*
    ) => {
        fn from_element(
            element: edit_xml::Element,
            document: &edit_xml::Document,
        ) -> Result<Self, crate::editor::XMLEditorError> {
        let mut builder = $builder::default();
        for child in element.child_elements(document) {
            match child.name(document) {
                    $(
                        $name => {
                            builder.$set_func(<$element_type as crate::editor::PomValue>::from_element(child, document)?);
                        }
                    )*
                    _ => {}
                }
            }
            let result = builder.build()?;
            return Ok(result);
        }
    };
}
pub(crate) use typed_from_element_using_builder;

#[cfg(test)]
pub(crate) mod test_utils {
    use crate::editor::{ElementConverter, HasElementName, XMLEditorError};
    use pretty_assertions::assert_eq;
    #[track_caller]
    pub fn create_xml_to_element<E>(xml: &str) -> Result<E, XMLEditorError>
    where
        E: ElementConverter + HasElementName,
    {
        let actual_xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            {xml}
            "#
        );
        let document = edit_xml::Document::parse_str(&actual_xml).unwrap();
        let Some(raw_element) = document.root_element() else {
            println!("{}", actual_xml);
            panic!("No root element found");
        };
        let name = raw_element.name(&document);
        assert_eq!(
            name,
            E::element_name(),
            "Expected element name to be {}",
            E::element_name()
        );
        E::from_element(raw_element, &document)
    }
}
