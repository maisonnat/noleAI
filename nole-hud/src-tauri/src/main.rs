// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use nole_core::vault;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;

// Importar los mocks de inteligencia
use the_crab_engram::Engram;
use notebooklm_rust_mcp::McpBridge;

// Estado global de la aplicación
struct AppState {
    current_task: Mutex<String>,
    is_studying: Mutex<bool>,
    engram: Engram,
    mcp: McpBridge,
}

#[tauri::command]
fn get_daily_task(state: State<AppState>) -> String {
    let vault_path = PathBuf::from("../../vault");
    
    match vault::parse_materias(&vault_path) {
        Ok(materias) => {
            if materias.is_empty() {
                "Configurar materias".to_string()
            } else {
                let task = materias[0].clone();
                *state.current_task.lock().unwrap() = task.clone();
                task
            }
        },
        Err(_) => "Error al leer vault".to_string()
    }
}

#[tauri::command]
fn start_session(state: State<AppState>) -> String {
    let mut studying = state.is_studying.lock().unwrap();
    if *studying {
        return "Sesión ya en curso".to_string();
    }
    
    *studying = true;
    let task = state.current_task.lock().unwrap().clone();
    
    // Hito 4: Conectar con Engram al iniciar (mock)
    state.engram.mem_save(&task, 25).unwrap_or_else(|e| eprintln!("{}", e));
    
    format!("🟢 Sesión iniciada para: {}", task)
}

#[tauri::command]
fn trigger_overload_mode() -> String {
    // Hito 5: Activar lógica de "No puedo" (Anti-patrones)
    println!("🚨 [TDAH] Modo Sobrecarga activado. Ocultando complejidad...");
    "Respira. Simplificando vista...".to_string()
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            current_task: Mutex::new("Cargando...".to_string()),
            is_studying: Mutex::new(false),
            engram: Engram::new(),
            mcp: McpBridge::new(),
        })
        .invoke_handler(tauri::generate_handler![get_daily_task, start_session, trigger_overload_mode])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
