use crate::types::TeleportGroup;
use crate::Result;

/// Trait for fetching teleport groups from a remote source
#[async_trait::async_trait]
pub trait RemoteProvider: Send + Sync {
    /// List all available group names from the remote source
    async fn list_available_groups(&self) -> Result<Vec<String>>;

    /// Fetch a specific teleport group by name
    async fn fetch_group(&self, name: &str) -> Result<TeleportGroup>;

    /// Fetch multiple groups at once
    async fn fetch_groups(&self, names: &[String]) -> Result<Vec<TeleportGroup>>;
}

