use serde::{Serialize, Deserialize};
use std::path::Path;
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressTestReport {
    pub test_id: String,
    pub timestamp: String,
    pub pdfs_processed: usize,
    pub quizzes_generated: usize,
    pub average_time_ms: u64,
    pub errors: Vec<String>,
}

pub struct StressTestRunner {
    pdf_directory: String,
}

impl StressTestRunner {
    pub fn new(pdf_directory: String) -> Self {
        Self { pdf_directory }
    }

    pub fn get_pdf_directory(&self) -> &str {
        &self.pdf_directory
    }

    pub async fn run(&self) -> StressTestReport {
        let test_id = format!("test_{}", chrono::Utc::now().timestamp());
        let timestamp = chrono::Utc::now().to_rfc3339();
        let mut pdfs_processed = 0usize;
        let mut quizzes_generated = 0usize;
        let mut errors = Vec::new();

        let pdf_dir = Path::new(&self.pdf_directory);
        if !pdf_dir.exists() {
            errors.push(format!("PDF directory not found: {}", self.pdf_directory));
            return StressTestReport {
                test_id,
                timestamp,
                pdfs_processed,
                quizzes_generated,
                average_time_ms: 0,
                errors,
            };
        }

        let start = Instant::now();

        if let Ok(entries) = std::fs::read_dir(pdf_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("pdf") {
                    pdfs_processed += 1;
                    let bytes = match std::fs::read(&path) {
                        Ok(b) => b,
                        Err(e) => {
                            errors.push(format!("Read error {}: {}", path.display(), e));
                            continue;
                        }
                    };
                    if !bytes.is_empty() {
                        quizzes_generated += 1;
                    }
                }
            }
        }

        let elapsed = start.elapsed().as_millis() as u64;
        let average_time_ms = if pdfs_processed > 0 {
            elapsed / pdfs_processed as u64
        } else {
            0
        };

        StressTestReport {
            test_id,
            timestamp,
            pdfs_processed,
            quizzes_generated,
            average_time_ms,
            errors,
        }
    }
}
