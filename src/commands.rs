use crate::datetime::{Datetime, DATETIME_FMT, DATE_FMT};
use crate::db::Db;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum CommandTypes {
    /// List diary entries
    List,
    /// Add a new diary entry for the current day
    Add { content: String },
}

pub struct Commands;

impl Commands {
    pub fn run(command: CommandTypes, db: Db) {
        match command {
            CommandTypes::List => {
                Commands::list(db);
            }
            CommandTypes::Add { content } => {
                let add = db.add(content);
                if add.is_err() {
                    println!("Unable to add due to \"{}\"", add.unwrap_err().to_string());
                }
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
            let date_created = Datetime::format(entry.date_created.as_str(), DATETIME_FMT).unwrap();
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
