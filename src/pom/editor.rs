//! Pom Editor
//!
//! ## What is a Pom Editor?
//! A pom editor is a struct that allows you to edit and create pom files.
//! ## What is the difference between a [PomEditor] and a [crate::pom::Pom]?
//!
//! [crate::pom::Pom] is a struct that represents the data in a pom file. If you were to use it to edit a pom file. The data could be lost due
//! How XML works with Serde
//!
//! [PomEditor] uses the crate edit-xml to parse the xml into a dom like structure.
//! This allows for easy editing of the xml file. Without losing any data or original structure.

use std::io::Write;

use edit_xml::{Document, EditXMLError, Element, WriteOptions};
mod build;
use crate::editor::{
    utils::{add_or_update_item, find_element, get_all_children_of_element, MissingElementError},
    ElementConverter, UpdatableElement, XMLEditorError,
};

use super::{depend::Dependency, Parent, Repository};

/// A struct that allows editing and creating pom files
/// A pom file is an xml file that follows the maven pom schema
#[derive(Debug)]
pub struct PomEditor {
    /// The document is kept private to prevent the root element from being changed.
    ///
    /// The root element must always be a project element. As this code assumes it exists and without it. Panicking would occur.
    document: Document,
    pub ident_level: usize,
}
impl Default for PomEditor {
    fn default() -> Self {
        let mut document = Document::new();
        let container = document.container();
        Element::build("project")
            .attribute("xmlns", "http://maven.apache.org/POM/4.0.0")
            .attribute("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance")
            .attribute(
                "xsi:schemaLocation",
                "http://maven.apache.org/POM/4.0.0 http://maven.apache.org/maven-v4_0_0.xsd",
            )
            .push_to(&mut document, container);
        Self {
            document,
            ident_level: 2,
        }
    }
}
macro_rules! top_level_getter_setter {
    (
        $set:ident, $get:ident, $name:literal
    ) => {
        pub fn $set(&mut self, value: &str) {
            let root = self.root();
            let element = crate::editor::utils::get_or_create_top_level_element(
                $name,
                &mut self.document,
                root,
            );

            element.set_text_content(&mut self.document, value);
        }
        pub fn $get(&self) -> Option<String> {
            let root = self.root();
            let element = crate::editor::utils::find_element(root, $name, &self.document);
            return element.map(|x| x.text_content(&self.document));
        }
    };
}
impl PomEditor {
    pub fn new_with_group_and_artifact(group_id: &str, artifact_id: &str) -> Self {
        let mut editor = Self::default();
        editor.set_group_id(group_id);
        editor.set_artifact_id(artifact_id);
        editor
    }
    pub fn get_parent(&self) -> Result<Option<Parent>, XMLEditorError> {
        let root = self.root();
        find_element(root, "parent", &self.document)
            .map(|x| Parent::from_element(x, &self.document))
            .transpose()
    }
    pub fn set_parent(&mut self, parent: Parent) -> Result<(), XMLEditorError> {
        let root = self.root();
        let parent_element = find_element(root, "parent", &self.document);
        if let Some(parent_element) = parent_element {
            parent.update_element(parent_element, &mut self.document)?;
        }
        let new_parent_element = parent.into_element(&mut self.document)?;
        root.push_child(&mut self.document, new_parent_element.into())?;
        Ok(())
    }
    top_level_getter_setter!(set_group_id, get_group_id, "groupId");
    top_level_getter_setter!(set_artifact_id, get_artifact_id, "artifactId");
    top_level_getter_setter!(set_version, get_version, "version");
    top_level_getter_setter!(set_name, get_name, "name");
    top_level_getter_setter!(set_description, get_description, "description");
    // TODO: Repositories, pluginRepositories
    // Loads a pom from a string
    pub fn load_from_str(value: &str) -> Result<Self, XMLEditorError> {
        let document = Document::parse_str(value)?;
        Self::assert_requirements_for_pom(&document)?;
        Ok(Self {
            document,
            ident_level: 2,
        })
    }
    // Loads a pom from a reader
    pub fn load_from_reader<R: std::io::Read>(reader: R) -> Result<Self, XMLEditorError> {
        let document = Document::parse_reader(reader)?;
        Self::assert_requirements_for_pom(&document)?;
        Ok(Self {
            document,
            ident_level: 2,
        })
    }
    fn assert_requirements_for_pom(document: &Document) -> Result<(), XMLEditorError> {
        let root = document
            .root_element()
            .ok_or(MissingElementError("project"))?;
        if root.name(document) != "project" {
            return Err(XMLEditorError::UnexpectedElementType {
                expected: "project",
                found: root.name(document).to_owned(),
            });
        }
        Ok(())
    }
    pub fn get_repositories(&self) -> Result<Vec<Repository>, XMLEditorError> {
        let root = self.root();
        let Some(repositories_element) = find_element(root, "repositories", &self.document) else {
            return Ok(vec![]);
        };
        let result =
            get_all_children_of_element::<Repository>(&self.document, repositories_element)?;
        Ok(result.into_iter().map(|(repo, _)| repo).collect())
    }
    pub fn add_or_update_repository(
        &mut self,
        repository: Repository,
    ) -> Result<Option<Repository>, XMLEditorError> {
        let root = self.root();
        let repositories_element = find_element(root, "repositories", &self.document);
        add_or_update_item(&mut self.document, repositories_element, root, repository)
    }
    pub fn get_dependencies(&self) -> Result<Vec<Dependency>, XMLEditorError> {
        let root = self.root();
        let Some(dependencies_element) = find_element(root, "dependencies", &self.document) else {
            return Ok(vec![]);
        };
        let result =
            get_all_children_of_element::<Dependency>(&self.document, dependencies_element)?;
        Ok(result.into_iter().map(|(depend, _)| depend).collect())
    }
    pub fn add_or_update_dependency(
        &mut self,
        dependency: Dependency,
    ) -> Result<Option<Dependency>, XMLEditorError> {
        let root = self.root();
        let dependencies_element = find_element(root, "dependencies", &self.document);
        add_or_update_item(&mut self.document, dependencies_element, root, dependency)
    }
    /// Creates a new build editor
    ///
    /// If no build element is present, it will create one
    ///
    /// If a build element is present, it will return the existing one
    pub fn build_editor(&mut self) -> build::BuildEditor<'_> {
        return build::BuildEditor::new(self);
    }

    pub(crate) fn root(&self) -> Element {
        self.document.root_element().unwrap()
    }
    pub fn write_to_str(&self) -> Result<String, EditXMLError> {
        self.document.write_str_with_opts(WriteOptions {
            write_decl: true,
            indent_size: self.ident_level,
            ..Default::default()
        })
    }
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), EditXMLError> {
        self.document.write_with_opts(
            writer,
            WriteOptions {
                write_decl: true,
                indent_size: self.ident_level,
                ..Default::default()
            },
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    pub fn create() -> anyhow::Result<()> {
        let mut editor = PomEditor::default();
        editor.set_group_id("dev.wyatt-herkamp");
        editor.set_artifact_id("test");
        let value = editor.write_to_str()?;
        println!("{}", value);
        let mut new_editor = PomEditor::load_from_str(value.as_str())?;

        // Make sure the group id and artifact id are correct
        assert_eq!(
            new_editor.get_group_id(),
            Some("dev.wyatt-herkamp".to_string())
        );
        assert_eq!(new_editor.get_artifact_id(), Some("test".to_string()));
        // Try Changing the group id and artifact id
        new_editor.set_group_id("dev.wyatt-herkamp2");
        new_editor.set_artifact_id("test2");
        assert_eq!(
            new_editor.get_group_id(),
            Some("dev.wyatt-herkamp2".to_string())
        );
        assert_eq!(new_editor.get_artifact_id(), Some("test2".to_string()));
        let value = new_editor.write_to_str()?;
        println!("{}", value);
        Ok(())
    }
    #[test]
    pub fn dependencies() -> anyhow::Result<()> {
        let mut editor = PomEditor::new_with_group_and_artifact("dev.wyatt-herkamp", "test");
        let dependency = Dependency {
            group_id: "com.google.guava".to_string(),
            artifact_id: "guava".to_string(),
            version: "30.1-jre".to_string(),
            depend_type: None,
            scope: None,
            classifier: None,
        };
        editor.add_or_update_dependency(dependency.clone())?;

        let value = editor.write_to_str()?;
        println!("{}", value);

        let new_editor = PomEditor::load_from_str(value.as_str())?;
        let dependencies = new_editor.get_dependencies()?;
        println!("{:#?}", dependencies);
        assert!(dependencies.len() == 1);
        assert_eq!(dependencies[0], dependency);

        Ok(())
    }

    #[test]
    pub fn repositories() -> anyhow::Result<()> {
        let mut editor = PomEditor::new_with_group_and_artifact("dev.wyatt-herkamp", "test");
        let repository = Repository {
            id: Some("central".to_string()),
            name: Some("Maven Central Repository".to_string()),
            url: "https://repo.maven.apache.org/maven2".to_string(),
            layout: None,
            ..Default::default()
        };
        editor.add_or_update_repository(repository.clone())?;

        let value = editor.write_to_str()?;
        println!("{}", value);

        let new_editor = PomEditor::load_from_str(value.as_str())?;
        let repositories = new_editor.get_repositories()?;
        println!("{:#?}", repositories);
        assert!(repositories.len() == 1);
        assert_eq!(repositories[0], repository);

        Ok(())
    }
}
