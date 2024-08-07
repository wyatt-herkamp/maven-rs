use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    #[serde(rename = "localRepository")]
    pub local_repository: Option<String>,
    #[serde(default)]
    pub servers: Servers,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Servers {
    #[serde(default, rename = "server")]
    pub servers: Vec<Server>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Server {
    pub id: String,
    pub username: Option<String>,
    pub password: Option<String>,
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
        pub fn get_local_repository(&self) -> Option<PathBuf> {
            if let Some(ref local_repository) = self.local_repository {
                Some(PathBuf::from(local_repository))
            } else {
                get_settings_directory().and_then(|dir| Some(dir.join("repository")))
            }
        }
    }
}

#[cfg(all(test, feature = "local"))]
pub mod tests {
    use crate::local_config::{Server, Servers, Settings};

    #[test]
    pub fn test_to_string() {
        let settings = Settings {
            local_repository: Some("test".to_string()),
            servers: Servers {
                servers: vec![Server {
                    id: "test".to_string(),
                    username: Some("test".to_string()),
                    password: Some("test".to_string()),
                }],
            },
        };

        println!("{}", quick_xml::se::to_string(&settings).unwrap());
    }

    #[test]
    pub fn test_read_local_config() {
        let settings = Settings::read_local_config().unwrap();
        println!("{:?}", settings);
    }
}
