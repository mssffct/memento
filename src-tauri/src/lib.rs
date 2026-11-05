use rusqlite::{Connection, Result as SqlResult};
use std::path::PathBuf;
use tauri::command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Unit {
    id: i64,
    entity_type: String,
    last_name: Option<String>,
    first_name: Option<String>,
    middle_name: Option<String>,
    pet_name: Option<String>,
    birth_date: Option<String>,
    gender: Option<String>,
    photo_path: Option<String>,
}

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
        "CREATE TABLE IF NOT EXISTS units (
            id INTEGER PRIMARY KEY,
            name TEXT
        )",
        [],
    ).map_err(|e| e.to_string())?;

    let units_columns = vec![
        ("entity_type", "TEXT DEFAULT 'person'"),
        ("last_name", "TEXT"),
        ("first_name", "TEXT"),
        ("middle_name", "TEXT"),
        ("pet_name", "TEXT"),
        ("birth_date", "TEXT"),
        ("gender", "TEXT"),
        ("photo_path", "TEXT"),
    ];

    for (col_name, col_type) in units_columns {
        let sql = format!("ALTER TABLE units ADD COLUMN {} {}", col_name, col_type);
        let _ = conn.execute(&sql, []);
    };

    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM units",
        [],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    if count == 0 {
         conn.execute(
            "INSERT INTO units (entity_type, last_name, first_name, middle_name, birth_date, gender)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            ("person", "Петров", "Иван", "Сергеевич", "1950-03-15", "male"),
        ).map_err(|e| e.to_string())?;
        
        conn.execute(
            "INSERT INTO units (entity_type, pet_name, birth_date, gender)
             VALUES (?1, ?2, ?3, ?4)",
            ("pet", "Шарик", "2015-06-01", "male"),
        ).map_err(|e| e.to_string())?;
    }

    let mut stmt = conn.prepare(
        "SELECT COALESCE(last_name || ' ' || first_name, pet_name, name) FROM units ORDER BY id"
    ).map_err(|e| e.to_string())?;

    let names = stmt.query_map([], |row| row.get::<_, String>(0))
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect::<Vec<String>>();

    Ok(names)
}

#[command]
fn get_all_units(password: String) -> Result<Vec<Unit>, String> {
    let conn = open_db(&password).map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT id, entity_type, last_name, first_name, middle_name, pet_name, birth_date, gender, photo_path 
         FROM units ORDER BY id"
    ).map_err(|e| e.to_string())?;

    let units = stmt.query_map([], |row| {
        Ok(Unit {
            id: row.get(0)?,
            entity_type: row.get(1)?,
            last_name: row.get(2)?,
            first_name: row.get(3)?,
            middle_name: row.get(4)?,
            pet_name: row.get(5)?,
            birth_date: row.get(6)?,
            gender: row.get(7)?,
            photo_path: row.get(8)?,
        })
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();

    Ok(units)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![init_and_test_db, get_all_units])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
