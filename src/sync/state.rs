use crate::Result;
use crate::manager::TeleportManager;
use crate::remote::RemoteProvider;
use crate::types::TeleportGroup;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum SyncStatus {
    Installed, // Exists locally
    Available, // Exists remotely but not locally
}

#[derive(Debug, Clone)]
pub struct GroupSyncState {
    pub name: String,
    pub status: SyncStatus,
}

pub struct SyncStateManager {
    local_manager: TeleportManager,
    remote_provider: Box<dyn RemoteProvider>,
}

impl SyncStateManager {
    pub fn new(local_manager: TeleportManager, remote_provider: Box<dyn RemoteProvider>) -> Self {
        Self {
            local_manager,
            remote_provider,
        }
    }

    /// Get all local groups
    pub fn get_local_groups(&self) -> Vec<TeleportGroup> {
        self.local_manager.get_all_groups().to_vec()
    }

    /// Compute sync states by comparing local and remote
    pub async fn compute_sync_states(&self) -> Result<Vec<GroupSyncState>> {
        let local_groups = self.local_manager.get_all_groups();
        let local_names: HashSet<String> = local_groups.iter().map(|g| g.name.clone()).collect();

        let remote_names = self.remote_provider.list_available_groups().await?;

        let mut states = Vec::new();

        // Add installed groups
        for name in &local_names {
            states.push(GroupSyncState {
                name: name.clone(),
                status: SyncStatus::Installed,
            });
        }

        // Add available (not installed) groups
        for name in remote_names {
            if !local_names.contains(&name) {
                states.push(GroupSyncState {
                    name,
                    status: SyncStatus::Available,
                });
            }
        }

        Ok(states)
    }

    /// Sync (download and install) a single group
    pub async fn sync_group(&mut self, name: &str) -> Result<()> {
        let group = self.remote_provider.fetch_group(name).await?;
        self.local_manager.add_group(group)?;
        self.local_manager.save_to_original()?;
        Ok(())
    }

    /// Remove a local group
    pub fn remove_local_group(&mut self, name: &str) -> Result<()> {
        self.local_manager.remove_group(name)?;
        self.local_manager.save_to_original()?;
        Ok(())
    }
}
