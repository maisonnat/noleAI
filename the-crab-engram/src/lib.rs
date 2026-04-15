pub mod storage;
pub mod models;
pub mod fsrs;

pub use storage::{Storage, FileStorage};
pub use models::{SessionRecord, MasteryLevel, SessionMetrics};

use chrono::Utc;
use storage::StorageResult;

/// Save a study session record using the provided storage backend.
pub async fn mem_save(
    storage: &dyn Storage,
    subject: &str,
    duration_seconds: u32,
    breaks: u32,
) -> StorageResult<()> {
    let session = SessionRecord {
        timestamp: Utc::now(),
        subject: subject.to_string(),
        duration_seconds,
        breaks,
    };
    storage.save_session(&session).await
}

/// Retrieve the mastery level for a given subject.
pub async fn get_mastery_level(
    storage: &dyn Storage,
    subject: &str,
) -> StorageResult<Option<MasteryLevel>> {
    storage.get_mastery(subject).await
}

/// Get all mastery levels that are due for review based on FSRS intervals.
///
/// Filters mastery records whose next scheduled review date has passed.
pub async fn mem_reviews(
    storage: &dyn Storage,
) -> StorageResult<Vec<MasteryLevel>> {
    let all_mastery = storage.get_all_mastery().await?;
    let now = chrono::Utc::now();
    Ok(all_mastery
        .into_iter()
        .filter(|m| {
            let (_, interval) = fsrs::calculate_next_review(
                m.last_updated,
                m.review_count.max(1),
                3,
            );
            m.last_updated + chrono::Duration::days(interval as i64) <= now
        })
        .collect())
}

/// Ensure the data directory exists, creating it if necessary.
pub fn ensure_data_dir(path: &std::path::Path) -> std::io::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}
