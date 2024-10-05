use std::error::Error;
use rusqlite::{Connection, Result};

pub struct DbConfig {
    pub path: String,
    pub encrypt: bool,
}

pub fn init_db(config: &DbConfig) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(&config.path,)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clipboard (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL,
            type TEXT NOT NULL DEFAULT 'text',
            created_at TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub fn save_clipboard_content(content: &String) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("clipboard.db")?;
    let db_config = DbConfig {
        path: "clipboard.db".to_string(),
        encrypt: false,
    };
    init_db(&db_config)?;
    conn.execute(
        "INSERT INTO clipboard (content, created_at) VALUES (?1, datetime('now'))",
        &[content],
    )?;
    Ok(())
}

pub fn get_clipboard_content() -> Result<Vec<String>, Box<dyn Error>> {
    let conn = Connection::open("clipboard.db")?;
    let mut stmt = conn.prepare("SELECT content FROM clipboard ORDER BY created_at DESC")?;
    let rows = stmt.query_map([], |row| row.get(0))?;
    let mut contents = Vec::new();
    for content in rows {
        contents.push(content?);
    }
    Ok(contents)
}
