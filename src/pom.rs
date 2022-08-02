use std::iter::Map;
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Developers {
    pub developer: Vec<Developer>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Developer {
    pub id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scm {
    pub url: Option<String>,
    pub connection: Option<String>,
    #[serde(rename = "developerConnection")]
    pub developer_connection: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pom {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub version: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub scm: Option<Scm>,
}

