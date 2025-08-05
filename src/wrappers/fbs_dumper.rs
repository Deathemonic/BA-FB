use anyhow::Result;
use baad_core::errors::{ErrorContext, ErrorExt};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;

pub struct FbsDumper {
    binary: PathBuf,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FbsDumperOptions {
    pub dummy_dll: PathBuf,
    pub game_assembly: PathBuf,
    pub output_file: Option<PathBuf>,
    pub namespace: Option<String>,
    pub force_snake_case: bool,
    pub namespace_to_look_for: Option<String>,
}

impl FbsDumper {
    pub fn new(binary: PathBuf) -> Result<Self> {
        if !binary.exists() {
            return None.error_context(&format!("Binary path: {}", binary.display()));
        }
        Ok(Self { binary })
    }

    pub fn run(&self, options: FbsDumperOptions) -> Result<()> {
        let mut cmd = Command::new(&self.binary);

        cmd.arg("-dummydll").arg(&options.dummy_dll);
        cmd.arg("-gameassembly").arg(&options.game_assembly);

        if let Some(output_file) = &options.output_file {
            cmd.arg("-outputfile").arg(output_file);
        }

        if let Some(namespace) = &options.namespace {
            cmd.arg("-namespace").arg(namespace);
        }

        if options.force_snake_case {
            cmd.arg("-forcesnakecase");
        }

        if let Some(namespace_to_look_for) = &options.namespace_to_look_for {
            cmd.arg("-namespacetolookfor").arg(namespace_to_look_for);
        }

        let status = cmd.status()
            .handle_errors()
            .error_context(&format!("Failed to execute FbsDumper at {}", self.binary.display()))?;

        if !status.success() {
            return Err(anyhow::anyhow!(
                "FbsDumper failed with exit code {:?}",
                status.code()
            )).error_context("FbsDumper execution failed");
        }

        Ok(())
    }
}