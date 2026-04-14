// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use nole_core::vault;
use std::path::PathBuf;

#[tauri::command]
fn get_daily_task() -> String {
    // Usamos un path relativo por ahora. En producción se leerá de la config.
    let vault_path = PathBuf::from("../../vault");
    
    match vault::parse_materias(&vault_path) {
        Ok(materias) => {
            if materias.is_empty() {
                "Configurar materias".to_string()
            } else {
                materias[0].clone()
            }
        },
        Err(_) => "Error al leer vault".to_string()
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_daily_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
