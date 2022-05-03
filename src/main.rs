mod commands;
mod datetime;
mod db;

use clap::Parser;
use commands::{CommandTypes, Commands};
use db::Db;

/// A command line diary program!
#[derive(Parser, Debug)]
#[clap(name = "diary-rs")]
#[clap(version, about)]
struct Cli {
    #[clap(subcommand)]
    command: CommandTypes,
}

fn main() {
    let db = Db::new();
    let args = Cli::parse();

    Commands::run(args.command, db)
}
