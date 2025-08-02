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
    pub dummy_dir: PathBuf,
    pub libil2cpp_path: PathBuf,
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

        cmd.arg("--dummy-dir").arg(&options.dummy_dir);
        cmd.arg("--libil2cpp-path").arg(&options.libil2cpp_path);

        if let Some(output_file) = &options.output_file {
            cmd.arg("--output-file").arg(output_file);
        }

        if let Some(namespace) = &options.namespace {
            cmd.arg("--namespace").arg(namespace);
        }

        if options.force_snake_case {
            cmd.arg("--force-snake-case");
        }

        if let Some(namespace_to_look_for) = &options.namespace_to_look_for {
            cmd.arg("--namespace-to-look-for").arg(namespace_to_look_for);
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