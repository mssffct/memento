use rusqlite::{Connection, Result as SqlResult, params};
use std::path::PathBuf;
use super::types::Unit;


pub fn db_path() -> PathBuf {
    let mut path = dirs::data_dir().expect("Failed to get data dir");
    path.push("memento");
    std::fs::create_dir_all(&path).expect("Failed to create app folder");
    path.push("main.db");
    path
}

pub fn open_db(password: &str) -> SqlResult<Connection> {
    let conn = Connection::open(db_path())?;
    conn.pragma_update(None, "key", password)?;
    Ok(conn)
}

pub fn init_schema(conn: &Connection) -> SqlResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS units (
            id INTEGER PRIMARY KEY,
            name TEXT
        )",
        [],
    )?;

    let columns_to_add = vec![
        ("entity_type", "TEXT DEFAULT 'person'"),
        ("last_name", "TEXT"),
        ("first_name", "TEXT"),
        ("middle_name", "TEXT"),
        ("pet_name", "TEXT"),
        ("gender", "TEXT"),
        ("photo_path", "TEXT"),
        ("birth_year", "INTEGER"),
        ("birth_year_qual", "TEXT"),
        ("birth_month", "INTEGER"),
        ("birth_month_qual", "TEXT"),
        ("birth_day", "INTEGER"),
        ("birth_day_qual", "TEXT"),
        ("death_year", "INTEGER"),
        ("death_year_qual", "TEXT"),
        ("death_month", "INTEGER"),
        ("death_month_qual", "TEXT"),
        ("death_day", "INTEGER"),
        ("death_day_qual", "TEXT"),
    ];

    for (col, typ) in columns_to_add {
        let sql = format!("ALTER TABLE units ADD COLUMN {} {}", col, typ);
        let _ = conn.execute(&sql, []); // игнорируем ошибку "колонка уже есть"
    }

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM units", [], |r| r.get(0))?;
    if count == 0 {
        conn.execute(
            "INSERT INTO units (entity_type, last_name, first_name, middle_name, gender)
             VALUES ('person', 'Петров', 'Иван', 'Сергеевич', 'male')",
            [],
        )?;
        conn.execute(
            "INSERT INTO units (entity_type, pet_name, gender)
             VALUES ('pet', 'Шарик', 'male')",
            [],
        )?;
    }
    Ok(())
}

pub fn get_all_units(conn: &Connection) -> SqlResult<Vec<Unit>> {
    let mut stmt = conn.prepare(
        "SELECT id, entity_type, last_name, first_name, middle_name, pet_name, gender, photo_path,
                birth_year, birth_year_qual, birth_month, birth_month_qual, birth_day, birth_day_qual,
                death_year, death_year_qual, death_month, death_month_qual, death_day, death_day_qual
         FROM units ORDER BY id",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(Unit {
            id: row.get(0)?,
            entity_type: row.get(1)?,
            last_name: row.get(2)?,
            first_name: row.get(3)?,
            middle_name: row.get(4)?,
            pet_name: row.get(5)?,
            gender: row.get(6)?,
            photo_path: row.get(7)?,
            birth_year: row.get(8)?,
            birth_year_qual: row.get(9)?,
            birth_month: row.get(10)?,
            birth_month_qual: row.get(11)?,
            birth_day: row.get(12)?,
            birth_day_qual: row.get(13)?,
            death_year: row.get(14)?,
            death_year_qual: row.get(15)?,
            death_month: row.get(16)?,
            death_month_qual: row.get(17)?,
            death_day: row.get(18)?,
            death_day_qual: row.get(19)?,
        })
    })?;

    rows.collect()
}

pub fn save_unit(conn: &Connection, unit: &Unit) -> SqlResult<i64> {
    let id_param: Option<i64> = if unit.id == 0 { None } else { Some(unit.id) };

    conn.execute(
        "INSERT INTO units (
            id, entity_type, last_name, first_name, middle_name, pet_name, gender, photo_path,
            birth_year, birth_year_qual, birth_month, birth_month_qual, birth_day, birth_day_qual,
            death_year, death_year_qual, death_month, death_month_qual, death_day, death_day_qual
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)
        ON CONFLICT(id) DO UPDATE SET
            entity_type = excluded.entity_type,
            last_name = excluded.last_name,
            first_name = excluded.first_name,
            middle_name = excluded.middle_name,
            pet_name = excluded.pet_name,
            gender = excluded.gender,
            photo_path = excluded.photo_path,
            birth_year = excluded.birth_year,
            birth_year_qual = excluded.birth_year_qual,
            birth_month = excluded.birth_month,
            birth_month_qual = excluded.birth_month_qual,
            birth_day = excluded.birth_day,
            birth_day_qual = excluded.birth_day_qual,
            death_year = excluded.death_year,
            death_year_qual = excluded.death_year_qual,
            death_month = excluded.death_month,
            death_month_qual = excluded.death_month_qual,
            death_day = excluded.death_day,
            death_day_qual = excluded.death_day_qual",
        params![
            id_param,
            unit.entity_type, unit.last_name, unit.first_name, unit.middle_name,
            unit.pet_name, unit.gender, unit.photo_path,
            unit.birth_year, unit.birth_year_qual, unit.birth_month, unit.birth_month_qual,
            unit.birth_day, unit.birth_day_qual,
            unit.death_year, unit.death_year_qual, unit.death_month, unit.death_month_qual,
            unit.death_day, unit.death_day_qual,
        ],
    )?;

    if unit.id == 0 {
        Ok(conn.last_insert_rowid())
    } else {
        Ok(unit.id)
    }
}