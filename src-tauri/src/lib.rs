mod units;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            units::commands::init_and_test_db, 
            units::commands::get_all_units,
            units::commands::save_unit,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
