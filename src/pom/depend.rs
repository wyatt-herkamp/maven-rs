use std::str::FromStr;
#[cfg(feature = "resolver")]
pub mod resolve;
use crate::{
    editor::{
        utils::{
            add_if_present, create_basic_text_element, find_or_create_then_set_text_content,
            sync_element, typed_from_element_using_builder,
        },
        ChildOfListElement, ComparableElement, ElementConverter, HasElementName, UpdatableElement,
        XMLEditorError,
    },
    types::Property,
    utils::group_id_and_artifact_id_and_version_to_path,
};
use derive_builder::Builder;
use edit_xml::{Document, Element};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Dependencies {
    #[serde(default, rename = "dependency")]
    pub dependencies: Vec<Dependency>,
}
#[derive(Debug, Error)]
pub enum DependencyParseError {
    #[error("Missing artifact id")]
    MissingArtifactId,
    #[error("Missing Version")]
    MissingVersion,
    #[error("Missing Separator")]
    MissingSeparator,
}
/// A dependency in a pom file.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    /// The group id of the dependency.
    /// ```xml
    /// <groupId>com.google.guava</groupId>
    /// ```
    #[builder(setter(into))]
    pub group_id: String,
    /// The artifact id of the dependency.
    /// ```xml
    /// <artifactId>guava</artifactId>
    /// ```
    #[builder(setter(into))]
    pub artifact_id: String,
    /// The version of the dependency.
    ///
    /// ```xml
    /// <version>1.0.0</version>
    /// ```
    #[builder(default, setter(into, strip_option))]
    pub version: Option<Property>,
    /// The type of the dependency.
    /// ```xml
    /// <type>jar</type>
    /// ```
    #[builder(default, setter(into, strip_option))]
    #[serde(rename = "type")]
    pub depend_type: Option<String>,
    #[builder(default, setter(into, strip_option))]
    pub scope: Option<String>,
    #[builder(default, setter(into, strip_option))]
    pub classifier: Option<String>,
}

impl Dependency {
    /// Checks if the dependency is the same as the other dependency.
    ///
    /// Basically, it checks if the group id and artifact id are the same.
    pub fn is_same_dependency(&self, other: &Dependency) -> bool {
        self.group_id == other.group_id && self.artifact_id == other.artifact_id
    }

    pub fn pom_name(&self) -> String {
        let version = self.version.clone().unwrap_or_default();
        format!("{}-{}.pom", self.artifact_id, version)
    }
    pub fn pom_path(&self) -> String {
        let version = self.version.clone().unwrap_or_default();

        let path = group_id_and_artifact_id_and_version_to_path(
            &self.group_id,
            &self.artifact_id,
            &version.to_string(),
        );
        format!("{}/{}", path, self.pom_name())
    }
}
impl ChildOfListElement for Dependency {
    fn parent_element_name() -> &'static str {
        "dependencies"
    }
}
impl ComparableElement for Dependency {
    fn is_same_item(&self, other: &Self) -> bool {
        self.is_same_dependency(other)
    }
}
impl UpdatableElement for Dependency {
    fn update_element(
        &self,
        element: Element,
        document: &mut Document,
    ) -> Result<(), XMLEditorError> {
        sync_element(
            document,
            element,
            "version",
            self.version.as_ref().map(|v| v.to_string()),
        );
        if let Some(depend_type) = &self.depend_type {
            find_or_create_then_set_text_content(document, element, "type", depend_type);
        }
        if let Some(scope) = &self.scope {
            find_or_create_then_set_text_content(document, element, "scope", scope);
        }
        if let Some(classifier) = &self.classifier {
            find_or_create_then_set_text_content(document, element, "classifier", classifier);
        }
        Ok(())
    }
}
impl TryFrom<&str> for Dependency {
    type Error = DependencyParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() || !value.contains(":") {
            return Err(DependencyParseError::MissingSeparator);
        }
        let parts: Vec<&str> = value.split(':').collect();
        let group_id = parts.first().unwrap().to_string();
        let artifact_id = parts
            .get(1)
            .ok_or(DependencyParseError::MissingArtifactId)?
            .to_string();
        let version = parts
            .get(2)
            .ok_or(DependencyParseError::MissingVersion)?
            .to_string();
        let version = Property::Literal(version);
        // TODO: Add support for type, scope, and classifier.

        let result = Dependency {
            group_id,
            artifact_id,
            version: Some(version),
            depend_type: None,
            scope: None,
            classifier: None,
        };
        Ok(result)
    }
}
impl TryFrom<String> for Dependency {
    type Error = DependencyParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Dependency::try_from(value.as_str())
    }
}
impl FromStr for Dependency {
    type Err = DependencyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Dependency::try_from(s)
    }
}

impl std::fmt::Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let version = self.version.clone().unwrap_or_default();
        write!(f, "{}:{}:{}", self.group_id, self.artifact_id, version)
    }
}
impl HasElementName for Dependency {
    fn element_name() -> &'static str {
        "dependency"
    }
}
impl ElementConverter for Dependency {
    typed_from_element_using_builder!(
        DependencyBuilder,
        element,
        document,
        "groupId"(String) => group_id,
        "artifactId"(String) => artifact_id,
        "version"(Property) => version,
        "type"(String) => depend_type,
        "scope"(String) => scope,
        "classifier"(String) => classifier
    );
    fn into_children(self, document: &mut Document) -> Result<Vec<Element>, XMLEditorError> {
        let Self {
            group_id,
            artifact_id,
            version,
            depend_type,
            scope,
            classifier,
        } = self;

        let mut children = vec![
            create_basic_text_element(document, "groupId", group_id),
            create_basic_text_element(document, "artifactId", artifact_id),
        ];
        add_if_present!(document, children, version, "version");
        add_if_present!(document, children, depend_type, "type");
        add_if_present!(document, children, scope, "scope");
        add_if_present!(document, children, classifier, "classifier");

        Ok(children)
    }
}

