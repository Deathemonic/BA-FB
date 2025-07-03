use anyhow::Result;
use baad::utils::FileManager;
use baad_core::{info, errors::ErrorContext, success};
use reqwest::Url;
use std::env::consts::{ARCH, OS};
use std::rc::Rc;
use trauma::download::Download;
use trauma::downloader::{Downloader, DownloaderBuilder};

pub struct ToolsFetcher {
    file_manager: Rc<FileManager>,
    downloader: Downloader,
}

impl ToolsFetcher {
    pub fn new(file_manager: Rc<FileManager>) -> Result<Self> {
        let downloader = DownloaderBuilder::new()
            .directory(file_manager.get_data_dir().to_path_buf())
            .use_range_for_content_length(true)
            .single_file_progress(true)
            .overwrite(true)
            .build();

        Ok(Self {
            file_manager,
            downloader,
        })
    }

    async fn download(&self, url: &[Download]) -> Result<()> {
        self.downloader.download(url).await;
        
        success!("Successfully downloaded.");
        Ok(())
    }

    fn get_platform(mac_prefix: bool) -> Result<&'static str> {
        match (OS, ARCH) {
            ("windows", "x86_64") => Ok("win-x64"),
            ("macos", "x86_64") => Ok(if mac_prefix { "mac-x64" } else { "osx-x64" }),
            ("macos", "aarch64") => Ok(if mac_prefix { "mac-arm64" } else { "osx-arm64" }),
            ("linux", "x86_64") => Ok("linux-x64"),
            ("linux", "aarch64") => Ok("linux-arm64"),
            _ => None.error_context("Unsupported platform")?,
        }
    }
    
    pub async fn il2cpp_dumper(&self) -> Result<()> {
        info!("Downloading Il2CppInspectorRedux...");
        
        let base_url = "https://nightly.link/LukeFZ/Il2CppInspectorRedux/workflows/build/new-ui";
        let platform = Self::get_platform(false)?;
        let url = format!("{}/Il2CppInspectorRedux.CLI-{}.zip", base_url, platform);
        let filename = self
            .file_manager
            .get_data_path("tools/Il2CppInspectorRedux.zip")
            .to_string_lossy()
            .to_string();

        let il2cpp_dumper = vec![Download {
            url: Url::parse(url.as_str())?,
            filename: filename.to_string(),
            hash: None,
        }];

        self.download(&il2cpp_dumper).await
    }

    pub async fn fbs_dumper(&self) -> Result<()> {
        info!("Downloading FbsDumperV2...");
        
        let base_url = "https://nightly.link/Deathemonic/FbsDumperV2/workflows/build/main";
        let platform = Self::get_platform(false)?;
        let url = format!("{}/FbsDumperV2-{}.zip", base_url, platform);
        let filename = self
            .file_manager
            .get_data_path("tools/FbsDumperV2.zip")
            .to_string_lossy()
            .to_string();

        let fbs_dumper = vec![Download {
            url: Url::parse(url.as_str())?,
            filename,
            hash: None,
        }];

        self.download(&fbs_dumper).await
    }

    pub async fn flatc(&self) -> Result<()> {
        info!("Downloading Flatc...");

        let base_url = "https://deathemonic.github.io/storage/tools/flatc";
        let platform = Self::get_platform(false)?;
        let url = format!("{}/flatc-{}.zip", base_url, platform);
        let filename = self
            .file_manager
            .get_data_path("tools/Flatc.zip")
            .to_string_lossy()
            .to_string();

        let fbs_dumper = vec![Download {
            url: Url::parse(url.as_str())?,
            filename,
            hash: None,
        }];

        self.download(&fbs_dumper).await
    }
}
