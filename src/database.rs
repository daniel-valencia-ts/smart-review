use crate::utils::Item;
use std::fs::create_dir_all;

use anyhow::{Context, Result, bail};
use directories::ProjectDirs;
use rusqlite::{Connection, params};

pub fn init() -> Result<Connection> {
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
        review_date INTEGER,
        successful_recalls INTEGER,
        time_interval INTEGER,
        easiness_factor REAL
    );
    ",
    )?;
    Ok(conn)
}

pub fn insert_item(database: &mut Connection, item: &Item) -> Result<()> {
    let description = match item.description.clone() {
        Some(desc) => desc,
        None => String::from("None"),
    };

    match database.execute(
        "INSERT INTO items (id, title, description, review_date, successful_recalls, time_interval, easiness_factor) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (item.id, item.title.as_str(), description, item.review_date, item.successful_recalls, item.time_interval, item.easiness_factor)
    )? {
        0 => {
            bail!("the item couldn't be inserted")
        }
        _ => {
            Ok(())
        }
    }
}

pub fn get_item(database: &mut Connection, pattern: &str) -> Result<Item> {
    Ok(database
        .query_row(
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
        )
        .context("no matching item was found")?)
}

pub fn get_all_items(database: &mut Connection) -> Result<Option<Vec<Item>>> {
    let mut stmt = database.prepare("SELECT * FROM items")?;

    let items = stmt
        .query_map([], |row| {
            Ok(Item {
                id: row.get("id")?,
                title: row.get("title")?,
                description: row.get("description")?,
                review_date: row.get("review_date")?,
                successful_recalls: row.get("successful_recalls")?,
                time_interval: row.get("time_interval")?,
                easiness_factor: row.get("easiness_factor")?,
            })
        })?
        .collect::<Result<Vec<_>, rusqlite::Error>>()?;

    match items.len() {
        0 => Ok(None),
        _ => Ok(Some(items)),
    }
}

pub fn update_item(database: &mut Connection, item: Item) -> Result<()> {
    match database.execute(
        "UPDATE items SET review_date = ?1, successful_recalls = ?2, time_interval = ?3, easiness_factor = ?4 WHERE id = ?5",
        params![item.review_date, item.successful_recalls, item.time_interval, item.easiness_factor, item.id]
    )? {
        0 => {
            bail!("the item couldn't be updated")
        }
        _ => {
            Ok(())
        }
    }
}

pub fn remove_item(database: &mut Connection, pattern: &str) -> Result<()> {
    get_item(database, pattern)?; // Check whether the item even exists first for more precise error messages
    match database.execute(
        "DELETE FROM items WHERE id = ?1 OR title = ?1",
        params![pattern],
    )? {
        0 => {
            bail!("the item couldn't be removed")
        }
        _ => Ok(()),
    }
}
