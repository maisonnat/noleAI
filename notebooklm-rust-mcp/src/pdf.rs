use async_trait::async_trait;
use pdf_extract::extract_text_from_mem;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PDFError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("PDF extraction error: {0}")]
    ExtractionError(String),
    #[error("File not found: {0}")]
    FileNotFound(String),
}

pub type PDFResult<T> = Result<T, PDFError>;

#[async_trait]
pub trait PDFProcessor: Send + Sync {
    async fn extract_text(&self, pdf_path: &Path) -> PDFResult<String>;
}

pub struct PDFTextExtractor;

impl PDFTextExtractor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PDFProcessor for PDFTextExtractor {
    async fn extract_text(&self, pdf_path: &Path) -> PDFResult<String> {
        if !pdf_path.exists() {
            return Err(PDFError::FileNotFound(pdf_path.to_string_lossy().to_string()));
        }
        
        let pdf_bytes = std::fs::read(pdf_path)?;
        
        match extract_text_from_mem(&pdf_bytes) {
            Ok(text) => Ok(text),
            Err(e) => Err(PDFError::ExtractionError(e.to_string())),
        }
    }
}

impl Default for PDFTextExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extractor_creation() {
        let extractor = PDFTextExtractor::new();
        assert!(true);  // Just verifies it doesn't panic
    }
}
