use serde::{Deserialize, Serialize};

pub mod pom;
pub mod maven_metadata;
pub mod time;
pub mod snapshot_metadata;
mod local_config;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to read {0}")]
    Io(#[from]std::io::Error),
    #[error("Failed to parse {0}")]
    XMLParser(#[from]serde_xml_rs::Error),
}