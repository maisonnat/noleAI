use crate::events::EventPublisher;
use notify::{RecursiveMode, Watcher, RecommendedWatcher, Event as NotifyEvent, EventKind};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WatcherError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Watcher error: {0}")]
    WatcherError(#[from] notify::Error),
}

pub type WatcherResult<T> = Result<T, WatcherError>;

pub struct VaultWatcher {
    _watcher: RecommendedWatcher,
    vault_path: PathBuf,
}

impl VaultWatcher {
    pub fn new<P: AsRef<Path>>(
        vault_path: P,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> WatcherResult<Self> {
        let path = vault_path.as_ref().to_path_buf();
        
        let event_handler = move |res: Result<NotifyEvent, _>| {
            if let Ok(event) = res {
                if event.kind.is_modify() || event.kind.is_create() {
                    let publisher = Arc::clone(&event_publisher);
                    tokio::spawn(async move {
                        // Debounce: publish event after short delay
                        tokio::time::sleep(Duration::from_millis(500)).await;
                        let _ = publisher.publish(crate::events::Event::ObsidianVaultChanged).await;
                    });
                }
            }
        };
        
        let mut watcher = notify::recommended_watcher(event_handler)?;
        watcher.watch(&path, RecursiveMode::Recursive)?;
        
        Ok(Self {
            _watcher: watcher,
            vault_path: path,
        })
    }
    
    pub fn get_vault_path(&self) -> &Path {
        &self.vault_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EventBus;
    
    #[test]
    fn test_watcher_creation() {
        let event_bus = Arc::new(EventBus::new());
        let temp_dir = std::env::temp_dir();
        
        // This should not panic
        let result = VaultWatcher::new(&temp_dir, event_bus);
        assert!(result.is_ok());
    }
}
