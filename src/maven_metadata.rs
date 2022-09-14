use chrono::{DateTime, Utc};
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
#[cfg(test)]
pub mod test{
    use std::io::BufReader;
    use std::path::{Path, PathBuf};
    use crate::MANIFEST;
    use crate::maven_metadata::DeployMetadata;

    #[test]
    pub fn load_kakara_engine_metadata() {
        let buf = PathBuf::from(MANIFEST).join("tests").join("data").join("kakara-engine").join("maven-metadata.xml");
        if !buf.exists(){
            panic!("Test file not found");
        }
        let file = std::fs::File::open(buf).unwrap();
        let x: DeployMetadata = quick_xml::de::from_reader(BufReader::new(file)).unwrap();
        println!("{:#?}", x);
    }

}