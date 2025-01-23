use derive_builder::Builder;
use edit_xml::Element;
use serde::{Deserialize, Serialize};

use crate::editor::{
    utils::{add_if_present, find_or_create_then_set_text_content, sync_element},
    ComparableElement, ElementConverter, HasElementName, PomValue, UpdatableElement,
};

use super::{ChecksumPolicy, UpdatePolicy};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DistributionRepository {
    #[builder(setter(into, strip_option), default)]
    pub id: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub name: Option<String>,
    pub url: String,
    #[builder(setter(into, strip_option), default)]
    pub layout: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub update_policy: Option<UpdatePolicy>,
    #[builder(setter(into, strip_option), default)]
    pub checksum_policy: Option<ChecksumPolicy>,
}
impl DistributionRepository {
    pub fn repository(self) -> DistributionRepositoryRepository {
        DistributionRepositoryRepository::new(self)
    }
    pub fn snapshot_repository(self) -> DistributionRepositorySnapshotRepository {
        DistributionRepositorySnapshotRepository::new(self)
    }
}

impl ComparableElement for DistributionRepository {
    fn is_same_item(&self, other: &Self) -> bool {
        if self.name.is_none() {
            return false;
        }
        self.name == other.name
    }
}
impl UpdatableElement for DistributionRepository {
    fn update_element(
        &self,
        element: Element,
        document: &mut edit_xml::Document,
    ) -> Result<(), crate::editor::XMLEditorError> {
        sync_element(document, element, "id", self.id.as_deref());
        sync_element(document, element, "name", self.name.as_deref());
        find_or_create_then_set_text_content(document, element, "url", self.url.as_str());
        // TODO: Layout
        Ok(())
    }
}
impl ElementConverter for DistributionRepository {
    fn from_element(
        element: edit_xml::Element,
        document: &edit_xml::Document,
    ) -> Result<Self, crate::editor::XMLEditorError> {
        let mut builder = DistributionRepositoryBuilder::default();
        for child in element.child_elements(document) {
            match child.name(document) {
                "id" => {
                    builder.id(String::from_element(child, document)?);
                }
                "name" => {
                    builder.name(String::from_element(child, document)?);
                }
                "url" => {
                    builder.url(String::from_element(child, document)?);
                }
                "layout" => {
                    builder.layout(String::from_element(child, document)?);
                }
                "updatePolicy" => {
                    builder.update_policy(UpdatePolicy::from_element(child, document)?);
                }
                "checksumPolicy" => {
                    builder.checksum_policy(ChecksumPolicy::from_element(child, document)?);
                }
                _ => {}
            }
        }
        let result = builder.build()?;
        Ok(result)
    }
    // TODO: Releases, Snapshots

    fn into_children(
        self,
        document: &mut edit_xml::Document,
    ) -> Result<Vec<edit_xml::Element>, crate::editor::XMLEditorError> {
        let Self {
            id,
            name,
            url,
            layout,
            update_policy,
            checksum_policy,
        } = self;
        let mut children = vec![];
        add_if_present!(document, children, id, "id");
        add_if_present!(document, children, name, "name");
        children.push(crate::editor::utils::create_basic_text_element(
            document, "url", url,
        ));
        add_if_present!(document, children, layout, "layout");
        add_if_present!(document, children, update_policy, "updatePolicy");
        add_if_present!(document, children, checksum_policy, "checksumPolicy");

        Ok(children)
    }
}

pub struct DistributionRepositoryRepository(DistributionRepository);
impl DistributionRepositoryRepository {
    pub fn new(repository: DistributionRepository) -> Self {
        Self(repository)
    }
}
impl From<DistributionRepository> for DistributionRepositoryRepository {
    fn from(repository: DistributionRepository) -> Self {
        Self(repository)
    }
}
impl HasElementName for DistributionRepositoryRepository {
    fn element_name() -> &'static str {
        "repository"
    }
}
impl ComparableElement for DistributionRepositoryRepository {
    fn is_same_item(&self, other: &Self) -> bool {
        self.0.is_same_item(&other.0)
    }
}
impl UpdatableElement for DistributionRepositoryRepository {
    fn update_element(
        &self,
        element: Element,
        document: &mut edit_xml::Document,
    ) -> Result<(), crate::editor::XMLEditorError> {
        self.0.update_element(element, document)
    }
}
impl ElementConverter for DistributionRepositoryRepository {
    fn from_element(
        element: edit_xml::Element,
        document: &edit_xml::Document,
    ) -> Result<Self, crate::editor::XMLEditorError> {
        let repository = DistributionRepository::from_element(element, document)?;
        Ok(Self(repository))
    }
    // TODO: Releases, Snapshots

    fn into_children(
        self,
        document: &mut edit_xml::Document,
    ) -> Result<Vec<edit_xml::Element>, crate::editor::XMLEditorError> {
        self.0.into_children(document)
    }
}

pub struct DistributionRepositorySnapshotRepository(DistributionRepository);
impl DistributionRepositorySnapshotRepository {
    pub fn new(repository: DistributionRepository) -> Self {
        Self(repository)
    }
}
impl From<DistributionRepository> for DistributionRepositorySnapshotRepository {
    fn from(repository: DistributionRepository) -> Self {
        Self(repository)
    }
}
impl HasElementName for DistributionRepositorySnapshotRepository {
    fn element_name() -> &'static str {
        "snapshotRepository"
    }
}
impl ComparableElement for DistributionRepositorySnapshotRepository {
    fn is_same_item(&self, other: &Self) -> bool {
        self.0.is_same_item(&other.0)
    }
}
impl UpdatableElement for DistributionRepositorySnapshotRepository {
    fn update_element(
        &self,
        element: Element,
        document: &mut edit_xml::Document,
    ) -> Result<(), crate::editor::XMLEditorError> {
        self.0.update_element(element, document)
    }
}
impl ElementConverter for DistributionRepositorySnapshotRepository {
    fn from_element(
        element: edit_xml::Element,
        document: &edit_xml::Document,
    ) -> Result<Self, crate::editor::XMLEditorError> {
        let repository = DistributionRepository::from_element(element, document)?;
        Ok(Self(repository))
    }
    // TODO: Releases, Snapshots

    fn into_children(
        self,
        document: &mut edit_xml::Document,
    ) -> Result<Vec<edit_xml::Element>, crate::editor::XMLEditorError> {
        self.0.into_children(document)
    }
}
