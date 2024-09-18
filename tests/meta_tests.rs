use maven_rs::extension::MavenFileExtension;
use maven_rs::meta::{DeployMetadata, SnapshotMetadata};

use std::io::BufReader;
use std::path::PathBuf;
fn meta_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("meta")
}
#[test]
pub fn test_load_regular_meta() {
    let buf = meta_path().join("kakara-engine").join("maven-metadata.xml");
    if !buf.exists() {
        panic!("Test file not found");
    }
    let file = std::fs::File::open(buf).unwrap();
    let x: DeployMetadata = quick_xml::de::from_reader(BufReader::new(file)).unwrap();
    println!("{:#?}", x);
}
#[test]
pub fn test_snapshot() {
    let buf = meta_path().join("kakara-engine").join("snapshot.xml");
    if !buf.exists() {
        panic!("Test file not found");
    }
    let file = std::fs::File::open(buf).unwrap();
    let snapshot_meta: SnapshotMetadata = quick_xml::de::from_reader(BufReader::new(file)).unwrap();
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
