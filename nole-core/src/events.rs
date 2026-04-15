use async_channel::{Sender, Receiver, unbounded};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use thiserror::Error;

/// All events that flow through the NoleAI event bus.
///
/// Events are published by services (timer, watcher, overload) and
/// consumed by the HUD for real-time UI updates.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Event {
    TimerTick { remaining_secs: u32 },
    TimerBreakRequested,
    SessionStarted { subject: String },
    SessionEnded { subject: String, duration_seconds: u32 },
    ObsidianVaultChanged,
    OverloadModeActivated,
    OverloadPlanGenerated { simplified_tasks: Vec<String> },
    AntiPatternDetected { pattern_type: String, message: String },
    KnowledgeGraphUpdated { node_count: usize },
    SessionMetricsUpdated { session_id: String },
}

/// Errors that can occur during event bus operations.
#[derive(Error, Debug)]
pub enum EventError {
    #[error("Channel send error: {0}")]
    SendError(#[from] async_channel::SendError<Event>),
    #[error("Channel receive error: {0}")]
    ReceiveError(#[from] async_channel::RecvError),
}

/// Result type for event operations.
pub type EventResult<T> = Result<T, EventError>;

/// Trait for publishing events to the event bus.
///
/// Implemented by `EventBus` and can be mocked for testing.
#[async_trait]
pub trait EventPublisher: Send + Sync {
    /// Publish an event to all subscribers.
    async fn publish(&self, event: Event) -> EventResult<()>;
}

/// Broadcast event bus using async channels.
///
/// Create one via `EventBus::new()`, clone it for multiple producers/consumers.
pub struct EventBus {
    sender: Sender<Event>,
    receiver: Receiver<Event>,
}

impl EventBus {
    /// Create a new event bus with an unbounded channel.
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        Self { sender, receiver }
    }

    /// Get cloned sender and receiver for wiring up consumers.
    pub fn channel(&self) -> (Sender<Event>, Receiver<Event>) {
        (self.sender.clone(), self.receiver.clone())
    }
}

#[async_trait]
impl EventPublisher for EventBus {
    async fn publish(&self, event: Event) -> EventResult<()> {
        self.sender.send(event).await?;
        Ok(())
    }
}

impl Clone for EventBus {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            receiver: self.receiver.clone(),
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
