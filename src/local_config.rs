use crate::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    #[serde(rename = "localRepository")]
    pub local_repository: Option<String>,
    #[serde(default)]
    pub servers: Vec<Server>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub id: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[cfg(feature = "directories")]
pub mod directories {
    use std::path::PathBuf;
    use crate::Error;
    use super::Settings;

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
            let result = get_settings_path().ok_or(Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "System User Path Not Found")))?;
            if !result.exists() {
                return Ok(Settings::default());
            }
            let file = std::fs::File::open(result)?;
            serde_xml_rs::from_reader(&file).map_err(|e| Error::XMLParser(e))
        }
        pub fn get_local_repository(&self) -> Option<PathBuf> {
            if let Some(ref local_repository) = self.local_repository {
                Some(PathBuf::from(local_repository))
            } else {
                get_settings_directory().and_then(|dir| Some(dir.join("repository")))
            }
        }
    }
}
