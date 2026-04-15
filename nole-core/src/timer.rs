use crate::events::{EventPublisher, Event, EventResult};
use std::sync::{Arc, Mutex};

/// Immutable snapshot of the timer's current state.
#[derive(Debug, Clone)]
pub struct TimerState {
    /// Total session duration in seconds.
    pub duration: u32,
    /// Seconds elapsed since start.
    pub elapsed: u32,
    /// Whether the timer is currently paused.
    pub is_paused: bool,
    /// Whether the timer is actively running.
    pub is_running: bool,
}

impl TimerState {
    /// Create a new timer state with the given duration.
    pub fn new(duration_seconds: u32) -> Self {
        Self {
            duration: duration_seconds,
            elapsed: 0,
            is_paused: false,
            is_running: false,
        }
    }

    /// Remaining seconds in the session (0 when elapsed >= duration).
    pub fn get_remaining_seconds(&self) -> u32 {
        if self.elapsed >= self.duration {
            0
        } else {
            self.duration - self.elapsed
        }
    }
}

/// Trait for timer operations (start, pause, resume, stop).
#[async_trait::async_trait]
pub trait Timer: Send + Sync {
    /// Start the timer from the beginning.
    async fn start(&mut self) -> EventResult<()>;
    /// Pause a running timer.
    async fn pause(&mut self) -> EventResult<()>;
    /// Resume a paused timer.
    async fn resume(&mut self) -> EventResult<()>;
    /// Stop and reset the timer.
    async fn stop(&mut self) -> EventResult<()>;
    /// Get a snapshot of the current timer state.
    fn get_state(&self) -> TimerState;
    /// Get remaining seconds without cloning the full state.
    fn get_remaining_seconds(&self) -> u32;
}

/// Tokio-based timer service that publishes tick and break events.
///
/// Emits `TimerTick` every second while running, `TimerBreakRequested`
/// when the session completes, and `AntiPatternDetected` after 45 minutes
/// without a break.
pub struct TimerService {
    state: Arc<Mutex<TimerState>>,
    event_publisher: Arc<dyn EventPublisher>,
}

/// Threshold in seconds before warning about a long session (45 minutes).
const LONG_SESSION_THRESHOLD_SECS: u32 = 2700;

impl TimerService {
    /// Create a new timer service with `duration_seconds` total session length.
    pub fn new(duration_seconds: u32, event_publisher: Arc<dyn EventPublisher>) -> Self {
        Self {
            state: Arc::new(Mutex::new(TimerState::new(duration_seconds))),
            event_publisher,
        }
    }

    /// Process a single tick. Called externally at the desired interval.
    ///
    /// Publishes `TimerTick`, checks for session completion, and emits
    /// `AntiPatternDetected` every 45 minutes.
    async fn tick(&self) -> EventResult<()> {
        let mut state = self.state.lock().unwrap();
        if !state.is_paused && state.is_running {
            state.elapsed += 1;
            let remaining = state.get_remaining_seconds();

            self.event_publisher.publish(Event::TimerTick { remaining_secs: remaining }).await?;

            if remaining == 0 {
                state.is_running = false;
                self.event_publisher.publish(Event::TimerBreakRequested).await?;
            }

            if state.elapsed > 0 && state.elapsed % LONG_SESSION_THRESHOLD_SECS == 0 {
                self.event_publisher.publish(Event::AntiPatternDetected {
                    pattern_type: "long_session".to_string(),
                    message: format!(
                        "You've been studying for {} minutes without a break. Consider pausing.",
                        state.elapsed / 60
                    ),
                }).await?;
            }
        }
        Ok(())
    }
}

#[async_trait::async_trait]
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
