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

use edit_xml::{Document, Element, ReadOptions, WriteOptions};
mod build;
mod dependency_management;
use super::{depend::Dependency, Developer, Parent, Repository, Scm};
use crate::editor::{
    utils::{add_or_update_item, get_all_children_of_element, MissingElementError},
    ElementConverter, UpdatableElement, XMLEditorError,
};
pub use build::*;
pub use dependency_management::*;

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
        let document = Document::new_with_root("project", |project| {
            project
                .attribute("xmlns", "http://maven.apache.org/POM/4.0.0")
                .attribute("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance")
                .attribute(
                    "xsi:schemaLocation",
                    "http://maven.apache.org/POM/4.0.0 http://maven.apache.org/maven-v4_0_0.xsd",
                )
        });
        let mut editor = Self {
            document,
            ident_level: 2,
        };

        editor.set_model_version("4.0.0");
        editor
    }
}
macro_rules! simple_type_getter_setter {
    (
        $(#[$shared_docs:meta])*
        $name:literal {
            $(#[$set_docs:meta])*
            set: $set:ident,
            $(#[$get_docs:meta])*
            get: $get:ident,
        }
    ) => {
        $(#[$set_docs])*
        $(#[$shared_docs])*
        pub fn $set<S, O>(&mut self, value: O)
            where
                S: Into<String>,
                O: Into<Option<S>>
            {
            let root = self.root();
            let value: Option<S> = value.into();
            if let Some(value) = value {
                let value = value.into();
                let element = crate::editor::utils::get_or_create_top_level_element(
                    $name,
                    &mut self.document,
                    root,
                );
                element.set_text_content(&mut self.document, value);
            }else{
                let element = root.find(&self.document, $name);
                if let Some(element) = element {
                    element.detach(&mut self.document).expect("Failed to remove element");
                }
            }
        }
        $(#[$get_docs])*
        $(#[$shared_docs])*
        pub fn $get(&self) -> Option<String> {
            let root = self.root();
            let element = root.find(&self.document, $name);
            return element.map(|x| x.text_content(&self.document));
        }

    };
    [
        $(
            $(#[$shared_docs:meta])*
            $name:literal {
                $(#[$set_docs:meta])*
                set: $set:ident,
                $(#[$get_docs:meta])*
                get: $get:ident,
            }
        ),*
    ] => {
        $(

            simple_type_getter_setter! {
                $(#[$shared_docs])*
                $name {
                    $(#[$set_docs])*
                    set: $set,
                    $(#[$get_docs])*
                    get: $get,
                }
            }
        )*
    };

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
            let root = self.root();
            root.find(&self.document, $element_name)
                .map(|x| $structured_type::from_element(x, &self.document))
                .transpose()
        }
        $(#[$set_docs])*
        pub fn $set<U>(&mut self, value: U) -> Result<(), XMLEditorError>
        where
            U: Into<Option<$structured_type>> {
            let value: Option<$structured_type> = value.into();
            let root = self.root();
            let existing_element = root.find(&self.document, $element_name);
            if let Some(value) = value{
                if let Some(element) = existing_element {
                    value.update_element(element, &mut self.document)?;
                }
                let new_element = value.into_element(&mut self.document)?;
                root.push_child(&mut self.document, new_element)?;
            }else{
                if let Some(element) = existing_element {
                    element.detach(&mut self.document)?;
                }
            }

            Ok(())
        }
    };
}

macro_rules! list_item_getter_and_add {
    (
        $(#[$get_docs:meta])*
        get: $get:ident,
        $(#[$add_docs:meta])*
        add: $add:ident,
        $(#[$clear_docs:meta])*
        clear: $clear:ident,
        $parent:literal => $list_element:ident
    ) => {
        $(#[$get_docs])*
        pub fn $get(&self) -> Result<Vec<$list_element>, XMLEditorError> {
            let root = self.root();
            let Some(parent_element) = root.find(&self.document, $parent)
            else {
                return Ok(vec![]);
            };
            let result =
                get_all_children_of_element::<$list_element>(&self.document, parent_element)?;
            Ok(result.into_iter().map(|(v, _)| v).collect())
        }
        $(#[$add_docs])*
        pub fn $add(
            &mut self,
            value: $list_element,
        ) -> Result<Option<$list_element>, XMLEditorError> {
            let root = self.root();
            let parent_element = root.find(&self.document, $parent);
            add_or_update_item(&mut self.document, parent_element, root, value)
        }
        $(#[$clear_docs])*
        pub fn  $clear(&mut self)-> Result<(), XMLEditorError> {
            let root = self.root();
            let parent_element = root.find(&self.document, $parent);
            if let Some(parent_element) = parent_element {
                parent_element.clear_children(&mut self.document);
            }
            Ok(())
        }
    };
}
impl PomEditor {
    /// Creates a new [PomEditor] with the group id and artifact id set
    pub fn new_with_group_and_artifact(group_id: &str, artifact_id: &str) -> Self {
        let mut editor = Self::default();
        editor.set_group_id(group_id);
        editor.set_artifact_id(artifact_id);
        editor
    }
    top_level_structured_type!(
        /// Sets the parent of the pom file
        ///
        /// If [None] is passed in. The parent element will be removed
        set: set_parent,
        /// Gets the parent of the pom file
        get: get_parent,
        "parent" => Parent,
    );
    top_level_structured_type!(
        /// Sets the scm of the pom file
        ///
        /// If [None] is passed in. The scm element will be removed
        set: set_scm,
        /// Gets the scm of the pom file
        get: get_scm,
        "scm" => Scm,
    );
    simple_type_getter_setter![
        /// The group id of the pom
        ///
        /// [More Info](https://maven.apache.org/pom.html#maven-coordinates)
        /// Example Usage:
        /// ```rust
        /// use maven_rs::pom::editor::PomEditor;
        /// let mut editor = PomEditor::default();
        /// editor.set_group_id("dev.wyatt-herkamp");
        /// assert_eq!(editor.get_group_id(), Some("dev.wyatt-herkamp".to_string()));
        /// ```
        "groupId" {
            /// Sets the group Id in the pom file. For the maven project
            set: set_group_id,
            get: get_group_id,
        },
        /// The artifact id of the pom
        /// [More Info](https://maven.apache.org/pom.html#maven-coordinates)
        ///
        /// Example Usage:
        /// ```rust
        /// use maven_rs::pom::editor::PomEditor;
        /// let mut editor = PomEditor::default();
        /// editor.set_artifact_id("test");
        /// assert_eq!(editor.get_artifact_id(), Some("test".to_string()));
        /// ```
        "artifactId" {
            set: set_artifact_id,
            get: get_artifact_id,
        },
        /// The version of the project file
        /// [More Info](https://maven.apache.org/pom.html#maven-coordinates)
        ///
        /// Example Usage:
        /// ```rust
        /// use maven_rs::pom::editor::PomEditor;
        /// let mut editor = PomEditor::default();
        /// editor.set_version("1.0.0");
        /// assert_eq!(editor.get_version(), Some("1.0.0".to_string()));
        /// ```
        "version" {
            set: set_version,
            get: get_version,
        },
        /// The name of the maven project
        ///
        /// [More Info](https://maven.apache.org/pom.html#More_Project_Information)
        "name" {
            set: set_name,
            get: get_name,
        },
        /// The description of the maven project
        ///
        /// [More Info](https://maven.apache.org/pom.html#More_Project_Information)
        "description" {
            set: set_description,
            get: get_description,
        },
        /// [More Info](https://maven.apache.org/pom.html#More_Project_Information)
        "url" {
            set: set_url,
            get: get_url,
        },
        /// The inception year of the project
        "inceptionYear" {
            set: set_inception_year,
            get: get_inception_year,
        },
        /// Sets the model version of the pom file
        ///
        /// The model version is currently always 4.0.0
        "modelVersion" {
            /// Sets the model version of the pom file
            set: set_model_version,
            /// Gets the model version of the pom file
            get: get_model_version,
        },
        /// [More Info](https://maven.apache.org/pom.html#Packaging)
        "packaging" {
            set: set_packaging,
            get: get_packaging,
        }
    ];

    list_item_getter_and_add!(
        /// Gets all the repositories in the pom file
        /// ```rust
        /// use maven_rs::pom::editor::PomEditor;
        /// use maven_rs::pom::Repository;
        /// let xml = r#"
        /// <project>
        ///   <repositories>
        ///    <repository>
        ///      <id>central</id>
        ///      <name>Maven Central Repository</name>
        ///      <url>https://repo.maven.apache.org/maven2</url>
        ///   </repository>
        /// </repositories>
        /// </project>
        /// "#;
        /// let editor = PomEditor::load_from_str(xml).unwrap();
        /// let repositories = editor.get_repositories().unwrap();
        /// assert_eq!(repositories.len(), 1);
        /// assert_eq!(repositories[0].id, Some("central".to_string()));
        get: get_repositories,
        /// Adds or Updates a repository in the pom file
        /// ```rust
        /// use maven_rs::pom::editor::PomEditor;
        /// use maven_rs::pom::Repository;
        /// let mut editor = PomEditor::default();
        /// editor.add_or_update_repository(Repository {
        ///   id: Some("central".to_string()),
        ///   name: Some("Maven Central Repository".to_string()),
        ///   url: "https://repo.maven.apache.org/maven2".to_string(),
        ///   ..Default::default()
        /// }).unwrap();
        /// let repositories = editor.get_repositories().unwrap();
        /// assert_eq!(repositories.len(), 1);
        /// assert_eq!(repositories[0].id, Some("central".to_string()));
        /// ```
        add: add_or_update_repository,
        /// Clears all the repositories in the pom file
        clear: clear_repositories,
        "repositories" => Repository
    );
    list_item_getter_and_add!(
        /// Gets all the developers in the pom file
        /// ```rust
        /// use maven_rs::pom::editor::PomEditor;
        /// use maven_rs::pom::Developer;
        /// let xml = r#"
        /// <project>
        ///    <developers>
        ///       <developer>
        ///         <id>dev.wyatt-herkamp</id>
        ///         <name>Wyatt Herkamp</name>
        ///        </developer>
        ///   </developers>
        /// </project>
        /// "#;
        /// let editor = PomEditor::load_from_str(xml).unwrap();
        /// let developers = editor.get_developers().unwrap();
        /// assert_eq!(developers.len(), 1);
        /// assert_eq!(developers[0].id, Some("dev.wyatt-herkamp".to_string()));
        /// ```
        get: get_developers,
        /// Adds or Updates a developer in the pom file
        /// ```rust
        /// use maven_rs::pom::editor::PomEditor;
        /// use maven_rs::pom::Developer;
        /// let mut editor = PomEditor::default();
        /// editor.add_or_update_developer(Developer {
        ///    id: Some("dev.wyatt-herkamp".to_string()),
        ///    name: Some("Wyatt Herkamp".to_string()),
        ///    ..Default::default()
        /// }).unwrap();
        /// let developers = editor.get_developers().unwrap();
        /// assert_eq!(developers.len(), 1);
        /// assert_eq!(developers[0].id, Some("dev.wyatt-herkamp".to_string()));
        /// ```
        add: add_or_update_developer,
        /// Clears all the developers in the pom file
        clear: clear_developers,
        "developers" => Developer
    );
    list_item_getter_and_add!(
        /// Gets all the dependencies in the pom file
        /// ```rust
        /// use maven_rs::pom::editor::PomEditor;
        /// use maven_rs::pom::Dependency;
        /// let xml = r#"
        /// <project>
        ///  <dependencies>
        ///   <dependency>
        ///    <groupId>com.google.guava</groupId>
        ///    <artifactId>guava</artifactId>
        ///    <version>30.1-jre</version>
        /// </dependency>
        /// </dependencies>
        /// </project>
        /// "#;
        /// let editor = PomEditor::load_from_str(xml).unwrap();
        /// let dependencies = editor.get_dependencies().unwrap();
        /// assert_eq!(dependencies.len(), 1);
        /// assert_eq!(dependencies[0].group_id, "com.google.guava".to_string());
        /// ```
        get: get_dependencies,
        /// Adds or Updates a dependency in the pom file
        ///
        /// ```rust
        /// use maven_rs::pom::editor::PomEditor;
        /// use maven_rs::pom::Dependency;
        /// let mut editor = PomEditor::default();
        /// editor.add_or_update_dependency(Dependency {
        ///  group_id: "com.google.guava".to_string(),
        /// artifact_id: "guava".to_string(),
        /// version: Some("30.1-jre".parse().unwrap()),
        /// ..Default::default()
        /// }).unwrap();
        /// let dependencies = editor.get_dependencies().unwrap();
        /// assert_eq!(dependencies.len(), 1);
        /// assert_eq!(dependencies[0].group_id, "com.google.guava".to_string());
        /// ```
        add: add_or_update_dependency,
        /// Clears all the dependencies in the pom file
        clear: clear_dependencies,
        "dependencies" => Dependency
    );
    // TODO:  pluginRepositories
    // Loads a pom from a string
    pub fn load_from_str(value: &str) -> Result<Self, XMLEditorError> {
        let document = Document::parse_str_with_opts(
            value,
            ReadOptions {
                require_decl: false,
                ..Default::default()
            },
        )?;
        Self::assert_requirements_for_pom(&document)?;
        Ok(Self {
            document,
            ident_level: 2,
        })
    }
    /// Loads a pom from a reader
    ///
    /// # Errors
    /// If the xml is not a valid pom file
    pub fn load_from_reader<R: std::io::Read>(reader: R) -> Result<Self, XMLEditorError> {
        let document = Document::parse_reader_with_opts(
            reader,
            ReadOptions {
                require_decl: false,
                ..Default::default()
            },
        )?;
        Self::assert_requirements_for_pom(&document)?;
        Ok(Self {
            document,
            ident_level: 2,
        })
    }

    /// Assets that the document has an root element of project
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

    pub(crate) fn root(&self) -> Element {
        self.document.root_element().unwrap()
    }
    pub fn write_to_str(&self) -> Result<String, XMLEditorError> {
        self.document
            .write_str_with_opts(WriteOptions {
                write_decl: true,
                indent_size: self.ident_level,
                ..Default::default()
            })
            .map_err(XMLEditorError::from)
    }
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), XMLEditorError> {
        self.document
            .write_with_opts(
                writer,
                WriteOptions {
                    write_decl: true,
                    indent_size: self.ident_level,
                    ..Default::default()
                },
            )
            .map_err(XMLEditorError::from)
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
            version: Some("30.1-jre".parse().unwrap()),
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
