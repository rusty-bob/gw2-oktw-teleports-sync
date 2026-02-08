use crate::types::{TeleportConfig, TeleportGroup};
use crate::{Result, TeleportError};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;

/// Manager for handling teleport.json operations
pub struct TeleportManager {
    config: TeleportConfig,
    file_path: Option<String>,
}

impl TeleportManager {
    /// Create a new TeleportManager with default configuration
    pub fn new() -> Self {
        Self {
            config: TeleportConfig::default(),
            file_path: None,
        }
    }

    /// Create a TeleportManager from an existing configuration
    pub fn from_config(config: TeleportConfig) -> Self {
        Self {
            config,
            file_path: None,
        }
    }

    /// Load teleport configuration from a JSON file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path)?;
        let config: TeleportConfig = serde_json::from_str(&content)?;
        Ok(Self {
            config,
            file_path: Some(path.as_ref().to_string_lossy().to_string()),
        })
    }

    /// Save the current configuration to a JSON file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.config)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Save to the originally loaded file path (if available)
    pub fn save_to_original(&self) -> Result<()> {
        if let Some(path) = &self.file_path {
            self.save(path)
        } else {
            Err(TeleportError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No file path associated with this manager",
            )))
        }
    }

    /// Add a new teleport group
    pub fn add_group(&mut self, group: TeleportGroup) -> Result<()> {
        // Check if group with same name already exists
        if self.config.teleport_groups.iter().any(|g| g.name == group.name) {
            return Err(TeleportError::GroupAlreadyExists(group.name));
        }
        self.config.teleport_groups.push(group);
        Ok(())
    }

    /// Remove a teleport group by name
    pub fn remove_group(&mut self, name: &str) -> Result<TeleportGroup> {
        let index = self
            .config
            .teleport_groups
            .iter()
            .position(|g| g.name == name)
            .ok_or_else(|| TeleportError::GroupNotFound(name.to_string()))?;
        
        Ok(self.config.teleport_groups.remove(index))
    }

    /// Get a hash of a teleport group by name
    pub fn get_group_hash(&self, name: &str) -> Result<u64> {
        let group = self
            .config
            .teleport_groups
            .iter()
            .find(|g| g.name == name)
            .ok_or_else(|| TeleportError::GroupNotFound(name.to_string()))?;
        
        let mut hasher = DefaultHasher::new();
        // Hash the serialized JSON representation for consistency
        let json = serde_json::to_string(group).unwrap();
        json.hash(&mut hasher);
        Ok(hasher.finish())
    }

    /// Get all teleport groups
    pub fn get_all_groups(&self) -> &[TeleportGroup] {
        &self.config.teleport_groups
    }

    /// Get a mutable reference to all teleport groups
    pub fn get_all_groups_mut(&mut self) -> &mut Vec<TeleportGroup> {
        &mut self.config.teleport_groups
    }

    /// Get a specific teleport group by name
    pub fn get_group(&self, name: &str) -> Option<&TeleportGroup> {
        self.config.teleport_groups.iter().find(|g| g.name == name)
    }

    /// Get a mutable reference to a specific teleport group by name
    pub fn get_group_mut(&mut self, name: &str) -> Option<&mut TeleportGroup> {
        self.config.teleport_groups.iter_mut().find(|g| g.name == name)
    }

    /// Get a reference to the entire configuration
    pub fn get_config(&self) -> &TeleportConfig {
        &self.config
    }

    /// Get a mutable reference to the entire configuration
    pub fn get_config_mut(&mut self) -> &mut TeleportConfig {
        &mut self.config
    }

    /// Get the number of teleport groups
    pub fn group_count(&self) -> usize {
        self.config.teleport_groups.len()
    }
}

impl Default for TeleportManager {
    fn default() -> Self {
        Self::new()
    }
}

