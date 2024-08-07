use crate::MavenFileExtension;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeployMetadata {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub versioning: Versioning,
}

impl DeployMetadata {
    /// Attempts to pull latest
    /// Then attempts to pull release
    /// Then attempts te first version in the list
    #[inline]
    pub fn get_latest_version(&self) -> Option<&String> {
        self.versioning
            .latest
            .as_ref()
            .or(self.versioning.release.as_ref())
            .or_else(|| self.versioning.versions.version.first())
    }
    /// Returns a tuple of the latest version and the artifact name.
    pub fn get_latest_artifact_name(
        &self,
        extension: impl Into<MavenFileExtension>,
    ) -> Option<(&str, String)> {
        if let Some(value) = self.get_latest_version() {
            let string = self.get_artifact_name(value, extension);
            Some((value, string))
        } else {
            None
        }
    }
    #[inline]
    pub fn get_artifact_name(
        &self,
        version: &str,
        extension: impl Into<MavenFileExtension>,
    ) -> String {
        let extension = extension.into();
        format!("{}-{}{}", self.artifact_id, version, extension)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Versioning {
    pub release: Option<String>,
    pub latest: Option<String>,
    pub versions: Versions,
    #[serde(rename = "lastUpdated", with = "crate::time::standard_time")]
    pub last_updated: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Versions {
    pub version: Vec<String>,
}

#[cfg(test)]
pub mod test {
    use crate::maven_metadata::DeployMetadata;
    use crate::MANIFEST;
    use std::io::BufReader;
    use std::path::PathBuf;

    #[test]
    pub fn load_kakara_engine_metadata() {
        let buf = PathBuf::from(MANIFEST)
            .join("tests")
            .join("data")
            .join("kakara-engine")
            .join("maven-metadata.xml");
        if !buf.exists() {
            panic!("Test file not found");
        }
        let file = std::fs::File::open(buf).unwrap();
        let x: DeployMetadata = quick_xml::de::from_reader(BufReader::new(file)).unwrap();
        println!("{:#?}", x);
    }
}
