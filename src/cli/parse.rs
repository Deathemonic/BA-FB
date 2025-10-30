use crate::cli::args::{Args, Commands, DumpTarget};
use crate::cli::config::Config;
use crate::helpers::config::*;
use crate::tools::extract::ToolsExtractor;
use crate::tools::fetch::ToolsFetcher;
use crate::wrappers::fbs_dumper::{FbsDumper, FbsDumperOptions};
use crate::wrappers::flatc::{FlatC, FlatCOptions, Language};
use crate::wrappers::il2cpp_dumper::{Il2CppDumper, Il2CppDumperOptions};

use baad::apk::{ApkExtractor, ApkFetcher};
use baad::helpers::{ServerConfig, ServerRegion};
use baad::info;
use baad::utils::file;
use clap::CommandFactory;
use eyre::Result;
use std::path::{Path, PathBuf};
use std::rc::Rc;

pub struct CommandHandler {
    args: Args,
    config: Config,
}

impl CommandHandler {
    pub fn new(args: Args) -> Result<Self> {
        let config = Config::load(args.config.as_deref())?;
        Ok(Self { args, config })
    }

    pub async fn handle(&self) -> Result<()> {
        if self.args.clean {
            info!("Cleaning data...");

            let tools_dir = file::get_data_path("tools")?;
            file::clear_all(&tools_dir).await?;

            info!(success = true, "Data cleared");
        }

        match &self.args.command {
            Some(Commands::Dump { target }) => self.handle_dump(target).await,
            Some(Commands::Generate {
                fbs,
                language,
                output,
            }) => self.execute_generate(fbs, language, output).await,
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
            DumpTarget::Japan { output } => self.execute_dump(ServerRegion::Japan, output).await,
            DumpTarget::Global { output } => self.execute_dump(ServerRegion::Global, output).await,
        }
    }

    async fn handle_update(&self) -> Result<()> {
        info!("Forcing update...");

        let japan_config = ServerConfig::new(ServerRegion::Japan, None, None)?;
        let apk_fetcher = ApkFetcher::new(japan_config.clone())?;
        apk_fetcher.download_apk(true).await?;

        let global_config = ServerConfig::new(ServerRegion::Global, None, None)?;
        let apk_fetcher = ApkFetcher::new(global_config.clone())?;
        apk_fetcher.download_apk(true).await?;

        let tool_fetcher = ToolsFetcher::new()?;
        tool_fetcher
            .il2cpp_dumper(
                self.config
                    .il2cpp_dumper
                    .url
                    .as_deref()
                    .unwrap_or(IL2CPP_INSPECTOR_REPO),
            )
            .await?;
        tool_fetcher
            .fbs_dumper(
                self.config
                    .fbs_dumper
                    .url
                    .as_deref()
                    .unwrap_or(FBS_DUMPER_REPO),
            )
            .await?;
        tool_fetcher
            .flatc(self.config.flatc.url.as_deref().unwrap_or(FLATC_REPO))
            .await?;

        let tool_extractor = ToolsExtractor::new()?;
        tool_extractor.il2cpp_dumper(
            self.config
                .il2cpp_dumper
                .binary_name
                .as_deref()
                .unwrap_or(IL2CPP_INSPECTOR_BINARY),
            true,
        )?;
        tool_extractor.fbs_dumper(
            self.config
                .fbs_dumper
                .binary_name
                .as_deref()
                .unwrap_or(FBS_DUMPER_BINARY),
            true,
        )?;
        tool_extractor.flatc(
            self.config
                .flatc
                .binary_name
                .as_deref()
                .unwrap_or(FLATC_BINARY),
            true,
        )?;

        Ok(())
    }

    async fn execute_dump(&self, region: ServerRegion, output: &Path) -> Result<()> {
        let server_config = ServerConfig::new(region, None, None)?;

        self.prepare_dump_files(&server_config).await?;
        let (il2cpp_dumper, fbs_dumper) = self.prepare_dumper_tools()?;
        self.run_il2cpp_dumper(&il2cpp_dumper, &server_config, output)?;
        self.run_fbs_dumper(&fbs_dumper, &server_config, output)?;

        Ok(())
    }

    async fn execute_generate(&self, fbs: &Path, language: &Language, output: &Path) -> Result<()> {
        self.prepare_generate_files().await?;
        let flatc = self.prepare_generate_tools()?;
        self.run_flatc(&flatc, language, fbs, output)?;

        Ok(())
    }

    async fn prepare_generate_files(&self) -> Result<()> {
        let tool_fetcher = ToolsFetcher::new()?;

        let il2cpp_zip_path = file::get_data_path("tools/Flatc.zip")?;
        if !il2cpp_zip_path.exists() {
            tool_fetcher
                .flatc(self.config.flatc.url.as_deref().unwrap_or(FLATC_REPO))
                .await?;
        }

        Ok(())
    }

    fn prepare_generate_tools(&self) -> Result<FlatC> {
        let tool_extractor = ToolsExtractor::new()?;

        let flatc_bin = tool_extractor.flatc(
            self.config
                .flatc
                .binary_name
                .as_deref()
                .unwrap_or(FLATC_BINARY),
            false,
        )?;
        let flatc = FlatC::new(flatc_bin)?;

        Ok(flatc)
    }

