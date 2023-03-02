use std::path::PathBuf;
use std::str::FromStr;

use structopt::clap::AppSettings;
use structopt::StructOpt;

use crate::chunk_type::ChunkType;
use crate::commands::{extract, insert, list, remove};

#[derive(StructOpt)]
#[structopt(global_settings(& [AppSettings::VersionlessSubcommands]))]
pub struct Cli {
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(StructOpt)]
pub enum Subcommand {
    Extract(ExtractArgs),
    Insert(InsertArgs),
    Remove(RemoveArgs),
    List(ListArgs),
}

#[derive(StructOpt)]
pub struct ExtractArgs {
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
    #[structopt(parse(try_from_str = ChunkType::from_str))]
    pub chunk_type: ChunkType,
    #[structopt(parse(from_os_str))]
    pub output: PathBuf,
}

#[derive(StructOpt)]
pub struct InsertArgs {
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
    #[structopt(parse(from_os_str))]
    pub chunk: PathBuf,
    #[structopt(parse(from_os_str))]
    pub output: PathBuf,
}

#[derive(StructOpt)]
pub struct RemoveArgs {
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
    #[structopt(parse(try_from_str = ChunkType::from_str))]
    pub chunk_type: ChunkType,
    #[structopt(parse(from_os_str))]
    pub output: PathBuf,
}

#[derive(StructOpt)]
pub struct ListArgs {
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
}

pub fn run(subcommand: &Subcommand) -> Result<(), Box<dyn std::error::Error>> {
    match subcommand {
        Subcommand::Extract(args) => extract(args),
        Subcommand::Insert(args) => insert(args),
        Subcommand::Remove(args) => remove(args),
        Subcommand::List(args) => list(args),
        _ => unimplemented!(),
    }
}

