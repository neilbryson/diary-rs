use clap::Subcommand;
use crate::datetime::Datetime;
use crate::db::Db;

#[derive(Subcommand, Debug)]
pub enum CommandTypes {
    /// List diary entries
    List,
    /// Add a new diary entry for the current day
    Add {
        content: String,
    },
}

pub struct Commands;

const DATE_FMT: &str = "[day padding:none] [month repr:long] [year]";
const DATETIME_FMT: &str = "[day padding:none] [month repr:long] [year] [hour]:[minute]:[second]";

impl Commands {
    pub fn run(command: CommandTypes, db: Db) {
        match command {
            CommandTypes::List => {
                Commands::list(db);
            },
            CommandTypes::Add { content } => {
                db.add(content);
            }
        }
    }

    pub fn list(db: Db) {
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
            let date = Datetime::format(entry.date_created.as_str(), DATE_FMT).unwrap();
            let date_created =
                Datetime::format(entry.date_created.as_str(), DATETIME_FMT).unwrap();
            let date_modified =
                Datetime::format(entry.date_modified.as_str(), DATETIME_FMT).unwrap();
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
}