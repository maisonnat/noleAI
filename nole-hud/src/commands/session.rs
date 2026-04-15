// Session commands
use the_crab_engram::{SessionRecord, MasteryLevel, Storage};
use std::sync::Arc;

#[derive(Clone)]
pub struct SessionState {
    pub storage: Arc<dyn Storage>,
}

pub async fn save_session(
    subject: String,
    duration_seconds: u32,
    breaks: u32,
    state: SessionState,
) -> Result<String, String> {
    let session = SessionRecord {
        timestamp: chrono::Utc::now(),
        subject,
        duration_seconds,
        breaks,
    };
    
    state.storage.save_session(&session).await
        .map_err(|e| e.to_string())?;
    
    Ok("Session saved".to_string())
}

pub async fn get_mastery_level(
    subject: String,
    state: SessionState,
) -> Result<Option<MasteryLevel>, String> {
    state.storage.get_mastery(&subject).await
        .map_err(|e| e.to_string())
}
