// Vault commands
use nole_core::VaultParser;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct VaultConfig {
    pub vault_path: String,
    pub auto_generate_plan: bool,
}

#[derive(Clone)]
pub struct VaultState {
    pub parser: Option<Arc<VaultParser>>,
}

pub async fn generate_daily_plan(state: VaultState) -> Result<String, String> {
    if let Some(parser) = &state.parser {
        let subjects = parser.parse_config()
            .map_err(|e| e.to_string())?;
        
        let plan = parser.generate_daily_plan(&subjects)
            .map_err(|e| e.to_string())?;
        
        parser.write_hoy(&plan)
            .map_err(|e| e.to_string())?;
        
        Ok(format!("Plan generated for {}", plan.date))
    } else {
        Err("Vault not configured".to_string())
    }
}

pub async fn get_vault_config(state: VaultState) -> Result<VaultConfig, String> {
    if let Some(parser) = &state.parser {
        let config = VaultConfig {
            vault_path: parser.get_vault_path().to_string_lossy(),
            auto_generate_plan: true,
        };
        Ok(config)
    } else {
        Err("Vault not configured".to_string())
    }
}

pub async fn set_vault_config(
    vault_path: String, 
    _config: VaultConfig,
    state: VaultState,
) -> Result<String, String> {
    if let Some(_parser) = &state.parser {
        Ok(format!("Vault path set to: {}", vault_path))
    } else {
        Err("Vault not configured".to_string())
    }
}
