use std::fmt::{Display, Formatter};

/// Maven Files have the following pattern {name}-{version}-{classifier}.{extension}.{hash}
/// This will represent the classifier and extension
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct MavenFileExtension {
    pub hash: Option<String>,
    pub file_extension: String,
    pub classifier: Option<String>,
}
impl MavenFileExtension {
    pub fn with_hash(mut self, hash: impl Into<String>) -> Self {
        self.hash = Some(hash.into());
        self
    }
    pub fn with_classifier(mut self, classifier: impl Into<String>) -> Self {
        self.classifier = Some(classifier.into());
        self
    }
}
impl From<&str> for MavenFileExtension {
    fn from(value: &str) -> Self {
        MavenFileExtension::from(value.to_owned())
    }
}

impl From<String> for MavenFileExtension {
    fn from(value: String) -> Self {
        MavenFileExtension {
            file_extension: value,
            ..Default::default()
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
