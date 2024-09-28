use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::extension::MavenFileExtension;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeployMetadata {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub versioning: StableVersioning,
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
pub struct StableVersioning {
    pub release: Option<String>,
    pub latest: Option<String>,
    pub versions: StableVersions,
    #[serde(rename = "lastUpdated", with = "crate::utils::time::standard_time")]
    pub last_updated: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StableVersions {
    pub version: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    pub fn parse_metadata() {
        let metadata = r#"
        <metadata>
            <groupId>org.kakara</groupId>
            <artifactId>engine</artifactId>
            <versioning>
                <latest>1.0-SNAPSHOT</latest>
                <versions>
                    <version>1.0-SNAPSHOT</version>
                </versions>
                <lastUpdated>20220826191631</lastUpdated>
            </versioning>
        </metadata>
        "#;
        let metadata: DeployMetadata = quick_xml::de::from_str(metadata).unwrap();
        assert_eq!(metadata.group_id, "org.kakara");
        assert_eq!(metadata.artifact_id, "engine");
        assert_eq!(metadata.versioning.latest, Some("1.0-SNAPSHOT".to_string()));
        assert_eq!(
            metadata.versioning.versions.version,
            vec!["1.0-SNAPSHOT".to_string()]
        );
    }
}
