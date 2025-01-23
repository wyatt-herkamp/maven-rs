use edit_xml::Element;

use super::PomEditor;
use crate::editor::ElementConverter;
use crate::editor::{UpdatableElement, XMLEditorError};
use crate::pom::{DistributionRepositoryRepository, DistributionRepositorySnapshotRepository};
impl PomEditor {
    /// Creates a new build editor
    ///
    /// If no build element is present, it will create one
    /// # Note.
    /// This function will hold a mutable reference to the PomEditor.
    /// I would recommend using this function within a scope. To prevent borrowing issues.
    pub fn get_or_create_distribution_management_element(
        &mut self,
    ) -> DistributionManagementEditor<'_> {
        DistributionManagementEditor::new(self)
    }
    /// Checks if the build element is present in the pom file
    ///
    /// If the build element is present, it will return Some(BuildEditor) else it will return None
    pub fn get_distribution_management_element_or_none(
        &mut self,
    ) -> Option<DistributionManagementEditor<'_>> {
        if self.has_distribution_management() {
            return Some(DistributionManagementEditor::new(self));
        }
        None
    }
    pub fn has_distribution_management(&self) -> bool {
        let root = self.root();
        root.find(&self.document, "distributionManagement")
            .is_some()
    }
    pub fn delete_distribution_management(&mut self) -> Result<bool, XMLEditorError> {
        let root = self.root();
        let element = root.find(&self.document, "distributionManagement");
        if let Some(element) = element {
            element.detach(&mut self.document)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
/// Allows for editing the [Distribution Management](https://maven.apache.org/pom.html#Distribution_Management) section of a pom file
#[derive(Debug)]
pub struct DistributionManagementEditor<'a> {
    parent: &'a mut PomEditor,
    element: Element,
}
macro_rules! top_level_structured_type {
    (
        $(#[$set_docs:meta])*
        set: $set:ident,
        $(#[$get_docs:meta])*
        get: $get:ident,
        $element_name:literal => $structured_type:ident,
    ) => {
        $(#[$get_docs])*
        pub fn $get(&self) -> Result<Option<$structured_type>, XMLEditorError> {
            self.element.find(&self.parent.document, $element_name)
                .map(|x| $structured_type::from_element(x, &self.parent.document))
                .transpose()
        }
        $(#[$set_docs])*
        pub fn $set<U>(&mut self, value: U) -> Result<(), XMLEditorError>
        where
            U: Into<Option<$structured_type>> {
            let value: Option<$structured_type> = value.into();
            let root = self.element;
            let existing_element = root.find(&self.parent.document, $element_name);
            if let Some(value) = value{
                if let Some(element) = existing_element {
                    value.update_element(element, &mut self.parent.document)?;
                    return Ok(());
                }
                let new_element = value.into_element(&mut self.parent.document)?;
                root.push_child(&mut self.parent.document, new_element)?;
            }else{
                if let Some(element) = existing_element {
                    element.detach(&mut self.parent.document)?;
                }
            }

            Ok(())
        }
    };
}

impl<'a> DistributionManagementEditor<'a> {
    pub(super) fn new(parent: &'a mut PomEditor) -> Self {
        let root = parent.root();
        let element: Element = crate::editor::utils::get_or_create_top_level_element(
            "distributionManagement",
            &mut parent.document,
            root,
        );
        Self { parent, element }
    }
    top_level_structured_type!(
        set: set_repository,
        get: get_repository,
        "repository" => DistributionRepositoryRepository,
    );
    top_level_structured_type!(
        set: set_snapshot_repository,
        get: get_snapshot_repository,
        "snapshotRepository" => DistributionRepositorySnapshotRepository,
    );
}

#[cfg(test)]
mod tests {
    use crate::pom::{distribution_management, editor::PomEditor};

    #[test]
    pub fn test_plugins() -> anyhow::Result<()> {
        let mut editor = PomEditor::new_with_group_and_artifact("dev.wyatt-herkamp", "test");
        {
            let mut distribution_management =
                editor.get_or_create_distribution_management_element();
            let repository = distribution_management.get_repository()?;
            assert!(repository.is_none());

            let repository = distribution_management.get_snapshot_repository()?;
            assert!(repository.is_none());

            distribution_management.set_repository(Some(
                distribution_management::DistributionRepository {
                    id: Some("test".to_string()),
                    name: Some("test".to_string()),
                    url: "https://test.com".to_string(),
                    layout: Some("default".to_string()),
                    ..Default::default()
                }
                .repository(),
            ))?;

            distribution_management.set_repository(Some(
                distribution_management::DistributionRepository {
                    id: Some("test".to_string()),
                    name: Some("test".to_string()),
                    url: "https://test.com".to_string(),
                    layout: Some("default".to_string()),
                    ..Default::default()
                }
                .repository(),
            ))?;

            distribution_management.set_snapshot_repository(
                distribution_management::DistributionRepository {
                    id: Some("test-snapshot".to_string()),
                    name: Some("test".to_string()),
                    url: "https://test.com".to_string(),
                    layout: Some("default".to_string()),
                    ..Default::default()
                }
                .snapshot_repository(),
            )?;
        }
        let value = editor.write_to_str()?;
        println!("{}", value);

        Ok(())
    }
}
