use rusqlite::{Connection, Result as SqlResult};
use std::path::PathBuf;
use tauri::command;

fn db_path() -> PathBuf {
    let mut path = dirs::data_dir().expect("Failed to get data dir");
    path.push("memento");
    std::fs::create_dir_all(&path).expect("Failed to create app folder");
    path.push("main.db");
    path
}

fn open_db(password: &str) -> SqlResult<Connection> {
    let conn = Connection::open(db_path())?;
    conn.pragma_update(None, "key", password)?;
    Ok(conn)
}

#[command]
fn init_and_test_db(password: String) -> Result<Vec<String>, String> {
    let conn = open_db(&password).map_err(|e| e.to_string())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS people (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            birth_date TEXT
        )",
        [],
    ).map_err(|e| e.to_string())?;

    // Проверяем, есть ли уже записи, чтобы не дублировать при повторном вызове
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM people",
        [],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    if count == 0 {
        conn.execute(
            "INSERT INTO people (name, birth_date) VALUES (?1, ?2)",
            ("Иван Петров", "1950-03-15"),
        ).map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT INTO people (name, birth_date) VALUES (?1, ?2)",
            ("Мария Петрова", "1955-07-22"),
        ).map_err(|e| e.to_string())?;
    }

    let mut stmt = conn.prepare("SELECT name FROM people ORDER BY id").map_err(|e| e.to_string())?;
    let names = stmt.query_map([], |row| row.get::<_, String>(0))
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect::<Vec<String>>();

    Ok(names)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![init_and_test_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
