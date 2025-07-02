use crate::cli::args::{Args, Commands, DumpTarget, GenerateTarget};
use crate::tools::extract::ToolsExtractor;
use crate::tools::fetch::ToolsFetcher;
use crate::wrappers::fbs_dumper::{FbsDumper, FbsDumperOptions};
use crate::wrappers::il2cpp_dumper::{Il2CppDumper, Il2CppDumperOptions};

use anyhow::Result;
use baad::apk::{ApkExtractor, ApkFetcher};
use baad::helpers::{ServerConfig, ServerRegion};
use baad::utils::FileManager;
use baad_core::info;
use std::path::PathBuf;
use std::rc::Rc;

pub struct CommandHandler {
    args: Args,
}

impl CommandHandler {
    pub fn new(args: Args) -> Result<Self> {
        Ok(Self { args })
    }

    pub async fn handle(&self) -> Result<()> {
        match &self.args.command {
            Some(Commands::Dump { target }) => {
                self.handle_dump(target).await
            }
            Some(Commands::Generate { target }) => {
                self.handle_generate(target).await
            }
            None => {
                if self.args.update {
                    self.handle_update().await?;
                }
                Ok(())
            }
        }
    }

    async fn handle_dump(&self, target: &DumpTarget) -> Result<()> {
        match target {
            DumpTarget::Japan { output } => {
                self.execute_dump(ServerRegion::Japan, output).await
            }
            DumpTarget::Global { output } => {
                self.execute_dump(ServerRegion::Global, output).await
            }
        }
    }

    async fn handle_update(&self) -> Result<()> {
        info!("Forcing update...");

        let file_manager = FileManager::new()?;
        
        let japan_config = ServerConfig::new(ServerRegion::Japan)?;
        let apk_fetcher = ApkFetcher::new(file_manager.clone(), japan_config.clone())?;
        apk_fetcher.download_apk(true).await?;

        let global_config = ServerConfig::new(ServerRegion::Global)?;
        let apk_fetcher = ApkFetcher::new(file_manager.clone(), global_config.clone())?;
        apk_fetcher.download_apk(true).await?;
        
        let tool_fetcher = ToolsFetcher::new(file_manager.clone())?;
        tool_fetcher.il2cpp_dumper().await?;
        tool_fetcher.fbs_dumper().await?;

        let tool_extractor = ToolsExtractor::new(file_manager.clone())?;
        tool_extractor.il2cpp_dumper(true)?;
        tool_extractor.fbs_dumper(true)?;

        Ok(())
    }

    async fn execute_dump(&self, region: ServerRegion, output: &PathBuf) -> Result<()> {
        let server_config = ServerConfig::new(region)?;
        let file_manager = FileManager::new()?;

        self.prepare_dump_files(&server_config, &file_manager).await?;
        let (il2cpp_dumper, fbs_dumper) = self.prepare_dumper_tools(&file_manager)?;
        self.run_il2cpp_dumper(&il2cpp_dumper, &server_config, &file_manager, output)?;
        self.run_fbs_dumper(&fbs_dumper, &server_config, &file_manager, output)?;

        Ok(())
    }

    async fn prepare_dump_files(&self, server_config: &Rc<ServerConfig>, file_manager: &Rc<FileManager>) -> Result<()> {
        let apk_fetcher = ApkFetcher::new(file_manager.clone(), server_config.clone())?;
        let apk_extractor = ApkExtractor::new(file_manager.clone(), server_config.clone())?;
        let tool_fetcher = ToolsFetcher::new(file_manager.clone())?;

        apk_fetcher.download_apk(false).await?;

        let (libil2cpp, metadata) = self.get_il2cpp_paths(server_config, file_manager);
        if !libil2cpp.exists() || !metadata.exists() {
            apk_extractor.extract_il2cpp()?;
        }

        let il2cpp_zip_path = file_manager.get_data_path("tools/Il2CppInspectorRedux.zip");
        if !il2cpp_zip_path.exists() {
            tool_fetcher.il2cpp_dumper().await?;
        }

        let fbs_zip_path = file_manager.get_data_path("tools/FbsDumperV2.zip");
        if !fbs_zip_path.exists() {
            tool_fetcher.fbs_dumper().await?;
        }

        Ok(())
    }

    fn prepare_dumper_tools(&self, file_manager: &Rc<FileManager>) -> Result<(Il2CppDumper, FbsDumper)> {
        let tool_extractor = ToolsExtractor::new(file_manager.clone())?;

        let il2cppdumper_bin = tool_extractor.il2cpp_dumper(false)?;
        let fbsdumper_bin = tool_extractor.fbs_dumper(false)?;

        let il2cpp_dumper = Il2CppDumper::new(il2cppdumper_bin)?;
        let fbs_dumper = FbsDumper::new(fbsdumper_bin)?;

        Ok((il2cpp_dumper, fbs_dumper))
    }

    fn get_il2cpp_paths(&self, server_config: &ServerConfig, file_manager: &FileManager) -> (PathBuf, PathBuf) {
        let libil2cpp = match server_config.region {
            ServerRegion::Japan => file_manager.get_data_path("il2cpp/japan/libil2cpp.so"),
            ServerRegion::Global => file_manager.get_data_path("il2cpp/global/libil2cpp.so"),
        };

        let metadata = match server_config.region {
            ServerRegion::Japan => file_manager.get_data_path("il2cpp/japan/global-metadata.dat"),
            ServerRegion::Global => file_manager.get_data_path("il2cpp/global/global-metadata.dat"),
        };

        (libil2cpp, metadata)
    }

    fn run_il2cpp_dumper(&self, il2cpp_dumper: &Il2CppDumper, server_config: &ServerConfig, file_manager: &FileManager, output: &PathBuf) -> Result<()> {
        let (libil2cpp, metadata) = self.get_il2cpp_paths(server_config, file_manager);

        let il2cpp_options = Il2CppDumperOptions {
            binary_files: vec![libil2cpp],
            metadata_file: Some(metadata),
            cs_out: Some(output.join("types.cs")),
            py_out: Some(output.join("il2cpp.py")),
            json_out: Some(output.join("metadata.json")),
            dll_out: Some(output.join("dummy")),
            ..Default::default()
        };

        il2cpp_dumper.run(il2cpp_options)
    }

    fn run_fbs_dumper(&self, fbs_dumper: &FbsDumper, server_config: &ServerConfig, file_manager: &FileManager, output: &PathBuf) -> Result<()> {
        let (libil2cpp, _) = self.get_il2cpp_paths(server_config, file_manager);

        let fbs_options = FbsDumperOptions {
            dummy_dir: output.join("dummy"),
            libil2cpp_path: libil2cpp,
            output_file: Some(output.join("BlueArchive.fbs")),
            ..Default::default()
        };

        fbs_dumper.run(fbs_options)
    }

    async fn handle_generate(&self, _target: &GenerateTarget) -> Result<()> {
        Ok(())
    }
}

pub async fn run(args: Args) -> Result<()> {
    let handler = CommandHandler::new(args)?;
    handler.handle().await
}