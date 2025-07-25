mod cli;
mod wrappers;
mod tools;

use crate::cli::args::Args;
use crate::cli::parse;

use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    parse::run(args).await
}
