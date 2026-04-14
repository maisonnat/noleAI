use async_channel::{Sender, Receiver, unbounded};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Event {
    // Timer events
    TimerTick { remaining_secs: u32 },
    TimerBreakRequested,
    SessionStarted { subject: String },
    SessionEnded { subject: String, duration_seconds: u32 },
    
    // Vault events
    ObsidianVaultChanged,
    
    // System events
    OverloadModeActivated,
    OverloadPlanGenerated { simplified_tasks: Vec<String> },
    
    // Anti-pattern events
    AntiPatternDetected { pattern_type: String, message: String },
    
    // Knowledge graph events
    KnowledgeGraphUpdated { node_count: usize },
    
    // Session tracking events
    SessionMetricsUpdated { session_id: String },
}

#[derive(Error, Debug)]
pub enum EventError {
    #[error("Channel send error: {0}")]
    SendError(#[from] async_channel::SendError<Event>),
    #[error("Channel receive error: {0}")]
    ReceiveError(#[from] async_channel::RecvError),
}

pub type EventResult<T> = Result<T, EventError>;

#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event: Event) -> EventResult<()>;
}

pub struct EventBus {
    sender: Sender<Event>,
    receiver: Receiver<Event>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        Self { sender, receiver }
    }
    
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
