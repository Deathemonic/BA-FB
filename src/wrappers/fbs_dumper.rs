use eyre::{eyre, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;

pub struct FbsDumper {
    binary: PathBuf,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FbsDumperOptions {
    pub url: Option<String>,
    pub dummy_dll: PathBuf,
    pub game_assembly: Option<PathBuf>,
    pub output_file: Option<PathBuf>,
    pub namespace: Option<String>,
    pub force_snake_case: bool,
    pub namespace_to_look_for: Option<String>,
    pub force: bool,
    pub verbose: bool,
    pub suppress_warnings: bool,
}

impl FbsDumper {
    pub fn new(binary: PathBuf) -> Result<Self> {
        if !binary.exists() {
            return Err(eyre!(format!("Binary path: {}", binary.display())));
        }
        Ok(Self { binary })
    }

    pub fn run(&self, options: FbsDumperOptions) -> Result<()> {
        let mut cmd = Command::new(&self.binary);

        cmd.arg("--dummy-dll").arg(&options.dummy_dll);

        if let Some(game_assembly) = &options.game_assembly {
            cmd.arg("--game-assembly").arg(game_assembly);
        }

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
            cmd.arg("--namespace-to-look-for")
                .arg(namespace_to_look_for);
        }

        if options.force {
            cmd.arg("--force");
        }

        if options.verbose {
            cmd.arg("--verbose");
        }

        if options.suppress_warnings {
            cmd.arg("--suppress-warnings");
        }

        let status = cmd.status().wrap_err_with(|| {
            format!("Failed to execute FbsDumper at {}", self.binary.display())
        })?;

        if !status.success() {
            return Err(eyre!("FbsDumper failed with exit code {:?}", status.code()))
                .wrap_err_with(|| "FbsDumper execution failed");
        }

        Ok(())
    }
}
