// Session tracking for anti-pattern detection
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetrics {
    pub session_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub subject: String,
    pub breaks_taken: u32,
    pub duration_without_breaks: u32,
}

pub struct SessionTracker {
    current_session: Option<SessionMetrics>,
    sessions: Vec<SessionMetrics>,
}

impl SessionTracker {
    pub fn new() -> Self {
        Self {
            current_session: None,
            sessions: Vec::new(),
        }
    }

    pub fn start_session(&mut self, subject: String) -> String {
        let session_id = format!("session_{}", Utc::now().timestamp());
        let metrics = SessionMetrics {
            session_id: session_id.clone(),
            start_time: Utc::now(),
            end_time: None,
            subject,
            breaks_taken: 0,
            duration_without_breaks: 0,
        };
        self.current_session = Some(metrics);
        session_id
    }

    pub fn end_session(&mut self) -> Option<SessionMetrics> {
        if let Some(mut metrics) = self.current_session.take() {
            metrics.end_time = Some(Utc::now());
            let session = metrics.clone();
            self.sessions.push(metrics);
            Some(session)
        } else {
            None
        }
    }

    pub fn record_break(&mut self) {
        if let Some(ref mut session) = self.current_session {
            session.breaks_taken += 1;
            session.duration_without_breaks = 0;
        }
    }

    pub fn increment_duration(&mut self, seconds: u32) {
        if let Some(ref mut session) = self.current_session {
            session.duration_without_breaks += seconds;
        }
    }
}

impl Default for SessionTracker {
    fn default() -> Self {
        Self::new()
    }
}
