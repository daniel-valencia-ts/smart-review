use crate::utils::Item;
use std::fs::create_dir_all;

use directories::ProjectDirs;
use rusqlite::{Connection, params};

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

pub fn get_item(
    database: &mut Connection,
    pattern: &str,
) -> Result<Item, Box<dyn std::error::Error>> {
    Ok(database.query_row(
        "SELECT * FROM items WHERE id = ?1 OR title = ?1",
        [pattern],
        |row| {
            Ok(Item {
                id: row.get("id")?,
                title: row.get("title")?,
                description: row.get("description")?,
                review_date: row.get("review_date")?,
                successful_recalls: row.get("successful_recalls")?,
                time_interval: row.get("time_interval")?,
                easiness_factor: row.get("easiness_factor")?,
            })
        },
    )?)
}

pub fn update_item(database: &mut Connection, item: Item) -> Result<usize, rusqlite::Error> {
    database.execute(
        "UPDATE items SET review_date = ?1, successful_recalls = ?2, time_interval = ?3, easiness_factor = ?4 WHERE id = ?5",
        params![item.review_date, item.successful_recalls, item.time_interval, item.easiness_factor, item.id]
    )
}

pub fn remove_item(database: &mut Connection, pattern: &str) -> Result<usize, rusqlite::Error> {
    database.execute("DELETE FROM items WHERE id = ?1 OR title = ?1", params![pattern])
}