use std::fmt::{Display, Formatter};

/// Maven Files have the following pattern {name}-{version}-{classifier}.{extension}.{hash}
/// This will represent the classifier and extension
pub struct MavenFileExtension {
    pub hash: Option<String>,
    pub file_extension: String,
    pub classifier: Option<String>,
}

impl From<&str> for MavenFileExtension {
    fn from(value: &str) -> Self {
        MavenFileExtension {
            hash: None,
            file_extension: value.to_string(),
            classifier: None,
        }
    }
}

impl From<String> for MavenFileExtension {
    fn from(value: String) -> Self {
        MavenFileExtension {
            hash: None,
            file_extension: value,
            classifier: None,
        }
    }
}

impl From<(String, String, String)> for MavenFileExtension {
    fn from((classifier, file, hash): (String, String, String)) -> Self {
        MavenFileExtension {
            hash: Some(hash),
            file_extension: file,
            classifier: Some(classifier),
        }
    }
}

impl From<(&str, &str, &str)> for MavenFileExtension {
    fn from((classifier, file, hash): (&str, &str, &str)) -> Self {
        MavenFileExtension {
            hash: Some(hash.to_string()),
            file_extension: file.to_string(),
            classifier: Some(classifier.to_string()),
        }
    }
}

impl From<(Option<String>, String, Option<String>)> for MavenFileExtension {
    fn from((classifier, file, hash): (Option<String>, String, Option<String>)) -> Self {
        MavenFileExtension {
            hash,
            file_extension: file,
            classifier,
        }
    }
}

impl From<(Option<&str>, &str, Option<&str>)> for MavenFileExtension {
    fn from((classifier, file, hash): (Option<&str>, &str, Option<&str>)) -> Self {
        MavenFileExtension {
            hash: hash.map(|x| x.to_owned()),
            file_extension: file.to_string(),
            classifier: classifier.map(|x| x.to_owned()),
        }
    }
}

impl Display for MavenFileExtension {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(classifier) = &self.classifier {
            write!(f, "-{}", classifier)?;
        }
        write!(f, ".{}", self.file_extension)?;
        if let Some(hash) = &self.hash {
            write!(f, ".{}", hash)?;
        }
        Ok(())
    }
}
