use std::path::PathBuf;

use maven_rs::{pom::editor::PomEditor, settings::Settings};
use walkdir::{DirEntry, WalkDir};

pub fn main() -> anyhow::Result<()> {
    let walk_location = match std::env::var("MAVEN_RS_REPOSITORY_TEST") {
        Ok(ok) => PathBuf::from(ok),
        Err(_) => Settings::read_local_config()?
            .get_local_repository_or_default()
            .expect("No Local Repository Found"),
    };
    let mut files_tested = 0;
    for entry in WalkDir::new(walk_location) {
        let entry = entry?;
        if is_pom_file(&entry)? {
            files_tested += 1;
            println!("Attempting to Parse Pom File at {:?}", entry.path());
            let file_as_string = std::fs::read_to_string(entry.path())?;

            let mut editor_read = PomEditor::load_from_str(file_as_string.as_str())?;
            {
                match editor_read.get_build_element_or_none() {
                    Some(build) => {
                        let plugins = build.get_plugins()?;
                        println!("{:#?}", plugins);
                    }
                    None => {
                        println!("No Build Element Found");
                    }
                };
            }
            let dependencies = editor_read.get_dependencies()?;
            println!("{:#?}", dependencies);

            let repositories = editor_read.get_repositories()?;
            println!("{:#?}", repositories);

            let developers = editor_read.get_developers()?;
            println!("{:#?}", developers);

            println!("Parent Element: {:#?}", editor_read.get_parent()?);

            println!("SCM Element: {:#?}", editor_read.get_scm()?);
        }
    }
    println!("Files Tested: {}", files_tested);
    Ok(())
}

fn is_pom_file(entry: &DirEntry) -> anyhow::Result<bool> {
    if entry.file_type().is_file() {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "pom" {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
