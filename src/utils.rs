pub(crate) mod parse;
pub(crate) mod serde_utils;
pub mod time;

pub fn group_id_to_path(group_id: &str) -> String {
    group_id.replace(".", "/")
}

pub fn group_id_and_artifact_id_to_path(group_id: &str, artifact_id: &str) -> String {
    format!("{}/{}", group_id_to_path(group_id), artifact_id)
}

pub fn group_id_and_artifact_id_and_version_to_path(
    group_id: &str,
    artifact_id: &str,
    version: &str,
) -> String {
    format!("{}/{}/{}", group_id_to_path(group_id), artifact_id, version)
}

/// Bug Files. Used for testing
///
/// When a bug is found a bug file is created with the section of the pom file that caused the bug.
///
/// Then the bug is added a test to ensure the bug is fixed. and can be used for future regression testing.
///
/// Files are stored in toml as they support multi line strings without escaping.
#[cfg(any(test, feature = "bug-files"))]
pub mod bug_testing {
    use std::fmt::Display;

    use serde::{Deserialize, Serialize};

    use crate::pom::Dependency;

    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    pub struct BugFile {
        pub source: Source,
        /// The error message that was found
        pub error: String,
        #[serde(default)]
        pub depends: Vec<DependBug>,
    }
    /// A dependency related to a bug
    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    pub struct DependBug {
        pub xml: String,
        pub expected: FoundBugDependency,
    }
    /// The source of the bug. Includes the group_id, artifact_id, version, and the file name.
    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    pub struct Source {
        pub group_id: String,
        pub artifact_id: String,
        pub version: String,
        pub file_name: String,
    }
    impl Display for Source {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Group ID {}\n Artifact ID {}\n Version {}",
                self.group_id, self.artifact_id, self.version
            )
        }
    }
    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    pub struct FoundBugDependency {
        pub group_id: String,
        pub artifact_id: String,
        pub version: String,
        pub depend_type: Option<String>,
        pub scope: Option<String>,
        pub classifier: Option<String>,
    }
    impl From<FoundBugDependency> for Dependency {
        fn from(val: FoundBugDependency) -> Self {
            Dependency {
                group_id: val.group_id,
                artifact_id: val.artifact_id,
                version: Some(val.version.parse().expect("Failed to parse version")),
                depend_type: val.depend_type,
                scope: val.scope,
                classifier: val.classifier,
            }
        }
    }

    #[cfg(test)]
    pub fn get_bugs_path() -> std::path::PathBuf {
        let cargo_home = env!("CARGO_MANIFEST_DIR");
        std::path::PathBuf::from(cargo_home)
            .join("tests")
            .join("data")
            .join("bugs")
    }
}
