use edit_xml::Element;

use crate::{
    editor::{
        utils::{add_or_update_item, get_all_children_of_element},
        XMLEditorError,
    },
    pom::Dependency,
};

use super::PomEditor;
impl PomEditor {
    /// Creates a new [DependencyManagementEditor]
    ///
    /// If no `dependencyManagement` element is present, it will create one
    /// # Note.
    /// This function will hold a mutable reference to the PomEditor.
    /// I would recommend using this function within a scope. To prevent borrowing issues.
    pub fn get_or_create_dependency_management_element(
        &mut self,
    ) -> DependencyManagementEditor<'_> {
        DependencyManagementEditor::new(self)
    }
    /// Checks if the `dependencyManagement` element is present in the pom file
    ///
    /// If the element is present, it will return Some(BuildEditor) else it will return None
    pub fn get_dependency_management_element_or_none(
        &mut self,
    ) -> Option<DependencyManagementEditor<'_>> {
        if self.has_build() {
            return Some(DependencyManagementEditor::new(self));
        }
        None
    }
    /// Checks if the `dependencyManagement` element is present in the pom file
    ///
    /// If the `dependencyManagement` element is present, it will return true else it will return false
    pub fn has_dependency_management(&self) -> bool {
        let root = self.root();
        root.find(&self.document, "dependencyManagement").is_some()
    }
    /// Deletes the `dependencyManagement` element from the pom file
    ///
    /// If the `dependencyManagement` element is present, it will delete it and return true else it will return false
    pub fn delete_dependency_management(&mut self) -> Result<bool, XMLEditorError> {
        let root = self.root();
        let element = root.find(&self.document, "dependencyManagement");
        if let Some(element) = element {
            element.detach(&mut self.document)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
#[derive(Debug)]
pub struct DependencyManagementEditor<'a> {
    parent: &'a mut PomEditor,
    dependency_management_element: Element,
}

impl<'a> DependencyManagementEditor<'a> {
    pub(super) fn new(parent: &'a mut PomEditor) -> Self {
        let root = parent.root();
        let build_element = crate::editor::utils::get_or_create_top_level_element(
            "dependencyManagement",
            &mut parent.document,
            root,
        );
        Self {
            parent,
            dependency_management_element: build_element,
        }
    }
    fn dependencies_element(&self) -> Option<Element> {
        self.dependency_management_element
            .find(&self.parent.document, "dependencies")
    }
    pub fn get_dependencies(&self) -> Result<Vec<Dependency>, XMLEditorError> {
        let Some(dependencies_element) = self.dependencies_element() else {
            return Ok(vec![]);
        };
        let result =
            get_all_children_of_element::<Dependency>(&self.parent.document, dependencies_element)?;
        Ok(result.into_iter().map(|(depend, _)| depend).collect())
    }
    pub fn add_or_update_dependency(
        &mut self,
        dependency: Dependency,
    ) -> Result<Option<Dependency>, XMLEditorError> {
        let dependencies_element = self.dependencies_element();
        add_or_update_item(
            &mut self.parent.document,
            dependencies_element,
            self.dependency_management_element,
            dependency,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::pom::editor::PomEditor;

    #[test]
    fn test_read_no_dependencies() -> anyhow::Result<()> {
        let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/maven-v4_0_0.xsd">
            <modelVersion>4.0.0</modelVersion>
            <groupId>dev.kingtux</groupId>
            <artifactId>test</artifactId>
            <version>1</version>
        </project>
        "#;
        let mut editor = PomEditor::load_from_str(xml).unwrap();
        assert!(
            !editor.has_dependency_management(),
            "Should not have dependency management"
        );
        let dependency_management = editor.get_or_create_dependency_management_element();
        let dependencies = dependency_management.get_dependencies().unwrap();
        assert_eq!(dependencies.len(), 0);
        Ok(())
    }

    #[test]
    fn test_read_lwjgl_bom() -> anyhow::Result<()> {
        let xml_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/data/test_poms/lwjgl-bom-3.3.4.pom");
        let file = std::fs::read_to_string(xml_path)?;
        let mut editor = PomEditor::load_from_str(&file)?;
        assert!(
            editor.has_dependency_management(),
            "Should have dependency management"
        );
        let dependency_management = editor.get_or_create_dependency_management_element();
        let dependencies = dependency_management.get_dependencies().unwrap();
        for dependency in dependencies {
            println!("{:?}", dependency);
        }
        Ok(())
    }

    #[test]
    fn delete_dependency_management() -> anyhow::Result<()> {
        let xml_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/data/test_poms/lwjgl-bom-3.3.4.pom");
        let file = std::fs::read_to_string(xml_path)?;
        let mut editor = PomEditor::load_from_str(&file)?;
        assert!(
            editor.has_dependency_management(),
            "Should have dependency management"
        );

        editor.delete_dependency_management()?;

        assert!(
            !editor.has_dependency_management(),
            "Should not have dependency management"
        );
        let saved_file = editor.write_to_str()?;

        let editor = PomEditor::load_from_str(&saved_file)?;
        assert!(
            !editor.has_dependency_management(),
            "Should not have dependency management"
        );
        println!("{}", saved_file);
        Ok(())
    }
}
