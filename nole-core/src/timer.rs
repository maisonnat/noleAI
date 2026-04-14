use crate::events::{EventPublisher, Event, EventResult};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::interval;

#[derive(Debug, Clone)]
pub struct TimerState {
    pub duration: u32,         // Total duration in seconds
    pub elapsed: u32,         // Elapsed time in seconds
    pub is_paused: bool,
    pub is_running: bool,
}

impl TimerState {
    pub fn new(duration_seconds: u32) -> Self {
        Self {
            duration: duration_seconds,
            elapsed: 0,
            is_paused: false,
            is_running: false,
        }
    }
    
    pub fn get_remaining_seconds(&self) -> u32 {
        if self.elapsed >= self.duration {
            0
        } else {
            self.duration - self.elapsed
        }
    }
}

#[async_trait]
pub trait Timer: Send + Sync {
    async fn start(&mut self) -> EventResult<()>;
    async fn pause(&mut self) -> EventResult<()>;
    async fn resume(&mut self) -> EventResult<()>;
    async fn stop(&mut self) -> EventResult<()>;
    fn get_state(&self) -> TimerState;
    fn get_remaining_seconds(&self) -> u32;
}

pub struct TimerService {
    state: Arc<Mutex<TimerState>>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl TimerService {
    pub fn new(duration_seconds: u32, event_publisher: Arc<dyn EventPublisher>) -> Self {
        Self {
            state: Arc::new(Mutex::new(TimerState::new(duration_seconds))),
            event_publisher,
        }
    }
    
    async fn tick(&self) -> EventResult<()> {
        let mut state = self.state.lock().unwrap();
        if !state.is_paused && state.is_running {
            state.elapsed += 1;
            let remaining = state.get_remaining_seconds();
            
            // Publish timer tick event
            self.event_publisher.publish(Event::TimerTick { remaining_secs: remaining }).await?;
            
            // Check if timer completed
            if remaining == 0 {
                state.is_running = false;
                self.event_publisher.publish(Event::TimerBreakRequested).await?;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Timer for TimerService {
    async fn start(&mut self) -> EventResult<()> {
        let mut state = self.state.lock().unwrap();
        if !state.is_running {
            state.is_running = true;
            state.elapsed = 0;
            state.is_paused = false;
        }
        Ok(())
    }
    
    async fn pause(&mut self) -> EventResult<()> {
        let mut state = self.state.lock().unwrap();
        if state.is_running && !state.is_paused {
            state.is_paused = true;
        }
        Ok(())
    }
    
    async fn resume(&mut self) -> EventResult<()> {
        let mut state = self.state.lock().unwrap();
        if state.is_running && state.is_paused {
            state.is_paused = false;
        }
        Ok(())
    }
    
    async fn stop(&mut self) -> EventResult<()> {
        let mut state = self.state.lock().unwrap();
        state.is_running = false;
        state.is_paused = false;
        state.elapsed = 0;
        Ok(())
    }
    
    fn get_state(&self) -> TimerState {
        self.state.lock().unwrap().clone()
    }
    
    fn get_remaining_seconds(&self) -> u32 {
        self.state.lock().unwrap().get_remaining_seconds()
    }
}

impl Clone for TimerService {
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
            event_publisher: Arc::clone(&self.event_publisher),
        }
    }
}
