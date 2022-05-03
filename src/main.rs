mod datetime;
mod db;
mod commands;

use clap::Parser;
use db::Db;
use commands::{Commands, CommandTypes};

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
