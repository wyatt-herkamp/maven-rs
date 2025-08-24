use derive_builder::Builder;
use edit_xml::{Document, Element};
use serde::{Deserialize, Serialize};

use crate::editor::{
    ChildOfListElement, ComparableElement, ElementConverter, HasElementName, UpdatableElement,
    XMLEditorError,
    utils::{add_if_present, from_element_using_builder, sync_element},
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Developers {
    pub developer: Vec<Developer>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Builder, Serialize, Deserialize)]
pub struct Developer {
    #[builder(setter(into, strip_option), default)]
    pub id: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub name: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub email: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub url: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub organization: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub organization_url: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub timezone: Option<String>,
    // TODO Add roles
}
impl Developer {
    /// Checks if the developer is the same as the other developer.
    ///
    /// Basically, it checks if the id is the same.
    pub fn is_same_developer(&self, other: &Developer) -> bool {
        self.id == other.id
    }
}

impl HasElementName for Developer {
    fn element_name() -> &'static str {
        "developer"
    }
}
impl ElementConverter for Developer {
    from_element_using_builder!(
        DeveloperBuilder,
        element,
        document,
        "id" => id,
        "name" => name,
        "email" => email,
        "url" => url,
        "organization" => organization,
        "organizationUrl" => organization_url,
        "timezone" => timezone
    );

    fn into_children(
        self,
        document: &mut edit_xml::Document,
    ) -> Result<Vec<edit_xml::Element>, crate::editor::XMLEditorError> {
        let Self {
            id,
            name,
            email,
            url,
            organization,
            organization_url,
            timezone,
        } = self;
        let mut children = vec![];
        add_if_present!(document, children, id, "id");
        add_if_present!(document, children, name, "name");
        add_if_present!(document, children, email, "email");
        add_if_present!(document, children, url, "url");
        add_if_present!(document, children, organization, "organization");
        add_if_present!(document, children, organization_url, "organizationUrl");
        add_if_present!(document, children, timezone, "timezone");
        Ok(children)
    }
}
impl ChildOfListElement for Developer {
    fn parent_element_name() -> &'static str {
        "developers"
    }
}
impl ComparableElement for Developer {
    fn is_same_item(&self, other: &Self) -> bool {
        self.is_same_developer(other)
    }
}
impl UpdatableElement for Developer {
    fn update_element(
        &self,
        element: Element,
        document: &mut Document,
    ) -> Result<(), XMLEditorError> {
        sync_element(document, element, "id", self.id.clone());
        sync_element(document, element, "name", self.name.clone());
        sync_element(document, element, "email", self.email.clone());
        sync_element(document, element, "url", self.url.clone());
        sync_element(document, element, "organization", self.organization.clone());
        sync_element(
            document,
            element,
            "organizationUrl",
            self.organization_url.clone(),
        );
        sync_element(document, element, "timezone", self.timezone.clone());

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use crate::editor::utils::test_utils;

    use super::Developer;

    fn test_parse_methods(value: &str, expected: Developer) -> anyhow::Result<()> {
        let dev_via_edit_xml = test_utils::create_xml_to_element::<Developer>(value)?;
        let dev_via_serde: Developer = quick_xml::de::from_str(value)?;

        assert_eq!(dev_via_edit_xml, expected);
        assert_eq!(dev_via_serde, expected);
        println!("{:#?}", dev_via_edit_xml);

        let dep_serialize_serde = quick_xml::se::to_string(&expected)?;
        println!("Serialized Over Serde \n {}", dep_serialize_serde);
        Ok(())
    }

    #[test]
    pub fn test_element_parse() -> anyhow::Result<()> {
        let test_value = r#"
            <developer>
                <id>wyatt-herkamp</id>
                <name>Wyatt Herkamp</name>
                <email>test@wyatt-herkamp.dev</email>
                <url>https://wyatt-herkamp.dev</url>
            </developer>
        "#;
        let dev = Developer {
            id: Some("wyatt-herkamp".to_string()),
            name: Some("Wyatt Herkamp".to_string()),
            email: Some("test@wyatt-herkamp.dev".to_owned()),
            url: Some("https://wyatt-herkamp.dev".to_owned()),

            ..Default::default()
        };
        test_parse_methods(test_value, dev)
    }
}
