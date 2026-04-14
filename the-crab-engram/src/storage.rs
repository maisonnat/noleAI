use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Not found: {0}")]
    NotFound(String),
}

pub type StorageResult<T> = Result<T, StorageError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRecord {
    pub timestamp: DateTime<Utc>,
    pub subject: String,
    pub duration_seconds: u32,
    pub breaks: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasteryLevel {
    pub subject: String,
    pub level: u8,
    pub last_updated: DateTime<Utc>,
    pub review_count: u32,
}

#[async_trait]
pub trait Storage: Send + Sync {
    async fn save_session(&self, session: &SessionRecord) -> StorageResult<()>;
    async fn get_sessions(&self) -> StorageResult<Vec<SessionRecord>>;
    async fn get_mastery(&self, subject: &str) -> StorageResult<Option<MasteryLevel>>;
    async fn update_mastery(&self, mastery: &MasteryLevel) -> StorageResult<()>;
    async fn get_all_mastery(&self) -> StorageResult<Vec<MasteryLevel>>;
}

pub struct FileStorage {
    base_path: PathBuf,
}

impl FileStorage {
    pub fn new<P: Into<PathBuf>>(base_path: P) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }
    
    fn ensure_directory_exists(&self) -> StorageResult<()> {
        std::fs::create_dir_all(&self.base_path)?;
        Ok(())
    }
    
    fn sessions_path(&self) -> PathBuf {
        self.base_path.join("sessions.json")
    }
    
    fn mastery_path(&self) -> PathBuf {
        self.base_path.join("mastery.json")
    }
}

#[async_trait]
impl Storage for FileStorage {
    async fn save_session(&self, session: &SessionRecord) -> StorageResult<()> {
        self.ensure_directory_exists()?;
        
        let mut sessions = self.get_sessions().await.unwrap_or_default();
        sessions.push(session.clone());
        
        let content = serde_json::to_string_pretty(&sessions)?;
        std::fs::write(self.sessions_path(), content)?;
        
        Ok(())
    }
    
    async fn get_sessions(&self) -> StorageResult<Vec<SessionRecord>> {
        if !self.sessions_path().exists() {
            return Ok(Vec::new());
        }
        
        let content = std::fs::read_to_string(self.sessions_path())?;
        let sessions: Vec<SessionRecord> = serde_json::from_str(&content)?;
        Ok(sessions)
    }
    
    async fn get_mastery(&self, subject: &str) -> StorageResult<Option<MasteryLevel>> {
        let all_mastery = self.get_all_mastery().await?;
        Ok(all_mastery.into_iter().find(|m| m.subject == subject))
    }
    
    async fn update_mastery(&self, mastery: &MasteryLevel) -> StorageResult<()> {
        self.ensure_directory_exists()?;
        
        let mut all_mastery = self.get_all_mastery().await?;
        
        if let Some(existing) = all_mastery.iter_mut().find(|m| m.subject == mastery.subject) {
            *existing = mastery.clone();
        } else {
            all_mastery.push(mastery.clone());
        }
        
        let content = serde_json::to_string_pretty(&all_mastery)?;
        std::fs::write(self.mastery_path(), content)?;
        
        Ok(())
    }
    
    async fn get_all_mastery(&self) -> StorageResult<Vec<MasteryLevel>> {
        if !self.mastery_path().exists() {
            return Ok(Vec::new());
        }
        
        let content = std::fs::read_to_string(self.mastery_path())?;
        let mastery: Vec<MasteryLevel> = serde_json::from_str(&content)?;
        Ok(mastery)
    }
}
