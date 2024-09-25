use edit_xml::Element;

use crate::{
    editor::{
        utils::{add_or_update_item, find_element, get_all_children_of_element},
        XMLEditorError,
    },
    pom::Dependency,
};

use super::PomEditor;

#[derive(Debug)]
pub struct DependencyManagement<'a> {
    parent: &'a mut PomEditor,
    dependency_management_element: Element,
}

impl<'a> DependencyManagement<'a> {
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
    pub fn get_dependencies(&self) -> Result<Vec<Dependency>, XMLEditorError> {
        let Some(dependencies_element) = find_element(
            self.dependency_management_element,
            "dependencies",
            &self.parent.document,
        ) else {
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
        let dependencies_element = find_element(
            self.dependency_management_element,
            "dependencies",
            &self.parent.document,
        );
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
        let dependency_management = editor.dependency_management_editor();
        let dependencies = dependency_management.get_dependencies().unwrap();
        assert_eq!(dependencies.len(), 0);
        Ok(())
    }

    #[test]
    fn test_read_lwjgl_bom() -> anyhow::Result<()> {
        let xml_path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/lwjgl-bom-3.3.4.pom");
        let file = std::fs::read_to_string(xml_path)?;
        let mut editor = PomEditor::load_from_str(&file)?;
        let dependency_management = editor.dependency_management_editor();
        let dependencies = dependency_management.get_dependencies().unwrap();
        for dependency in dependencies {
            println!("{:?}", dependency);
        }
        Ok(())
    }
}
