use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeployMetadata {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub versioning: Versioning,

}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Versioning {
    pub release: Option<String>,
    pub latest: Option<String>,
    pub versions: Versions,
    #[serde(rename = "lastUpdated", with = "crate::time::standard_time")]
    pub last_updated: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Versions {
    pub version: Vec<String>,
}
