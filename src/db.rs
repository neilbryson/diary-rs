use std::process::exit;
use rusqlite::{params, Connection, Result};
use dirs::home_dir;

pub struct Db {
    connection: Connection,
}

#[derive(Debug)]
struct DiaryEntry {
    id: u32,
    date: String,
    content: String,
    date_created: String,
    date_modified: String,
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
            },
            None => {
                let mut path = "".to_string();
                path.push_str("./");
                path.push_str(&db_file_name);
                path
            }
        };

        let connection = match Connection::open(&db_file_path) {
            Ok(conn) => conn,
            Err(_) => {
                exit(1)
            },
        };

        let create_exec = connection.execute(
            "CREATE TABLE IF NOT EXISTS diary (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date TEXT NOT NULL,
                content TEXT,
                date_created TEXT NOT NULL,
                date_modified TEXT NOT NULL
            )",
            []
        );

        if let Ok(_) = create_exec {
            connection.execute(
                "INSERT INTO diary (date, content, date_created, date_modified)
                VALUES (?1, ?2, ?3, ?4)",
                params!["2022-01-01", "Hello world", "2022-01-01 03:04:05", "2022-04-01 15:16:17"]
            ).unwrap();
        }

        Self { connection }
    }

    pub fn list(&self) -> Result<()> {
        let mut statement = &mut self.connection.prepare("SELECT * FROM diary")?;
        let diary_iter = statement.query_map([], |row| {
            Ok(DiaryEntry {
                id: row.get(0)?,
                date: row.get(1)?,
                content: row.get(2)?,
                date_modified: row.get(3)?,
                date_created: row.get(4)?,
            })
        })?;

        for diary in diary_iter {
            println!("Found diary {:?}", diary.unwrap());
        }

        Ok(())
    }
}
