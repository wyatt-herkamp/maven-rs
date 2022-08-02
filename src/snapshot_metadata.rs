use chrono::{DateTime, Utc};
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Versioning {
    pub snapshot: Option<Snapshot>,
    #[serde(rename = "snapshotVersions")]
    pub snapshot_versions: Option<Vec<SnapshotVersion>>,
    #[serde(rename = "lastUpdated", with = "crate::time::snapshot_time")]
    pub last_updated: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Snapshot {
    #[serde(with = "crate::time::snapshot_time")]
    pub timestamp: Option<DateTime<Utc>>,
    #[serde(rename = "buildNumber")]
    pub build_number: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SnapshotVersion {
    #[serde(default)]
    pub extension: String,
    pub value: String,
    #[serde(rename = "lastUpdated", with = "crate::time::standard_time")]
    pub last_updated: Option<DateTime<Utc>>,
}
