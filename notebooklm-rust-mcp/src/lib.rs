pub mod pdf;
pub mod client;

pub use pdf::{PDFProcessor, PDFTextExtractor};
pub use client::{NotebookLMClient, QuizRequest, QuizResponse};