#[cfg(test)]
mod tests {
    use std::{fmt::Display, path::PathBuf};

    use pretty_assertions::assert_eq;

    use crate::{
        editor::utils::test_utils,
        utils::bug_testing::{self, BugFile},
    };

    pub use super::*;
    #[test]
    fn test_simple() {
        let dep = Dependency {
            group_id: "com.google.guava".to_string(),
            artifact_id: "guava".to_string(),
            version: Some("30.1-jre".parse().unwrap()),
            depend_type: None,
            scope: None,
            classifier: None,
        };
        let dep_str = "com.google.guava:guava:30.1-jre";
        assert_eq!(dep, Dependency::try_from(dep_str).unwrap());
        assert_eq!(dep_str, dep.to_string());
    }
    #[test]
    pub fn test_is_same_dependency() {
        let dep = Dependency {
            group_id: "com.google.guava".to_string(),
            artifact_id: "guava".to_string(),
            version: Some("30.1-jre".parse().unwrap()),
            depend_type: None,
            scope: None,
            classifier: None,
        };
        let dep2 = Dependency {
            group_id: "com.google.guava".to_string(),
            artifact_id: "guava".to_string(),
            version: Some("30.2-jre".parse().unwrap()),
            depend_type: None,
            scope: None,
            classifier: None,
        };
        assert!(
            dep.is_same_dependency(&dep2),
            "Dependencies should be the same. Because the group id and artifact id are the same."
        );
    }
    fn test_parse_methods(value: &str, expected: Dependency) -> anyhow::Result<()> {
        let dep_via_edit_xml = test_utils::create_xml_to_element::<Dependency>(value)?;
        let dep_via_serde: Dependency = quick_xml::de::from_str(value)?;

        assert_eq!(dep_via_edit_xml, expected);
        assert_eq!(dep_via_serde, expected);
        println!("{:#?}", dep_via_edit_xml);

        let dep_serialize_serde = quick_xml::se::to_string(&expected)?;
        println!("Serialized Over Serde \n {}", dep_serialize_serde);
        Ok(())
    }
    #[test]
    pub fn parse_full() -> anyhow::Result<()> {
        let test_value = r#"
            <dependency>
                <groupId>com.google.guava</groupId>
                <artifactId>guava</artifactId>
                <version>30.1-jre</version>
                <type>jar</type>
                <scope>compile</scope>
                <classifier>tests</classifier>
            </dependency>
        "#;
        test_parse_methods(
            test_value,
            Dependency {
                group_id: "com.google.guava".to_string(),
                artifact_id: "guava".to_string(),
                version: Some("30.1-jre".parse().unwrap()),
                depend_type: Some("jar".to_string()),
                scope: Some("compile".to_string()),
                classifier: Some("tests".to_string()),
            },
        )?;
        Ok(())
    }

    #[test]
    pub fn parse_min() -> anyhow::Result<()> {
        let test_value = r#"
            <dependency>
                <groupId>com.google.guava</groupId>
                <artifactId>guava</artifactId>
                <version>30.1-jre</version>

            </dependency>
        "#;
        test_parse_methods(
            test_value,
            Dependency {
                group_id: "com.google.guava".to_string(),
                artifact_id: "guava".to_string(),
                version: Some("30.1-jre".parse().unwrap()),
                ..Default::default()
            },
        )?;
        Ok(())
    }
    #[test]
    pub fn parse_no_version() -> anyhow::Result<()> {
        let test_value = r#"
            <dependency>
                <groupId>com.google.guava</groupId>
                <artifactId>guava</artifactId>
            </dependency>
        "#;
        test_parse_methods(
            test_value,
            Dependency {
                group_id: "com.google.guava".to_string(),
                artifact_id: "guava".to_string(),
                version: None,
                ..Default::default()
            },
        )?;
        Ok(())
    }

    #[test]
    pub fn test_found_bugs() -> anyhow::Result<()> {
        let depend_bugs_dir = bug_testing::get_bugs_path().join("depends");
        let depend_bugs = depend_bugs_dir.read_dir()?;
        for bug in depend_bugs {
            let bug = bug?;
            let bug_path = bug.path();
            let bug_file = std::fs::read_to_string(&bug_path)?;
            let bug: BugFile = toml::de::from_str(&bug_file)?;
            if !bug.depends.is_empty() {
                println!("Bug File: \n {}", bug.source);
                println!("Error: {}", bug.error);
                for found_bug in bug.depends {
                    println!("Testing Dependency: {:?}", found_bug.expected);
                    let expected_depends: Dependency = found_bug.expected.into();
                    println!("Expected Dependency: {}", expected_depends);
                    test_parse_methods(&found_bug.xml, expected_depends.clone())?;
                }
            }
        }
        Ok(())
    }
}
