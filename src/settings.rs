use std::path::PathBuf;

use serde::{Deserialize, Serialize};
mod mirrors;
mod servers;
pub use mirrors::*;
pub use servers::*;
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
}
#[cfg(feature = "local")]
pub mod directories {
    use super::Settings;
    use crate::Error;
    use std::io::BufReader;
    use std::path::PathBuf;

    /// Returns the path to the .m2 folder
    pub fn get_settings_directory() -> Option<PathBuf> {
        directories::BaseDirs::new().map(|dirs| dirs.home_dir().join(".m2"))
    }

    /// Returns returns the path to the settings file.
    pub fn get_settings_path() -> Option<PathBuf> {
        get_settings_directory().map(|dir| dir.join("settings.xml"))
    }

    impl Settings {
        /// Attempts to read the local configuration file.
        pub fn read_local_config() -> Result<Settings, Error> {
            let result = get_settings_path().ok_or(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "System User Path Not Found",
            )))?;
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
                get_settings_directory().and_then(|dir| Some(dir.join("repository")))
            }
        }
    }
}

#[cfg(all(test, feature = "local"))]
pub mod tests {
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
