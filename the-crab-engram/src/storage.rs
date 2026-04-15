use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during storage operations.
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Not found: {0}")]
    NotFound(String),
}

/// Result alias for storage operations.
pub type StorageResult<T> = Result<T, StorageError>;

/// A record of a completed study session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRecord {
    /// When the session occurred.
    pub timestamp: DateTime<Utc>,
    /// Subject studied.
    pub subject: String,
    /// Session duration in seconds.
    pub duration_seconds: u32,
    /// Number of breaks taken during the session.
    pub breaks: u32,
}

/// Mastery level for a subject, tracked over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasteryLevel {
    /// Subject name.
    pub subject: String,
    /// Mastery level on a 1-5 scale.
    pub level: u8,
    /// When the mastery level was last updated.
    pub last_updated: DateTime<Utc>,
    /// Total number of reviews completed.
    pub review_count: u32,
}

/// Detailed metrics for a study session, used by anti-pattern detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetrics {
    /// Unique session identifier.
    pub session_id: String,
    /// Session start time.
    pub start_time: DateTime<Utc>,
    /// Session end time (None if in progress).
    pub end_time: Option<DateTime<Utc>>,
    /// Subject being studied.
    pub subject: String,
    /// Number of breaks taken.
    pub breaks_taken: u32,
    /// Consecutive seconds studied without a break.
    pub duration_without_breaks: u32,
}

/// Trait for persistent storage of sessions, mastery, and metrics.
///
/// Implementations MUST be Send + Sync for use across async boundaries.
#[async_trait]
pub trait Storage: Send + Sync {
    /// Save a session record.
    async fn save_session(&self, session: &SessionRecord) -> StorageResult<()>;
    /// Retrieve all session records.
    async fn get_sessions(&self) -> StorageResult<Vec<SessionRecord>>;
    /// Get the mastery level for a specific subject.
    async fn get_mastery(&self, subject: &str) -> StorageResult<Option<MasteryLevel>>;
    /// Update or insert a mastery level record.
    async fn update_mastery(&self, mastery: &MasteryLevel) -> StorageResult<()>;
    /// Get all mastery levels.
    async fn get_all_mastery(&self) -> StorageResult<Vec<MasteryLevel>>;
    /// Persist session metrics for anti-pattern analysis.
    async fn save_session_metrics(&self, metrics: &SessionMetrics) -> StorageResult<()>;
    /// Retrieve all session metrics.
    async fn get_session_metrics(&self) -> StorageResult<Vec<SessionMetrics>>;
}

/// JSON file-based storage implementation.
///
/// Stores data in `base_path/sessions.json`, `base_path/mastery.json`,
/// and `base_path/session_metrics.json`.
pub struct FileStorage {
    base_path: PathBuf,
}

impl FileStorage {
    /// Create a new file storage backed by the given directory.
    ///
    /// The directory will be created with restrictive permissions on first use.
    pub fn new<P: Into<PathBuf>>(base_path: P) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    fn ensure_directory_exists(&self) -> StorageResult<()> {
        std::fs::create_dir_all(&self.base_path)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(0o700);
            let _ = std::fs::set_permissions(&self.base_path, perms);
        }
        Ok(())
    }

    fn sessions_path(&self) -> PathBuf {
        self.base_path.join("sessions.json")
    }

    fn mastery_path(&self) -> PathBuf {
        self.base_path.join("mastery.json")
    }

    fn metrics_path(&self) -> PathBuf {
        self.base_path.join("session_metrics.json")
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

    async fn save_session_metrics(&self, metrics: &SessionMetrics) -> StorageResult<()> {
        self.ensure_directory_exists()?;
        let mut all_metrics = self.get_session_metrics().await.unwrap_or_default();
        all_metrics.push(metrics.clone());
        let content = serde_json::to_string_pretty(&all_metrics)?;
        std::fs::write(self.metrics_path(), content)?;
        Ok(())
    }

    async fn get_session_metrics(&self) -> StorageResult<Vec<SessionMetrics>> {
        if !self.metrics_path().exists() {
            return Ok(Vec::new());
        }
        let content = std::fs::read_to_string(self.metrics_path())?;
        let metrics: Vec<SessionMetrics> = serde_json::from_str(&content)?;
        Ok(metrics)
    }
}
