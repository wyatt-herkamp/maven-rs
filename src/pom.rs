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

#[cfg(all(test))]
pub mod tests {
    use std::io::BufReader;
    use std::path::PathBuf;
    use crate::pom::Pom;

    const MANIFEST: &str = env!("CARGO_MANIFEST_DIR");
    #[test]
    pub fn test_read_local_config() {
        let buf = PathBuf::from(MANIFEST).join("tests").join("data").join("test-pom.xml");
        if !buf.exists(){
            panic!("Test file not found");
        }
        let file = std::fs::File::open(buf).unwrap();
        let x: Pom = quick_xml::de::from_reader(BufReader::new(file)).unwrap();
        println!("{:#?}", x);
    }
}

