mod cli;
mod helpers;
mod tools;
mod wrappers;

use crate::cli::args::Args;
use crate::cli::parse;

use baad::helpers::{LoggingConfig, init_logging};
use eyre::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config = LoggingConfig {
        verbose_mode: args.verbose,
        enable_debug: args.verbose,
        ..LoggingConfig::default()
    };
    init_logging(config)?;

    parse::run(args).await
}
