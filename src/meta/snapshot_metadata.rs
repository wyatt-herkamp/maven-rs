use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::extension::MavenFileExtension;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SnapshotMetadata {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub version: String,
    pub versioning: SnapshotVersioning,
}

impl SnapshotMetadata {
    /// Returns None if the version is not found in the metadata.
    pub fn get_latest_artifact_name(
        &self,
        extension: impl Into<MavenFileExtension>,
    ) -> Option<String> {
        let extension = extension.into();
        if let Some(ref value) = self.versioning.snapshot_versions {
            let filter = value.snapshot_version.iter().find(|x| (*x).eq(&extension));
            return if let Some(value) = filter {
                let name = format!("{}-{}{}", self.artifact_id, value.value, extension);
                Some(name)
            } else {
                None
            };
        }
        None
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotVersioning {
    pub snapshot: Option<Snapshot>,
    pub snapshot_versions: Option<SnapshotVersions>,
    #[serde(with = "crate::utils::time::standard_time")]
    pub last_updated: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SnapshotVersions {
    #[serde(rename = "snapshotVersion")]
    pub snapshot_version: Vec<SnapshotVersion>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    #[serde(with = "crate::utils::time::snapshot_time")]
    pub timestamp: Option<NaiveDateTime>,
    pub build_number: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SnapshotVersion {
    pub classifier: Option<String>,
    #[serde(default)]
    pub extension: String,
    pub value: String,
    #[serde(with = "crate::utils::time::standard_time")]
    pub updated: Option<NaiveDateTime>,
}

impl PartialEq<MavenFileExtension> for SnapshotVersion {
    fn eq(&self, other: &MavenFileExtension) -> bool {
        self.extension.eq(&other.file_extension) && self.classifier.eq(&other.classifier)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    #[test]
    pub fn parse_snapshot() {
        let test = r#"
            <snapshot>
                <timestamp>20210101.010101</timestamp>
                <buildNumber>1</buildNumber>
            </snapshot>
        "#;
        let snapshot: super::Snapshot = quick_xml::de::from_str(test).unwrap();
        assert_eq!(
            snapshot.timestamp.unwrap().to_string(),
            "2021-01-01 01:01:01"
        );
        assert_eq!(snapshot.build_number, "1");
    }
}
