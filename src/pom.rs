//! The structure and functions to work with a pom file.
use serde::{Deserialize, Serialize};
mod build;
mod depend;
mod developers;
mod distribution_management;
pub mod editor;
mod parent;
mod properties;
mod repositories;
mod scm;
pub use build::*;
pub use depend::*;
pub use developers::*;
pub use distribution_management::*;
pub use parent::*;
pub use properties::*;
pub use repositories::*;
pub use scm::*;

/// Represents a pom file.
///
/// This structure is used with Serde to deserialize a pom file.
///
/// # Example
/// ```
/// use maven_rs::pom::Pom;
/// const EXAMPLE_POM: &str = r#"
///    <?xml version="1.0" encoding="UTF-8"?>
///    <project>
///         <modelVersion>4.0.0</modelVersion>
///         <groupId>org.apache.maven</groupId>
///         <artifactId>maven-artifact</artifactId>
///         <version>3.0</version>
///         <name>Apache Maven Artifact</name>
///    </project>
/// "#;
///
/// let x: Pom = maven_rs::quick_xml::de::from_str(EXAMPLE_POM).unwrap();
/// println!("{:#?}", x);
/// assert_eq!(x.group_id, Some("org.apache.maven".to_string()));
/// assert_eq!(x.artifact_id, "maven-artifact".to_string());
/// assert_eq!(x.version, Some("3.0".to_string()));
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pom {
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub parent: Option<Parent>,
    pub version: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub scm: Option<Scm>,
}
impl Pom {
    /// Gets the group id of the pom.
    /// If the group id is not present, it will attempt to get the group id from the parent.
    /// If the parent does not have a group id, it will return None.
    pub fn get_group_id(&self) -> Option<&str> {
        self.group_id
            .as_deref()
            .or(self.parent.as_ref().and_then(|x| x.group_id.as_deref()))
    }
    pub fn get_version(&self) -> Option<&str> {
        self.version
            .as_deref()
            .or(self.parent.as_ref().and_then(|x| x.version.as_deref()))
    }
}

#[cfg(test)]
pub mod tests {
    use anyhow::Context;

    use crate::pom::Pom;

    #[test]
    pub fn test_version_or_group_id_in_parent() -> anyhow::Result<()> {
        const EXAMPLE_POM: &str = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <project>
            <modelVersion>4.0.0</modelVersion>
            <parent>
                <groupId>com.google.code.gson</groupId>
                <artifactId>gson-parent</artifactId>
                <version>2.11.0</version>
            </parent>

            <artifactId>gson</artifactId>
            <name>Gson</name>
        </project>
        "#;
        let pom: Pom = quick_xml::de::from_str(EXAMPLE_POM).context("Unable to Parse Test Pom")?;
        assert_eq!(pom.get_group_id(), Some("com.google.code.gson"));
        assert_eq!(pom.get_version(), Some("2.11.0"));
        assert_eq!(pom.artifact_id, "gson");
        Ok(())
    }
}
