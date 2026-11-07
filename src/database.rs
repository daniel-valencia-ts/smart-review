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
        description TEXT NOT NULL,
        review_date TEXT NOT NULL,
        successful_recalls INTEGER,
        time_interval INTEGER,
        easiness_factor REAL
    );
    ",
    )?;
    Ok(conn)
}
