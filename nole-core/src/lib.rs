pub mod events;
pub mod vault;

pub use events::{Event, EventBus, EventPublisher};

// Re-exports para facilitar el uso desde otros módulos
pub struct NoleSystem;

impl NoleSystem {
    pub fn new() -> Self {
        Self
    }
}
