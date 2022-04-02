mod db;

use clap::Parser;
use db::Db;

/// A command line diary program!
#[derive(Parser, Debug)]
#[clap(version, about)]
struct Cli {
    command: String,
}

fn main() {
    let db = Db::new();
    let args = Cli::parse();

    println!("{:#?}", args);
    db.list();
}
