//! This module contains the structure of a pom file for serde deserialization.
//!
//! ```
//! use maven_rs::pom::Pom;
//! const EXAMPLE_POM: &str = r#"
//!    <?xml version="1.0" encoding="UTF-8"?>
//!   <project xmlns="http://maven.apache.org/POM/4.0.0"
//!      xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
//!      xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
//!         <modelVersion>4.0.0</modelVersion>
//!         <groupId>org.apache.maven</groupId>
//!         <artifactId>maven-artifact</artifactId>
//!         <version>3.0</version>
//!         <name>Apache Maven Artifact</name>
//! </project>
//! "#;
//!
//! let x: Pom = maven_rs::quick_xml::de::from_str(EXAMPLE_POM).unwrap();
//! println!("{:#?}", x);
//! ```
use serde::{Deserialize, Serialize};
mod build;
mod depend;
mod developers;
pub mod editor;
mod parent;
mod properties;
mod repositories;
mod scm;
pub use build::*;
pub use depend::*;
pub use developers::*;
pub use parent::*;
pub use properties::*;
pub use repositories::*;
pub use scm::*;
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
    use crate::pom::Pom;
    use crate::MANIFEST;
    use std::io::BufReader;
    use std::path::PathBuf;

    #[test]
    pub fn test_read_local_config() {
        let buf = PathBuf::from(MANIFEST)
            .join("tests")
            .join("data")
            .join("test-pom.xml");
        if !buf.exists() {
            panic!("Test file not found");
        }
        let file = std::fs::File::open(buf).unwrap();
        let x: Pom = quick_xml::de::from_reader(BufReader::new(file)).unwrap();
        println!("{:#?}", x);
    }
    #[test]
    pub fn read_gson_pom() {
        let buf = PathBuf::from(MANIFEST)
            .join("tests")
            .join("data")
            .join("gson-2.11.0.pom");
        if !buf.exists() {
            panic!("Test file not found");
        }
        let file = std::fs::File::open(buf).unwrap();
        let x: Pom = quick_xml::de::from_reader(BufReader::new(file)).unwrap();
        println!("{:#?}", x);
    }
}
