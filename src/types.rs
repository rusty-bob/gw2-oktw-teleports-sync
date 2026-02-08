use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a single teleport location
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Teleport {
    pub name: String,
    pub coordinates: [f32; 3],
    pub map: u32,
    pub tooltip: Option<String>,
}

/// Represents a group of teleport locations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TeleportGroup {
    pub name: String,
    pub teleports: Vec<Teleport>,
}

/// The teleport configuration structure - only parses teleport_groups
/// All other fields are preserved but ignored
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TeleportConfig {
    pub teleport_groups: Vec<TeleportGroup>,

    /// All other fields from the JSON are stored here and preserved when saving
    #[serde(flatten)]
    pub other_fields: serde_json::Map<String, Value>,
}

impl Default for TeleportConfig {
    fn default() -> Self {
        Self {
            teleport_groups: Vec::new(),
            other_fields: serde_json::Map::new(),
        }
    }
}
