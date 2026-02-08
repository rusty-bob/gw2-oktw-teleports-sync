use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

/// Represents a key binding
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyBind {
    pub keys: [Option<u32>; 3],
}

/// Represents the main window configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MainWindow {
    pub visible: bool,
    pub pos: [f32; 2],
    pub size: [f32; 2],
}

/// Represents popout group states
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PopoutGroupStates {
    pub open_groups: Vec<String>,
    pub windows: HashMap<String, serde_json::Value>,
}

/// The complete teleport configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TeleportConfig {
    pub main_window: MainWindow,
    pub key_binds: Vec<(String, KeyBind)>,
    pub teleport_groups: Vec<TeleportGroup>,
    pub popout_group_states: PopoutGroupStates,
    pub teleport_to_cursor: bool,
    pub death_teleport: bool,
    pub personal_waypoint_teleport: bool,
    pub next_teleport: bool,
    pub prev_teleport: bool,
    pub vertical_icons: bool,
    pub show_all_wps: bool,
    pub disclaimer: bool,
}

impl Default for TeleportConfig {
    fn default() -> Self {
        Self {
            main_window: MainWindow {
                visible: false,
                pos: [0.0, 0.0],
                size: [400.0, 300.0],
            },
            key_binds: Vec::new(),
            teleport_groups: Vec::new(),
            popout_group_states: PopoutGroupStates {
                open_groups: Vec::new(),
                windows: HashMap::new(),
            },
            teleport_to_cursor: false,
            death_teleport: false,
            personal_waypoint_teleport: false,
            next_teleport: false,
            prev_teleport: false,
            vertical_icons: false,
            show_all_wps: false,
            disclaimer: false,
        }
    }
}
