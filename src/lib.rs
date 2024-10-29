pub mod extension;
#[cfg(feature = "resolver")]
pub mod resolver;

pub mod editor;
pub mod meta;
pub mod pom;
pub mod settings;
pub mod types;
pub mod utils;
// Re-export quick_xml
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
    #[error("No Home Directory Found")]
    NoHomeDirectory,
}
