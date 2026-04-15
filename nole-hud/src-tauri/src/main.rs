#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use nole_core::{EventBus, TimerService, VaultParser, VaultWatcher, KnowledgeGraphService};
use nole_crab_engram::{FileStorage, Storage};
use nole_hud::{TimerState, VaultState, SessionState, KnowledgeGraphState, StressTestState};
use std::sync::Arc;
use std::path::PathBuf;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    event_bus: Arc<EventBus>,
    timer_state: TimerState,
    vault_state: VaultState,
    session_state: SessionState,
    kg_state: KnowledgeGraphState,
    stress_test_state: StressTestState,
    vault_path: PathBuf,
}

#[tauri::command]
pub async fn trigger_overload_mode(state: AppState) -> Result<String, String> {
    state.event_bus.publish(nole_core::Event::OverloadModeActivated).await
        .map_err(|e| e.to_string())?;
    Ok("Overload mode activated".to_string())
}

fn main() {
    let vault_path = PathBuf::from("./vault");

    let event_bus = Arc::new(EventBus::new());
    let timer_service = TimerService::new(1500, Arc::clone(&event_bus));

    std::fs::create_dir_all(&vault_path).unwrap();
    let vault_parser = VaultParser::new(&vault_path).unwrap();

    let storage = Arc::new(FileStorage::new(".engram-data")) as Arc<dyn Storage>;

    let _watcher = VaultWatcher::new(&vault_path, Arc::clone(&event_bus))
        .expect("Failed to start vault watcher");

    let kg_service = Arc::new(Mutex::new(KnowledgeGraphService::new()));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            app.manage(AppState {
                event_bus,
                timer_state: TimerState { service: Arc::new(timer_service) },
                vault_state: VaultState { parser: Arc::new(Some(
                    VaultParser::new(&vault_path).unwrap()
                )) },
                session_state: SessionState { storage },
                kg_state: KnowledgeGraphState {
                    service: kg_service,
                    vault_parser: Some(Arc::new(VaultParser::new(&vault_path).unwrap())),
                },
                stress_test_state: StressTestState {
                    runner: Arc::new(Mutex::new(None)),
                },
                vault_path,
            });
            Ok(())
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
            nole_hud::get_vault_config,
            nole_hud::set_vault_config,
            nole_hud::get_knowledge_graph,
            nole_hud::run_stress_test,
            trigger_overload_mode,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
