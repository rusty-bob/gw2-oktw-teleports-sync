use crate::remote::RemoteProvider;
use crate::types::TeleportGroup;
use crate::{Result, TeleportError};
use serde::Deserialize;

pub struct GitHubProvider {
    repo_owner: String,
    repo_name: String,
    branch: String,
    teleports_path: String,
    client: reqwest::Client,
}

impl GitHubProvider {
    pub fn new(repo_owner: String, repo_name: String) -> Self {
        Self {
            repo_owner,
            repo_name,
            branch: "main".to_string(),
            teleports_path: "teleports".to_string(),
            client: reqwest::Client::new(),
        }
    }

    fn api_url(&self) -> String {
        format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.repo_owner, self.repo_name, self.teleports_path
        )
    }

    fn raw_file_url(&self, filename: &str) -> String {
        format!(
            "https://raw.githubusercontent.com/{}/{}/{}/{}/{}",
            self.repo_owner, self.repo_name, self.branch, self.teleports_path, filename
        )
    }
}

#[derive(Deserialize)]
struct GitHubFile {
    name: String,
    #[serde(rename = "type")]
    file_type: String,
}

#[async_trait::async_trait]
impl RemoteProvider for GitHubProvider {
    async fn list_available_groups(&self) -> Result<Vec<String>> {
        let url = self.api_url();

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "tp_sync")
            .send()
            .await
            .map_err(|e| {
                TeleportError::IoError(std::io::Error::other(format!(
                    "Failed to fetch from GitHub: {}",
                    e
                )))
            })?;

        if !response.status().is_success() {
            return Err(TeleportError::IoError(std::io::Error::other(format!(
                "GitHub API returned status: {}",
                response.status()
            ))));
        }

        let files: Vec<GitHubFile> = response.json().await.map_err(|e| {
            TeleportError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to parse GitHub response: {}", e),
            ))
        })?;

        // Filter for JSON files and extract group names
        let group_names: Vec<String> = files
            .into_iter()
            .filter(|f| f.file_type == "file" && f.name.ends_with(".json"))
            .map(|f| {
                // Remove .json extension to get group name
                f.name.trim_end_matches(".json").to_string()
            })
            .collect();

        Ok(group_names)
    }

    async fn fetch_group(&self, name: &str) -> Result<TeleportGroup> {
        let filename = format!("{}.json", name);
        let url = self.raw_file_url(&filename);

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "tp_sync")
            .send()
            .await
            .map_err(|e| {
                TeleportError::IoError(std::io::Error::other(format!(
                    "Failed to fetch group from GitHub: {}",
                    e
                )))
            })?;

        if !response.status().is_success() {
            return Err(TeleportError::GroupNotFound(name.to_string()));
        }

        // GitHub files contain a single TeleportGroup directly, not a TeleportConfig
        let group: TeleportGroup = response.json().await.map_err(|e| {
            TeleportError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to parse group JSON: {}", e),
            ))
        })?;

        Ok(group)
    }

    async fn fetch_groups(&self, names: &[String]) -> Result<Vec<TeleportGroup>> {
        let mut groups = Vec::new();

        for name in names {
            let group = self.fetch_group(name).await?;
            groups.push(group);
        }

        Ok(groups)
    }
}
