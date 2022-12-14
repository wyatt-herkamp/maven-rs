mod extension;
pub mod local_config;
pub mod maven_metadata;
pub mod pom;
pub mod snapshot_metadata;
pub mod time;

pub use extension::MavenFileExtension;
pub use quick_xml;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to read {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse {0}")]
    XMLParser(#[from] quick_xml::Error),
    #[error("Failed to deserialize {0}")]
    XMLDeserialize(#[from] quick_xml::de::DeError),
    #[error("Invalid File Extension found")]
    InvalidFileExtension,
}

#[cfg(test)]
const MANIFEST: &str = env!("CARGO_MANIFEST_DIR");
