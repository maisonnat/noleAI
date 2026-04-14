// Vault commands
use nole_core::VaultParser;
use std::sync::Arc;
use tauri::State;

#[derive(Clone)]
pub struct VaultState {
    pub parser: Arc<VaultParser>,
}

pub async fn generate_daily_plan(state: State<'_, VaultState>) -> Result<String, String> {
    let subjects = state.parser.parse_config()
        .map_err(|e| e.to_string())?;
    
    let plan = state.parser.generate_daily_plan(&subjects)
        .map_err(|e| e.to_string())?;
    
    state.parser.write_hoy(&plan)
        .map_err(|e| e.to_string())?;
    
    Ok(format!("Plan generated for {}", plan.date))
}
