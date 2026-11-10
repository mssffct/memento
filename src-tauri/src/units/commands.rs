use tauri::command;
use super::repository;
use super::types::Unit;


#[command]
pub fn init_and_test_db(password: String) -> Result<(), String> {
    let conn = repository::open_db(&password).map_err(|e| e.to_string())?;
    repository::init_schema(&conn).map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub fn get_all_units(password: String) -> Result<Vec<Unit>, String> {
    let conn = repository::open_db(&password).map_err(|e| e.to_string())?;
    repository::get_all_units(&conn).map_err(|e| e.to_string())
}

#[command]
pub fn save_unit(password: String, unit: Unit) -> Result<i64, String> {
    let conn = repository::open_db(&password).map_err(|e| e.to_string())?;
    repository::save_unit(&conn, &unit).map_err(|e| e.to_string())
}