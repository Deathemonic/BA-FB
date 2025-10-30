use crate::helpers::config::*;

use baad::info;
use baad::utils::file;
use eyre::{eyre, Result};
use reqwest::{Client, Url};
use serde::Deserialize;
use std::env::consts::{ARCH, OS};
use std::path::Path;
use trauma::download::Download;
use trauma::downloader::{Downloader, DownloaderBuilder};

#[derive(Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Deserialize)]
struct GitHubRelease {
    assets: Vec<GitHubAsset>,
}

pub struct ToolsFetcher {
    downloader: Downloader,
    client: Client,
}

impl ToolsFetcher {
    pub fn new() -> Result<Self> {
        let downloader = DownloaderBuilder::new()
            .directory(file::data_dir()?.into())
            .use_range_for_content_length(true)
            .single_file_progress(true)
            .overwrite(true)
            .build();
        let client = Client::builder()
            .user_agent("BA-FB/1.7 (Blue Archive - FlatBuffer)")
            .build()?;
        Ok(Self { downloader, client })
    }

    async fn download(&self, url: &[Download]) -> Result<()> {
        self.downloader.download(url, None).await;
        info!(success = true, "Successfully downloaded.");
        Ok(())
    }

    async fn fetch_github(&self, repo: &str) -> Result<GitHubRelease> {
        let url = format!("{}/{}/releases/latest", GITHUB_API_BASE, repo);
        let response = self.client.get(&url).send().await?;
        let release: GitHubRelease = response.json().await?;
        Ok(release)
    }

    async fn download_tool<F>(&self, repo: &str, filename: &str, get_asset_name: F) -> Result<()>
    where
        F: FnOnce() -> Result<&'static str>,
    {
        let tool_name = Path::new(filename)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or(filename);

        let file_path = file::get_data_path(&format!("{}/{}", TOOLS_DIR, filename))?
            .to_string_lossy()
            .into();

        let download_url = if repo.starts_with(HTTP_PREFIX) || repo.starts_with(HTTPS_PREFIX) {
            info!(tool_name, "Downloading");
            repo.to_string()
        } else {
            info!(tool_name, "Downloading");
            let asset_name = get_asset_name()?;
            let release = self.fetch_github(repo).await?;

            release
                .assets
                .into_iter()
                .find(|a| a.name.contains(asset_name))
                .map(|a| a.browser_download_url)
                .ok_or_else(|| eyre!("No asset found"))?
        };

        let download = vec![Download {
            url: Url::parse(&download_url)?,
            filename: file_path,
            target_file: None,
            hash: None,
        }];

        self.download(&download).await
    }

    fn get_platform() -> Result<&'static str> {
        match (OS, ARCH) {
            ("windows", "x86_64") => Ok(WIN_X64),
            ("windows", "aarch64") => Ok(WIN_ARM64),
            ("macos", "x86_64") => Ok(OSX_X64),
            ("macos", "aarch64") => Ok(OSX_ARM64),
            ("linux", "x86_64") => Ok(LINUX_X64),
            ("linux", "aarch64") => Ok(LINUX_ARM64),
            _ => Err(eyre!("Unsupported platform")),
        }
    }

    pub async fn il2cpp_dumper(&self, repo: &str) -> Result<()> {
        self.download_tool(
            repo,
            IL2CPP_INSPECTOR_FILE,
            Self::get_platform,
        )
        .await
    }

    pub async fn fbs_dumper(&self, repo: &str) -> Result<()> {
        self.download_tool(repo, FBS_DUMPER_FILE, Self::get_platform)
            .await
    }

    pub async fn flatc(&self, repo: &str) -> Result<()> {
        self.download_tool(repo, FLATC_FILE, || match (OS, ARCH) {
            ("windows", _) => Ok("Windows.flatc.binary.zip"),
            ("macos", "aarch64") => Ok("Mac.flatc.binary.zip"),
            ("macos", "x86_64") => Ok("MacIntel.flatc.binary.zip"),
            ("linux", _) => Ok("Linux.flatc.binary.g++-13.zip"),
            _ => Err(eyre!("Unsupported platform")),
        })
        .await
    }
}
