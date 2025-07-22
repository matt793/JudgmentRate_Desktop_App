// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod calculator;
mod commands;
mod db;
mod models;
mod rate_fetcher;

use commands::{
    calculate, delete_state_rate_command, get_all_state_rates, get_api_key_configured,
    set_api_key, update_state_rate_command, validate_api_key_command,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialize database
            db::init_db(&app.handle()).expect("Failed to initialize database");
            
            // Initialize logging
            env_logger::init();
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            calculate,
            get_all_state_rates,
            update_state_rate_command,
            delete_state_rate_command,
            set_api_key,
            get_api_key_configured,
            validate_api_key_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
