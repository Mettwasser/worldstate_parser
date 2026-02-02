use std::{
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

use reqwest::Client;
use serde::Deserialize;
use tokio::fs;
use tracing::{debug, error, info};

#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    #[error("Request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("GitHub API error: {0}")]
    GitHub(String),

    #[error("Task join error: {0}")]
    Join(#[from] tokio::task::JoinError),

    #[error("Download failed for {path}: {message}")]
    Download { path: String, message: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CacheStrategy {
    #[default]
    None,
    Basic,
    Duration(Duration),
}

#[derive(Deserialize)]
struct GitHubTreeResponse {
    tree: Vec<GitHubTreeEntry>,
}

#[derive(Deserialize)]
struct GitHubTreeEntry {
    path: String,
    r#type: String,
}

const USER_AGENT: &str = "Rust-Worldstate-Parser";

pub struct DataFetcher {
    client: Client,
    data_dir: PathBuf,
    drops_dir: PathBuf,
    assets_dir: PathBuf,
    cache_strategy: CacheStrategy,
}

impl Default for DataFetcher {
    fn default() -> Self {
        Self::new().expect("Failed to create default DataFetcher")
    }
}

impl DataFetcher {
    /// Creates a new DataFetcher with default directories ("data", "drops", "assets") and no
    /// caching.
    pub fn new() -> Result<Self, FetchError> {
        let client = Client::builder().user_agent(USER_AGENT).build()?;
        Ok(Self {
            client,
            data_dir: PathBuf::from("data"),
            drops_dir: PathBuf::from("drops"),
            assets_dir: PathBuf::from("assets"),
            cache_strategy: CacheStrategy::None,
        })
    }

    /// Sets the directory where WFCD data will be stored.
    pub fn data_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.data_dir = path.into();
        self
    }

    /// Sets the directory where drop data will be stored.
    pub fn drops_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.drops_dir = path.into();
        self
    }

    /// Sets the directory where assets will be stored.
    pub fn assets_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.assets_dir = path.into();
        self
    }

    /// Sets the caching strategy.
    pub fn cache_strategy(mut self, strategy: CacheStrategy) -> Self {
        self.cache_strategy = strategy;
        self
    }

    /// Fetches all necessary data files.
    pub async fn fetch_all(&self) -> Result<(), FetchError> {
        self.fetch_data().await?;
        self.fetch_drops().await?;
        self.fetch_assets().await?;
        Ok(())
    }

    /// Fetches data from WFCD/warframe-worldstate-data.
    pub async fn fetch_data(&self) -> Result<(), FetchError> {
        info!("Fetching data from WFCD/warframe-worldstate-data...");
        self.fetch_github_folder(
            "WFCD",
            "warframe-worldstate-data",
            "master",
            "data",
            &self.data_dir,
        )
        .await
    }

    /// Fetches drops from warframestat.us.
    pub async fn fetch_drops(&self) -> Result<(), FetchError> {
        let url = "https://drops.warframestat.us/data/all.json";
        let target_path = self.drops_dir.join("data.json");

        if self.should_skip(&target_path).await {
            info!("Skipping drops download (cached)");
            return Ok(());
        }

        fs::create_dir_all(&self.drops_dir).await?;

        info!("Downloading {} to {}...", url, target_path.display());
        let res = self.client.get(url).send().await?;
        let bytes = res.bytes().await?;
        fs::write(target_path, bytes).await?;
        info!("Drops download complete.");

        Ok(())
    }

    /// Fetches assets from Mettwasser/worldstate_parser_assets using the 'main' branch.
    pub async fn fetch_assets(&self) -> Result<(), FetchError> {
        info!("Fetching assets from Mettwasser/worldstate_parser_assets...");
        self.fetch_github_folder(
            "Mettwasser",
            "worldstate_parser_assets",
            "main",
            "",
            &self.assets_dir,
        )
        .await
    }

    async fn fetch_github_folder(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        folder: &str,
        target_dir: &Path,
    ) -> Result<(), FetchError> {
        let api_url =
            format!("https://api.github.com/repos/{owner}/{repo}/git/trees/{branch}?recursive=1");

        let response = self
            .client
            .get(&api_url)
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?
            .json::<GitHubTreeResponse>()
            .await?;

        fs::create_dir_all(target_dir).await?;

        let files_to_download: Vec<_> = response
            .tree
            .into_iter()
            .filter(|entry| {
                if entry.r#type != "blob" {
                    return false;
                }
                let path = Path::new(&entry.path);
                if folder.is_empty() {
                    path.parent()
                        .map(|p| p.as_os_str().is_empty())
                        .unwrap_or(true)
                } else {
                    path.parent()
                        .map(|p| p.to_string_lossy() == folder)
                        .unwrap_or(false)
                        && path.extension().is_some_and(|ext| ext == "json")
                }
            })
            .collect();

        let mut handles = Vec::new();
        for entry in files_to_download {
            let file_name = Path::new(&entry.path).file_name().unwrap().to_owned();
            let target_path = target_dir.join(file_name);

            if self.should_skip(&target_path).await {
                info!(
                    "Skipping download for repo {repo} at {target_path} (cached)",
                    target_path = target_path.to_str().unwrap_or("")
                );
                continue;
            }

            let client = self.client.clone();
            let raw_url = format!(
                "https://raw.githubusercontent.com/{owner}/{repo}/{branch}/{}",
                entry.path
            );
            let path_clone = entry.path.clone();

            handles.push(tokio::spawn(async move {
                debug!("Downloading {}...", path_clone);
                let res = client
                    .get(&raw_url)
                    .send()
                    .await
                    .map_err(|e| e.to_string())?;
                if !res.status().is_success() {
                    return Err(format!("Status {}", res.status()));
                }
                let bytes = res.bytes().await.map_err(|e| e.to_string())?;
                fs::write(target_path, bytes)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok::<(), String>(())
            }));
        }

        if !handles.is_empty() {
            info!(
                "Downloading {} new/updated files from {}/{}.",
                handles.len(),
                owner,
                repo
            );
        }

        for handle in handles {
            match handle.await? {
                Ok(_) => {},
                Err(e) => {
                    error!("Download error: {}", e);
                },
            }
        }

        Ok(())
    }

    async fn should_skip(&self, path: &Path) -> bool {
        match self.cache_strategy {
            CacheStrategy::None => false,
            CacheStrategy::Basic => path.exists(),
            CacheStrategy::Duration(duration) => {
                if let Ok(metadata) = fs::metadata(path).await
                    && let Ok(modified) = metadata.modified()
                    && let Ok(elapsed) = SystemTime::now().duration_since(modified)
                {
                    elapsed < duration
                } else {
                    false
                }
            },
        }
    }
}

/// Helper function to fetch everything using default settings and no caching.
pub async fn fetch_all(cache_strategy: CacheStrategy) -> Result<(), FetchError> {
    DataFetcher::new()?
        .cache_strategy(cache_strategy)
        .fetch_all()
        .await
}
