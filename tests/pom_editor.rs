use std::path::PathBuf;

use maven_rs::pom::{Dependency, editor::PomEditor};

#[test]
pub fn test_read_test_pom() -> anyhow::Result<()> {
    let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("test_poms")
        .join("test-pom.xml");
    let reader = std::fs::File::open(file_path)?;
    let mut editor = PomEditor::load_from_reader(reader)?;
    let dependencies = editor.get_dependencies()?;
    println!("{:#?}", dependencies);
    let mut dependency = Dependency {
        group_id: "com.google.guava".to_string(),
        artifact_id: "guava".to_string(),
        version: Some("29.1-jre".parse().unwrap()),
        depend_type: None,
        scope: None,
        classifier: None,
    };
    editor.add_or_update_dependency(dependency.clone())?;
    println!("{:#?}", editor.get_dependencies()?);

    dependency.version = Some("30.1-jre".parse().unwrap());
    editor.add_or_update_dependency(dependency.clone())?;
    println!("{:#?}", editor.get_dependencies()?);

    {
        let build = editor.get_or_create_build_element();
        let plugins = build.get_plugins()?;
        println!("{:#?}", plugins);
    }

    Ok(())
}
