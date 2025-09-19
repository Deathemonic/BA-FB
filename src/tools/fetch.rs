use crate::helpers::config::*;

use anyhow::{anyhow, Result};
use baad::utils::FileManager;
use baad_core::{errors::ErrorContext, info, success};
use reqwest::{Client, Url};
use serde::Deserialize;
use std::env::consts::{ARCH, OS};
use std::path::Path;
use std::rc::Rc;
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
    tag_name: String,
}

pub struct ToolsFetcher {
    file_manager: Rc<FileManager>,
    downloader: Downloader,
    client: Client,
}

impl ToolsFetcher {
    pub fn new(file_manager: Rc<FileManager>) -> Result<Self> {
        let downloader = DownloaderBuilder::new()
            .directory(file_manager.get_data_dir().to_path_buf())
            .use_range_for_content_length(true)
            .single_file_progress(true)
            .overwrite(true)
            .build();

        let client = reqwest::Client::new();

        Ok(Self {
            file_manager,
            downloader,
            client,
        })
    }

    async fn download(&self, url: &[Download]) -> Result<()> {
        self.downloader.download(url).await;

        success!("Successfully downloaded.");
        Ok(())
    }

    fn get_platform(mac_prefix: bool) -> Result<&'static str> {
        match (OS, ARCH) {
            ("windows", "x86_64") => Ok(WIN_X64),
            ("windows", "aarch64") => Ok(WIN_ARM64),
            ("macos", "x86_64") => Ok(if mac_prefix { MAC_X64 } else { OSX_X64 }),
            ("macos", "aarch64") => Ok(if mac_prefix { MAC_ARM64 } else { OSX_ARM64 }),
            ("linux", "x86_64") => Ok(LINUX_X64),
            ("linux", "aarch64") => Ok(LINUX_ARM64),
            _ => None.error_context("Unsupported platform")?,
        }
    }

    fn find_asset<'a>(assets: &'a [GitHubAsset], suffix: &str) -> Result<&'a GitHubAsset> {
        assets
            .iter()
            .find(|asset| asset.name.contains(suffix))
            .ok_or_else(|| anyhow!("No asset found for platform: {}", suffix))
    }

    async fn fetch_github(&self, repo: &str) -> Result<GitHubRelease> {
        let url = format!("{}/{}/releases/latest", GITHUB_API_BASE, repo);
        let response = self.client.get(&url).send().await?;
        let release: GitHubRelease = response.json().await?;
        Ok(release)
    }

    async fn download_tool(&self, url: &str, filename: &str) -> Result<()> {
        let tool_name = Path::new(filename)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or(filename);

        info!("Downloading {} from {}...", tool_name, url);

        let download_url = if url.starts_with(HTTP_PREFIX) || url.starts_with(HTTPS_PREFIX) {
            url
        } else {
            let release = self.fetch_github(url).await?;
            let platform = Self::get_platform(false)?;
            let asset = Self::find_asset(&release.assets, platform)?;
            &asset.browser_download_url
        };

        let file_path = self
            .file_manager
            .get_data_path(&format!("{}/{}", TOOLS_DIR, filename))
            .to_string_lossy()
            .to_string();

        let download = vec![Download {
            url: Url::parse(&download_url)?,
            filename: file_path,
            hash: None,
        }];

        self.download(&download).await
    }

    pub async fn il2cpp_dumper(&self) -> Result<()> {
        let platform = Self::get_platform(false)?;
        let tool = format!("Il2CppInspectorRedux.CLI-{}.zip", platform);
        let url = format!("{}/{}", IL2CPP_BASE_URL, tool);

        self.download_tool(&url, IL2CPP_FILE).await
    }

    pub async fn fbs_dumper(&self) -> Result<()> {
        self.download_tool(FBS_DUMPER_REPO, FBS_DUMPER_FILE).await
    }

    pub async fn flatc(&self) -> Result<()> {
        let platform = Self::get_platform(false)?;
        let tool = format!("flatc-{}.zip", platform);
        let url = format!("{}/{}", FLATC_BASE_URL, tool);

        self.download_tool(&url, FLATC_FILE).await
    }
}