    async fn prepare_dump_files(&self, server_config: &Rc<ServerConfig>) -> Result<()> {
        let apk_fetcher = ApkFetcher::new(server_config.clone())?;
        let apk_extractor = ApkExtractor::new(server_config.clone())?;
        let tool_fetcher = ToolsFetcher::new()?;

        apk_fetcher.download_apk(false).await?;

        let (libil2cpp, metadata) = self.get_il2cpp_paths(server_config);
        if !libil2cpp.exists() || !metadata.exists() {
            apk_extractor.extract_il2cpp()?;
        }

        let il2cpp_zip_path = file::get_data_path("tools/Il2CppInspectorRedux.zip")?;
        if !il2cpp_zip_path.exists() {
            tool_fetcher
                .il2cpp_dumper(
                    self.config
                        .il2cpp_dumper
                        .url
                        .as_deref()
                        .unwrap_or(IL2CPP_INSPECTOR_REPO),
                )
                .await?;
        }

        let fbs_zip_path = file::get_data_path("tools/FbsDumperV2.zip")?;
        if !fbs_zip_path.exists() {
            tool_fetcher
                .fbs_dumper(
                    self.config
                        .fbs_dumper
                        .url
                        .as_deref()
                        .unwrap_or(FBS_DUMPER_REPO),
                )
                .await?;
        }

        Ok(())
    }

    fn prepare_dumper_tools(&self) -> Result<(Il2CppDumper, FbsDumper)> {
        let tool_extractor = ToolsExtractor::new()?;

        let il2cppdumper_bin = tool_extractor.il2cpp_dumper(
            self.config
                .il2cpp_dumper
                .binary_name
                .as_deref()
                .unwrap_or(IL2CPP_INSPECTOR_BINARY),
            false,
        )?;
        let fbsdumper_bin = tool_extractor.fbs_dumper(
            self.config
                .fbs_dumper
                .binary_name
                .as_deref()
                .unwrap_or(FBS_DUMPER_BINARY),
            false,
        )?;

        let il2cpp_dumper = Il2CppDumper::new(il2cppdumper_bin)?;
        let fbs_dumper = FbsDumper::new(fbsdumper_bin)?;

        Ok((il2cpp_dumper, fbs_dumper))
    }

    fn get_il2cpp_paths(&self, server_config: &ServerConfig) -> (PathBuf, PathBuf) {
        let libil2cpp = match server_config.region {
            ServerRegion::Japan => file::get_data_path("il2cpp/japan/libil2cpp.so"),
            ServerRegion::Global => file::get_data_path("il2cpp/global/libil2cpp.so"),
        };

        let metadata = match server_config.region {
            ServerRegion::Japan => file::get_data_path("il2cpp/japan/global-metadata.dat"),
            ServerRegion::Global => file::get_data_path("il2cpp/global/global-metadata.dat"),
        };

        (
            libil2cpp.expect("Unable to get il2cpp"),
            metadata.expect("Unable to get metadata"),
        )
    }

    fn run_il2cpp_dumper(
        &self,
        il2cpp_dumper: &Il2CppDumper,
        server_config: &ServerConfig,
        output: &Path,
    ) -> Result<()> {
        info!("Dumping il2cpp...");

        let (libil2cpp, metadata) = self.get_il2cpp_paths(server_config);

        let mut il2cpp_options = Il2CppDumperOptions {
            il2cpp: libil2cpp,
            metadata: Some(metadata),
            output: Some(output.to_path_buf()),
            output_csharp_stub: true,
            output_dummy_dlls: true,
            output_disassembler_metadata: true,
            disassembler: Some("IDA".to_string()),
            ..Default::default()
        };

        self.config.merge_il2cpp_dumper_config(&mut il2cpp_options);

        il2cpp_dumper.run(il2cpp_options)?;

        info!(success = true, "Successfully dumped");

        Ok(())
    }

    fn run_fbs_dumper(
        &self,
        fbs_dumper: &FbsDumper,
        server_config: &ServerConfig,
        output: &Path,
    ) -> Result<()> {
        info!("Dumping fbs...");

        let (libil2cpp, _) = self.get_il2cpp_paths(server_config);

        let mut fbs_options = FbsDumperOptions {
            dummy_dll: output.join("dll"),
            game_assembly: Some(libil2cpp),
            output_file: Some(output.join("BlueArchive.fbs")),
            namespace: Some(match server_config.region {
                ServerRegion::Japan => "Japan".to_string(),
                ServerRegion::Global => "Global".to_string(),
            }),
            ..Default::default()
        };

        self.config.merge_fbs_dumper_config(&mut fbs_options);

        fbs_dumper.run(fbs_options)?;

        info!(success = true, "Successfully dumped");

        Ok(())
    }

    fn run_flatc(
        &self,
        flatc: &FlatC,
        languages: &Language,
        fbs: &Path,
        output: &Path,
    ) -> Result<()> {
        info!("Generating flatbuffers...");

        let mut flatc_options = FlatCOptions {
            languages: vec![*languages],
            output_path: Some(output.to_path_buf()),
            no_warnings: true,
            scoped_enums: true,
            gen_object_api: true,
            rust_module_root_file: true,
            rust_serialize: true,
            ..Default::default()
        };

        self.config.merge_flatc_config(&mut flatc_options);

        flatc.compile(flatc_options, vec![fbs.to_path_buf()], vec![])?;

        info!(success = true, "Successfully generated");

        Ok(())
    }
}

pub async fn run(args: Args) -> Result<()> {
    if args.command.is_none() && !args.update && !args.clean {
        Args::command().print_help()?;
        std::process::exit(0);
    }

    let handler = CommandHandler::new(args)?;
    handler.handle().await
}
