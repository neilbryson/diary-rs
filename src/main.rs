mod datetime;
mod db;

use clap::{Parser, Subcommand};
use datetime::Datetime;
use db::Db;

#[derive(Subcommand, Debug)]
enum Commands {
    /// Lists diary entries
    List,
    Add {
        content: String,
    },
}

/// A command line diary program!
#[derive(Parser, Debug)]
#[clap(name = "diary-rs")]
#[clap(version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

fn main() {
    let db = Db::new();
    let args = Cli::parse();
    let dmy_fmt = "[day padding:none] [month repr:long] [year]";
    let datetime_fmt = "[day padding:none] [month repr:long] [year] [hour]:[minute]:[second]";

    match args.command {
        Commands::List => {
            let result = db.list();
            if result.is_err() {
                return;
            }

            let diary_entries = result.unwrap();

            if diary_entries.len() == 0 {
                println!("No diary entries yet. Start writing one today!");
                return;
            }

            for entry in diary_entries.iter() {
                let date = Datetime::format(entry.date_created.as_str(), &dmy_fmt).unwrap();
                let date_created =
                    Datetime::format(entry.date_created.as_str(), &datetime_fmt).unwrap();
                let date_modified =
                    Datetime::format(entry.date_modified.as_str(), &datetime_fmt).unwrap();
                println!(
                    "===============================================================\n
                    Diary entry for {}\n\n
{}\n

Created on {}
Modified on {}",
                    date, entry.content, date_created, date_modified
                )
            }
        }
        Commands::Add { content } => {
            db.add(content);
        }
    }
}
