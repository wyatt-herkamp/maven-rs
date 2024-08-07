use crate::MavenFileExtension;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SnapshotMetadata {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub version: String,
    pub versioning: Versioning,
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
pub struct Versioning {
    pub snapshot: Option<Snapshot>,
    pub snapshot_versions: Option<SnapshotVersions>,
    #[serde(with = "crate::time::standard_time")]
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
    #[serde(with = "crate::time::snapshot_time")]
    pub timestamp: Option<NaiveDateTime>,
    pub build_number: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SnapshotVersion {
    pub classifier: Option<String>,
    #[serde(default)]
    pub extension: String,
    pub value: String,
    #[serde(with = "crate::time::standard_time")]
    pub updated: Option<NaiveDateTime>,
}

impl PartialEq<MavenFileExtension> for SnapshotVersion {
    fn eq(&self, other: &MavenFileExtension) -> bool {
        self.extension.eq(&other.file_extension) && self.classifier.eq(&other.classifier)
    }
}

#[cfg(test)]
pub mod test {
    use crate::snapshot_metadata::SnapshotMetadata;
    use crate::{MavenFileExtension, MANIFEST};
    use std::io::BufReader;
    use std::path::PathBuf;

    #[test]
    pub fn load_kakara_engine_metadata() {
        let buf = PathBuf::from(MANIFEST)
            .join("tests")
            .join("data")
            .join("kakara-engine")
            .join("snapshot.xml");
        if !buf.exists() {
            panic!("Test file not found");
        }
        let file = std::fs::File::open(buf).unwrap();
        let snapshot_meta: SnapshotMetadata =
            quick_xml::de::from_reader(BufReader::new(file)).unwrap();
        if let Some(value) = snapshot_meta.get_latest_artifact_name("jar") {
            println!("{}", value);
        }
        let extension = MavenFileExtension {
            hash: Some("sha256".to_owned()),
            file_extension: "jar".to_owned(),
            classifier: Some("sources".to_owned()),
        };
        if let Some(value) = snapshot_meta.get_latest_artifact_name(extension) {
            println!("{}", value);
        }
        if let Some(value) = snapshot_meta
            .get_latest_artifact_name(MavenFileExtension::from("jar").with_classifier("javadoc"))
        {
            println!("{}", value);
        }

        println!("{:#?}", snapshot_meta);
    }
}
