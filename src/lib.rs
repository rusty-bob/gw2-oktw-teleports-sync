use std::io;

mod manager;
mod types;

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
            TeleportError::IoError(e) => write!(f, "IO error: {}", e),
            TeleportError::JsonError(e) => write!(f, "JSON error: {}", e),
            TeleportError::GroupNotFound(name) => write!(f, "Teleport group not found: {}", name),
            TeleportError::GroupAlreadyExists(name) => {
                write!(f, "Teleport group already exists: {}", name)
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
