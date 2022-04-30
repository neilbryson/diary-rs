use dirs::home_dir;
use rusqlite::{Connection, Result};
use std::process::exit;

pub struct Db {
    connection: Connection,
}

#[derive(Debug)]
pub struct DiaryEntry {
    pub id: u32,
    pub date: String,
    pub content: String,
    pub date_created: String,
    pub date_modified: String,
}

impl Db {
    pub fn new() -> Self {
        let db_file_name = ".diary-rs.db".to_string();
        let db_file_path = match home_dir() {
            Some(dir) => {
                let mut path = "".to_string();
                let home_dir = String::from(dir.to_string_lossy());
                path.push_str(&home_dir);
                path.push_str("/");
                path.push_str(&db_file_name);
                path
            }
            None => {
                let mut path = "".to_string();
                path.push_str("./");
                path.push_str(&db_file_name);
                path
            }
        };

        let connection = match Connection::open(&db_file_path) {
            Ok(conn) => conn,
            Err(_) => exit(1),
        };

        connection.execute(
            "CREATE TABLE IF NOT EXISTS diary (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date TEXT NOT NULL,
                content TEXT,
                date_created TEXT NOT NULL,
                date_modified TEXT NOT NULL
            )",
            [],
        ).unwrap();

        Self { connection }
    }

    pub fn list(&self) -> Result<Vec<DiaryEntry>> {
        let statement = &mut self
            .connection
            .prepare("SELECT * FROM diary ORDER BY date_created DESC")?;
        let diary_iter = statement.query_map([], |row| {
            Ok(DiaryEntry {
                id: row.get(0)?,
                date: row.get(1)?,
                content: row.get(2)?,
                date_modified: row.get(3)?,
                date_created: row.get(4)?,
            })
        })?;

        let mut diary_entries = Vec::new();

        for diary in diary_iter {
            diary_entries.push(diary.unwrap());
        }

        Ok(diary_entries)
    }
}
