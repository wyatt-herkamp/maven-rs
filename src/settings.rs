use std::path::PathBuf;

use serde::{Deserialize, Serialize};
mod mirrors;
mod servers;
use crate::Error;
pub use mirrors::*;
pub use servers::*;
use std::env;
use std::io::BufReader;

pub static MAVEN_FOLDER: &str = ".m2";
pub static SETTINGS_FILE: &str = "settings.xml";
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub local_repository: Option<PathBuf>,
    pub interactive_mode: Option<bool>,
    pub offline: Option<bool>,
    #[serde(default)]
    pub servers: Servers,
    #[serde(default)]
    pub mirrors: Mirrors,
}
impl Settings {
    pub fn get_local_repository(&self) -> Option<PathBuf> {
        self.local_repository.clone()
    }
    /// Attempts to read the local configuration file.
    pub fn read_local_config() -> Result<Settings, Error> {
        let result = get_settings_path().ok_or(Error::NoHomeDirectory)?;
        if !result.exists() {
            return Ok(Settings::default());
        }
        let file = std::fs::File::open(result)?;
        quick_xml::de::from_reader(BufReader::new(file)).map_err(Error::from)
    }
    /// Returns the local repository or the default repository.
    ///
    /// If None is Returned Home Directory is not found.
    pub fn get_local_repository_or_default(&self) -> Option<PathBuf> {
        if let Some(local_repository) = &self.local_repository {
            Some(local_repository.clone())
        } else {
            get_settings_directory().map(|dir| dir.join("repository"))
        }
    }
}
/// Returns the path to the .m2 folder
///
/// If the home directory is not found, None is returned.
///
/// # Example
///
/// ```
/// use maven_rs::settings::directories::get_settings_directory;
/// let path = get_settings_directory();
/// println!("{:?}", path);
/// ``````
pub fn get_settings_directory() -> Option<PathBuf> {
    env::home_dir().map(|dirs| dirs.join(MAVEN_FOLDER))
}

/// Returns returns the path to the settings file.
pub fn get_settings_path() -> Option<PathBuf> {
    get_settings_directory().map(|dir| dir.join(SETTINGS_FILE))
}

#[cfg(test)]
pub mod tests {
    use std::env;

    use crate::settings::{Server, Servers, Settings};

    #[test]
    pub fn test_to_string() {
        let settings = Settings {
            local_repository: None,
            servers: Servers {
                servers: vec![Server {
                    id: "test".to_string(),
                    username: Some("test".to_string()),
                    password: Some("test".to_string()),
                    ..Default::default()
                }],
            },
            ..Default::default()
        };

        println!("{}", quick_xml::se::to_string(&settings).unwrap());
    }

    #[test]
    pub fn test_read_local_config() {
        let settings = Settings::read_local_config().unwrap();
        println!("{:?}", settings);
    }
}
