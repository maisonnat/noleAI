pub mod pdf;
pub mod client;

pub use pdf::{PDFProcessor, PDFTextExtractor};
pub use client::{NotebookLMClient, QuizRequest, QuizResponse, ClientError};

use pdf::PDFResult;
use client::ClientResult;

pub struct PDFProcessingPipeline {
    pdf_extractor: PDFTextExtractor,
    client: NotebookLMClient,
}

impl PDFProcessingPipeline {
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        Self {
            pdf_extractor: PDFTextExtractor::new(),
            client: NotebookLMClient::new(api_key, base_url),
        }
    }

    pub async fn generate_quiz_from_pdf(
        &self,
        pdf_path: &std::path::Path,
        topic: &str,
        difficulty: &str,
        question_count: u32,
    ) -> Result<QuizResponse, ProcessingError> {
        let text = self.pdf_extractor.extract_text(pdf_path).await?;
        let request = QuizRequest {
            content: text,
            topic: topic.to_string(),
            difficulty: difficulty.to_string(),
            question_count,
        };
        let response = self.client.generate_quiz(&request).await?;
        Ok(response)
    }
}

#[derive(Debug)]
pub enum ProcessingError {
    PDFError(pdf::PDFError),
    ClientError(client::ClientError),
}

impl std::fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessingError::PDFError(e) => write!(f, "PDF error: {}", e),
            ProcessingError::ClientError(e) => write!(f, "Client error: {}", e),
        }
    }
}

impl std::error::Error for ProcessingError {}

impl From<pdf::PDFError> for ProcessingError {
    fn from(e: pdf::PDFError) -> Self {
        ProcessingError::PDFError(e)
    }
}

impl From<client::ClientError> for ProcessingError {
    fn from(e: client::ClientError) -> Self {
        ProcessingError::ClientError(e)
    }
}

pub fn report_progress(stage: &str, progress: f32) -> String {
    format!("[{:.0}%] {}", progress * 100.0, stage)
}
