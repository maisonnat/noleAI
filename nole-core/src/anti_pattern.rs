use crate::events::{EventPublisher, Event, EventResult};
use crate::session_tracker::SessionTracker;
use std::sync::{Arc, Mutex};

const LONG_SESSION_THRESHOLD_SECS: u32 = 2700; // 45 minutes

pub trait AntiPatternDetector: Send + Sync {
    fn check_long_session(&self, duration_without_breaks: u32) -> Option<String>;
}

pub struct DefaultAntiPatternDetector;

impl AntiPatternDetector for DefaultAntiPatternDetector {
    fn check_long_session(&self, duration_without_breaks: u32) -> Option<String> {
        if duration_without_breaks > LONG_SESSION_THRESHOLD_SECS {
            Some(format!(
                "You've been studying for {} minutes without a break. Consider taking a break.",
                duration_without_breaks / 60
            ))
        } else {
            None
        }
    }
}

pub struct MonitoredSessionTracker {
    inner: Arc<Mutex<SessionTracker>>,
    detector: Arc<dyn AntiPatternDetector>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl MonitoredSessionTracker {
    pub fn new(
        tracker: Arc<Mutex<SessionTracker>>,
        detector: Arc<dyn AntiPatternDetector>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            inner: tracker,
            detector,
            event_publisher,
        }
    }

    pub fn start_session(&self, subject: String) -> String {
        self.inner.lock().unwrap().start_session(subject)
    }

    pub fn end_session(&self) -> Option<crate::session_tracker::SessionMetrics> {
        self.inner.lock().unwrap().end_session()
    }

    pub fn record_break(&self) {
        self.inner.lock().unwrap().record_break();
    }

    pub async fn increment_duration(&self, seconds: u32) -> EventResult<()> {
        let duration_without_breaks = {
            let mut tracker = self.inner.lock().unwrap();
            tracker.increment_duration(seconds);
            tracker.get_current_duration_without_breaks()
        };

        if let Some(message) = self.detector.check_long_session(duration_without_breaks) {
            self.event_publisher.publish(Event::AntiPatternDetected {
                pattern_type: "long_session".to_string(),
                message,
            }).await?;
        }

        Ok(())
    }
}
