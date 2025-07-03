use crate::wrappers::flatc::Language;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "bafb")]
#[command(about = "A tool for dumping and generating Blue Archive flatbuffers")]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Force update
    #[arg(short, long)]
    pub update: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Dump {
        #[command(subcommand)]
        target: DumpTarget,
    },
    Generate {
        /// FlatBuffers schema file
        #[arg(short, long)]
        fbs: PathBuf,

        /// Programming language to generate
        #[arg(short, long)]
        language: Language,

        /// Output path
        #[arg(short, long)]
        output: PathBuf,
    },
}

#[derive(Subcommand)]
pub enum DumpTarget {
    Global {
        /// Output path
        #[arg(short, long)]
        output: PathBuf,
    },
    Japan {
        /// Output path
        #[arg(short, long)]
        output: PathBuf,
    },
}