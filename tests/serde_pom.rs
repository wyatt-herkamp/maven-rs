use std::path::PathBuf;

fn test_poms() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("test_poms")
}

#[test]
pub fn lwjgl_bom_file() -> anyhow::Result<()> {
    let file_path = test_poms().join("lwjgl-bom-3.3.4.pom");
    let reader = std::fs::File::open(file_path)?;
    let pom = quick_xml::de::from_reader::<_, maven_rs::pom::Pom>(std::io::BufReader::new(reader))?;

    assert_eq!(pom.group_id, Some("org.lwjgl".to_string()));
    assert_eq!(pom.artifact_id, "lwjgl-bom".to_string());
    assert_eq!(pom.version, Some("3.3.4".to_string()));
    Ok(())
}
#[test]
pub fn gson_bom() -> anyhow::Result<()> {
    let file_path = test_poms().join("gson-2.11.0.pom");
    let reader = std::fs::File::open(file_path)?;
    let pom = quick_xml::de::from_reader::<_, maven_rs::pom::Pom>(std::io::BufReader::new(reader))?;

    assert_eq!(pom.get_group_id(), Some("com.google.code.gson"));
    assert_eq!(pom.artifact_id, "gson".to_string());
    assert_eq!(pom.get_version(), Some("2.11.0"));
    Ok(())
}
