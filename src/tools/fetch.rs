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
            .directory(file::data_dir()?)
            .use_range_for_content_length(true)
            .single_file_progress(true)
            .overwrite(true)
            .build();

        let client = Client::builder()
            .user_agent("BA-FB/1.4 (Blue Archive - FlatBuffer)")
            .build()?;

        Ok(Self { downloader, client })
    }

    async fn download(&self, url: &[Download]) -> Result<()> {
        self.downloader.download(url, None).await;

        info!(success = true, "Successfully downloaded.");
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
            _ => Err(eyre!("Unsupported platform")),
        }
    }

    fn find_asset<'a>(assets: &'a [GitHubAsset], suffix: &str) -> Result<&'a GitHubAsset> {
        assets
            .iter()
            .find(|asset| asset.name.contains(suffix))
            .ok_or_else(|| eyre!("No asset found for platform: {}", suffix))
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

        let file_path = file::get_data_path(&format!("{}/{}", TOOLS_DIR, filename))?
            .to_string_lossy()
            .into();

        if url.starts_with(HTTP_PREFIX) || url.starts_with(HTTPS_PREFIX) {
            let download = vec![Download {
                url: Url::parse(url)?,
                filename: file_path,
                target_file: None,
                hash: None,
            }];
            self.download(&download).await
        } else {
            let release = self.fetch_github(url).await?;
            let platform = Self::get_platform(false)?;
            let asset = Self::find_asset(&release.assets, platform)?;

            let download = vec![Download {
                url: Url::parse(&asset.browser_download_url)?,
                filename: file_path,
                target_file: None,
                hash: None,
            }];
            self.download(&download).await
        }
    }

    pub async fn il2cpp_dumper(&self) -> Result<()> {
        self.download_tool(IL2CPP_INSPECTOR_REPO, IL2CPP_INSPECTOR_FILE)
            .await
    }

    pub async fn fbs_dumper(&self) -> Result<()> {
        self.download_tool(FBS_DUMPER_REPO, FBS_DUMPER_FILE).await
    }

    pub async fn flatc(&self) -> Result<()> {
        let platform = Self::get_platform(true)?;
        let tool = format!("flatc-{}.zip", platform);
        let url = format!("{}/{}", FLATC_BASE_URL, tool);

        self.download_tool(&url, FLATC_FILE).await
    }
}
