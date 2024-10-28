# Maven-RS
[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)
[![forthebadge](https://forthebadge.com/images/badges/powered-by-coffee.svg)](https://forthebadge.com)
[![CLI](https://github.com/wyatt-herkamp/maven-rs/actions/workflows/check.yml/badge.svg)](https://github.com/wyatt-herkamp/maven-rs/actions/workflows/check.yml)
[Docs](https://wyatt-herkamp.github.io/maven-rs/maven_rs/index.html)

the Maven Helper library for Rust that no one asked for

## Examples

### Reading a pom file with serde
```rust
let buf = PathBuf::from("my_pom.xml");
let file = File::open(buf).unwrap();
let pom_file: Pom = maven_rs::quick_xml::de::from_reader(BufReader::new(file)).unwrap();
println!("{:#?}", x);
```

### Generating a pom file with the editor
```rust
        let mut editor = PomEditor::default();
        editor.set_group_id("dev.wyatt-herkamp");
        editor.set_artifact_id("test");
        let value = editor.write_to_str()?;
        println!("{}", value);
        let mut new_editor = PomEditor::load_from_str(value.as_str())?;

        // Make sure the group id and artifact id are correct
        assert_eq!(
            new_editor.get_group_id(),
            Some("dev.wyatt-herkamp".to_string())
        );
        assert_eq!(new_editor.get_artifact_id(), Some("test".to_string()));
        // Try Changing the group id and artifact id
        new_editor.set_group_id("dev.wyatt-herkamp2");
        new_editor.set_artifact_id("test2");
        assert_eq!(
            new_editor.get_group_id(),
            Some("dev.wyatt-herkamp2".to_string())
        );
        assert_eq!(new_editor.get_artifact_id(), Some("test2".to_string()));
        let value = new_editor.write_to_str()?;
        println!("{}", value);
```
Read through the tests and the examples to see how to use the library.

### Scope

The primary purpose of this project is to provide interaction with Maven pom files for [nitro_repo](https://github.com/wyatt-herkamp/nitro_repo). However, this library can be used by anyone who is interested. Feel free to contribute to the project if you need a feature that is not currently implemented.

