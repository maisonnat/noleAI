#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use nole_core::{EventBus, TimerService, VaultParser, VaultWatcher};
use the_crab_engram::{FileStorage, Storage};
use nole_hud::{TimerState, VaultState, SessionState};
use std::sync::Arc;
use std::path::PathBuf;
use tauri::State;

#[derive(Clone)]
struct AppState {
    event_bus: Arc<EventBus>,
    timer_state: TimerState,
    vault_state: VaultState,
    session_state: SessionState,
    vault_path: PathBuf,
}

#[tauri::command]
async fn trigger_overload_mode(state: State<'_, AppState>) -> Result<String, String> {
    state.event_bus.publish(nole_core::Event::OverloadModeActivated).await
        .map_err(|e| e.to_string())?;
    Ok("Overload mode activated".to_string())
}

fn main() {
    let vault_path = PathBuf::from("./vault");
    
    let event_bus = Arc::new(EventBus::new());
    let timer_service = TimerService::new(1500, Arc::clone(&event_bus));  // 25 minutes
    
    // Create vault directory if it doesn't exist
    std::fs::create_dir_all(&vault_path).unwrap();
    let vault_parser = VaultParser::new(&vault_path).unwrap();
    
    let storage = Arc::new(FileStorage::new(".engram-data")) as Arc<dyn Storage>;
    
    // Start vault watcher
    let _watcher = VaultWatcher::new(&vault_path, Arc::clone(&event_bus))
        .expect("Failed to start vault watcher");
    
    tauri::Builder::default()
        .manage(AppState {
            event_bus,
            timer_state: TimerState { service: Arc::new(timer_service) },
            vault_state: VaultState { parser: Arc::new(vault_parser) },
            session_state: SessionState { storage },
            vault_path,
        })
        .invoke_handler(tauri::generate_handler![
            nole_hud::start_timer,
            nole_hud::pause_timer,
            nole_hud::resume_timer,
            nole_hud::stop_timer,
            nole_hud::get_timer_state,
            nole_hud::generate_daily_plan,
            nole_hud::save_session,
            nole_hud::get_mastery_level,
            trigger_overload_mode,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
