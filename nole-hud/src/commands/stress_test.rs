use nole_core::stress_test::{StressTestRunner, StressTestReport};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct StressTestState {
    pub runner: Arc<Mutex<Option<StressTestRunner>>>,
}

#[derive(Serialize)]
pub struct StressTestResponse {
    pub test_id: String,
    pub timestamp: String,
    pub pdfs_processed: usize,
    pub quizzes_generated: usize,
    pub average_time_ms: u64,
    pub errors: Vec<String>,
}

impl From<StressTestReport> for StressTestResponse {
    fn from(r: StressTestReport) -> Self {
        StressTestResponse {
            test_id: r.test_id,
            timestamp: r.timestamp,
            pdfs_processed: r.pdfs_processed,
            quizzes_generated: r.quizzes_generated,
            average_time_ms: r.average_time_ms,
            errors: r.errors,
        }
    }
}

pub async fn run_stress_test(
    pdf_directory: String,
    state: StressTestState,
) -> Result<StressTestResponse, String> {
    let runner = StressTestRunner::new(pdf_directory);
    let report = runner.run().await;
    let response = StressTestResponse::from(report);
    let mut guard = state.runner.lock().await;
    *guard = Some(runner);
    Ok(response)
}
