mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;


use structopt::StructOpt;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = args::Cli::from_args();
    args::run(&args.subcommand)
}
