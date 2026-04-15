// Timer commands
use nole_core::{Timer, TimerService};
use std::sync::Arc;

#[derive(Clone)]
pub struct TimerState {
    pub service: Arc<TimerService>,
}

pub async fn start_timer(state: TimerState) -> Result<String, String> {
    let mut timer = (*state.service).clone();
    timer.start().await.map_err(|e| e.to_string())?;
    Ok("Timer started".to_string())
}

pub async fn pause_timer(state: TimerState) -> Result<String, String> {
    let mut timer = (*state.service).clone();
    timer.pause().await.map_err(|e| e.to_string())?;
    Ok("Timer paused".to_string())
}

pub async fn resume_timer(state: TimerState) -> Result<String, String> {
    let mut timer = (*state.service).clone();
    timer.resume().await.map_err(|e| e.to_string())?;
    Ok("Timer resumed".to_string())
}

pub async fn stop_timer(state: TimerState) -> Result<String, String> {
    let mut timer = (*state.service).clone();
    timer.stop().await.map_err(|e| e.to_string())?;
    Ok("Timer stopped".to_string())
}

pub async fn get_timer_state(state: TimerState) -> Result<nole_core::TimerState, String> {
    let timer = (*state.service).clone();
    Ok(timer.get_state())
}
