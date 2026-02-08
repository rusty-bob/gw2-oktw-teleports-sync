use std::io;

pub mod config;
mod manager;
pub mod remote;
pub mod sync;
mod types;
pub mod ui;

pub use manager::TeleportManager;
pub use types::*;

/// Custom error type for teleport operations
#[derive(Debug)]
pub enum TeleportError {
    IoError(io::Error),
    JsonError(serde_json::Error),
    GroupNotFound(String),
    GroupAlreadyExists(String),
}

impl std::fmt::Display for TeleportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TeleportError::IoError(e) => write!(f, "{}", e),
            TeleportError::JsonError(e) => write!(f, "Invalid JSON: {}", e),
            TeleportError::GroupNotFound(name) => write!(f, "Group '{}' not found", name),
            TeleportError::GroupAlreadyExists(name) => {
                write!(f, "Group '{}' is already installed", name)
            }
        }
    }
}

impl std::error::Error for TeleportError {}

impl From<io::Error> for TeleportError {
    fn from(error: io::Error) -> Self {
        TeleportError::IoError(error)
    }
}

impl From<serde_json::Error> for TeleportError {
    fn from(error: serde_json::Error) -> Self {
        TeleportError::JsonError(error)
    }
}

pub type Result<T> = std::result::Result<T, TeleportError>;
