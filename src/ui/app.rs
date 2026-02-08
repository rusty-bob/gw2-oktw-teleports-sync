use crate::Result;
use crate::config::AppConfig;
use crate::manager::TeleportManager;
use crate::remote::GitHubProvider;
use crate::sync::{SyncStateManager, SyncStatus};
use ratatui::widgets::ListState;

#[derive(Debug, Clone, PartialEq)]
pub enum Pane {
    Local,
    Remote,
}

#[derive(Debug, Clone)]
pub enum AppMode {
    Normal,
    ConfirmDelete(String),
    ConfirmInstall(String),
}

pub struct App {
    pub sync_manager: SyncStateManager,
    pub active_pane: Pane,
    pub mode: AppMode,
    pub local_list_state: ListState,
    pub remote_list_state: ListState,
    pub local_groups: Vec<String>,
    pub remote_groups: Vec<String>,
    pub status_message: Option<String>,
    pub is_loading: bool,
}

impl App {
    pub async fn new(config: &AppConfig) -> Result<Self> {
        let local_manager = TeleportManager::load(&config.teleport_json_path)?;
        let remote_provider = Box::new(GitHubProvider::new(
            config.remote_repo_owner.clone(),
            config.remote_repo_name.clone(),
        ));
        let sync_manager = SyncStateManager::new(local_manager, remote_provider);

        let mut app = Self {
            sync_manager,
            active_pane: Pane::Local,
            mode: AppMode::Normal,
            local_list_state: ListState::default(),
            remote_list_state: ListState::default(),
            local_groups: Vec::new(),
            remote_groups: Vec::new(),
            status_message: None,
            is_loading: false,
        };

        app.refresh_data().await?;

        if !app.local_groups.is_empty() {
            app.local_list_state.select(Some(0));
        }

        Ok(app)
    }

    pub async fn refresh_data(&mut self) -> Result<()> {
        self.is_loading = true;

        let local = self.sync_manager.get_local_groups();
        self.local_groups = local.iter().map(|g| g.name.clone()).collect();

        let states = self.sync_manager.compute_sync_states().await?;
        self.remote_groups = states
            .into_iter()
            .filter(|s| matches!(s.status, SyncStatus::Available))
            .map(|s| s.name)
            .collect();

        self.is_loading = false;
        Ok(())
    }

    pub fn selected_group(&self) -> Option<&String> {
        match self.active_pane {
            Pane::Local => self
                .local_list_state
                .selected()
                .and_then(|i| self.local_groups.get(i)),
            Pane::Remote => self
                .remote_list_state
                .selected()
                .and_then(|i| self.remote_groups.get(i)),
        }
    }

    pub fn switch_pane(&mut self) {
        self.active_pane = match self.active_pane {
            Pane::Local => {
                if self.remote_list_state.selected().is_none() && !self.remote_groups.is_empty() {
                    self.remote_list_state.select(Some(0));
                }
                Pane::Remote
            }
            Pane::Remote => {
                if self.local_list_state.selected().is_none() && !self.local_groups.is_empty() {
                    self.local_list_state.select(Some(0));
                }
                Pane::Local
            }
        };
    }

    pub fn navigate_up(&mut self) {
        let (state, items_len) = match self.active_pane {
            Pane::Local => (&mut self.local_list_state, self.local_groups.len()),
            Pane::Remote => (&mut self.remote_list_state, self.remote_groups.len()),
        };

        if items_len == 0 {
            return;
        }

        let i = match state.selected() {
            Some(i) => {
                if i == 0 {
                    items_len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        state.select(Some(i));
    }

    pub fn navigate_down(&mut self) {
        let (state, items_len) = match self.active_pane {
            Pane::Local => (&mut self.local_list_state, self.local_groups.len()),
            Pane::Remote => (&mut self.remote_list_state, self.remote_groups.len()),
        };

        if items_len == 0 {
            return;
        }

        let i = match state.selected() {
            Some(i) => {
                if i >= items_len - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        state.select(Some(i));
    }

    pub fn handle_delete(&mut self) {
        if matches!(self.active_pane, Pane::Local)
            && let Some(group_name) = self.selected_group()
        {
            self.mode = AppMode::ConfirmDelete(group_name.clone());
        }
    }

    pub fn handle_space(&mut self) {
        if matches!(self.active_pane, Pane::Remote)
            && let Some(group_name) = self.selected_group()
        {
            self.mode = AppMode::ConfirmInstall(group_name.clone());
        }
    }

    pub async fn confirm_delete(&mut self, group_name: String) -> Result<()> {
        match self.sync_manager.remove_local_group(&group_name) {
            Ok(_) => {
                self.status_message = Some(format!("✓ Deleted '{}'", group_name));
            }
            Err(e) => {
                self.status_message = Some(format!("✗ Error: {}", e));
            }
        }
        self.mode = AppMode::Normal;
        self.refresh_data().await?;
        Ok(())
    }

    pub async fn confirm_install(&mut self, group_name: String) -> Result<()> {
        self.is_loading = true;
        match self.sync_manager.sync_group(&group_name).await {
            Ok(_) => {
                self.status_message = Some(format!("✓ Installed '{}'", group_name));
            }
            Err(e) => {
                self.status_message = Some(format!("✗ Error: {}", e));
            }
        }
        self.mode = AppMode::Normal;
        self.refresh_data().await?;
        Ok(())
    }

    pub fn cancel_confirmation(&mut self) {
        self.mode = AppMode::Normal;
    }
}
