use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

/// Enumeración central de eventos del sistema NoleAI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    SessionStarted { subject: String },
    SessionEnded { duration_mins: u32 },
    ObsidianVaultChanged,
    OverloadModeActivated,
}

/// Trait para publicar eventos en el sistema
pub trait EventPublisher: Send + Sync {
    fn publish(&self, event: Event) -> Result<()>;
}

/// Implementación simple de un Event Bus asíncrono
pub struct EventBus {
    sender: broadcast::Sender<Event>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }
}

impl EventPublisher for EventBus {
    fn publish(&self, event: Event) -> Result<()> {
        self.sender.send(event)?;
        Ok(())
    }
}
