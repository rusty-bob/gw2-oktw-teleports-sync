use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Path to the teleport.json file
    pub teleport_json_path: PathBuf,

    /// Remote repository configuration
    #[serde(default = "default_repo_owner")]
    pub remote_repo_owner: String,

    #[serde(default = "default_repo_name")]
    pub remote_repo_name: String,
}

fn default_repo_owner() -> String {
    "rusty-bob".to_string()
}

fn default_repo_name() -> String {
    "gw2-oktw-teleports".to_string()
}

impl AppConfig {
    /// Load config from disk, returns None if not found
    pub fn load() -> crate::Result<Option<Self>> {
        let path = Self::config_path()?;

        if !path.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&path)?;
        let config: Self = toml::from_str(&content).map_err(|e| {
            crate::TeleportError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to parse config: {}", e),
            ))
        })?;

        Ok(Some(config))
    }

    /// Save config to disk
    pub fn save(&self) -> crate::Result<()> {
        let path = Self::config_path()?;

        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self).map_err(|e| {
            crate::TeleportError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to serialize config: {}", e),
            ))
        })?;
        std::fs::write(&path, content)?;

        Ok(())
    }

    /// Get config file path (in the same directory as the executable)
    fn config_path() -> crate::Result<PathBuf> {
        let exe_path = std::env::current_exe()?;
        let exe_dir = exe_path.parent().ok_or_else(|| {
            crate::TeleportError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not determine executable directory",
            ))
        })?;

        Ok(exe_dir.join("config.toml"))
    }

    /// Create a new config with the given teleport.json path
    pub fn new(teleport_json_path: PathBuf) -> Self {
        Self {
            teleport_json_path,
            remote_repo_owner: default_repo_owner(),
            remote_repo_name: default_repo_name(),
        }
    }

    /// Validate that the teleport.json path exists and is readable
    pub fn validate(&self) -> crate::Result<()> {
        if !self.teleport_json_path.exists() {
            return Err(crate::TeleportError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "teleport.json not found at: {}",
                    self.teleport_json_path.display()
                ),
            )));
        }

        // Try to read it to ensure it's valid
        std::fs::read_to_string(&self.teleport_json_path)?;

        Ok(())
    }
}
