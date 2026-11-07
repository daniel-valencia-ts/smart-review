use crate::utils::Item;
use std::fs::create_dir_all;

use directories::ProjectDirs;
use rusqlite::Connection;

pub fn init() -> Result<Connection, Box<dyn std::error::Error>> {
    let path = ProjectDirs::from("", "daniel-valencia-ts", "smart-review")
        .unwrap()
        .data_dir()
        .to_path_buf();
    create_dir_all(&path)?;

    let conn = Connection::open(path.join("database.db"))?;
    conn.execute_batch(
        "
    CREATE TABLE IF NOT EXISTS items (
        id INTEGER PRIMARY KEY,
        title TEXT NOT NULL,
        description TEXT NULL,
        review_date TEXT NOT NULL,
        successful_recalls INTEGER,
        time_interval INTEGER,
        easiness_factor REAL
    );
    ",
    )?;
    Ok(conn)
}

pub fn insert_item(database: &mut Connection, item: &Item) -> Result<usize, rusqlite::Error> {
    let description = match item.description.clone() {
        Some(desc) => desc,
        None => String::from("None"),
    };

    database.execute(
        "INSERT INTO items (id, title, description, review_date, successful_recalls, time_interval, easiness_factor) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (item.id, item.title.as_str(), description, item.review_date.as_str(), item.successful_recalls, item.time_interval, item.easiness_factor)
    )
}
