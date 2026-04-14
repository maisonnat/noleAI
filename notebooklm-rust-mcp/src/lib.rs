/// Bridge para ingesta de material - NotebookLM MCP
pub struct McpBridge;

impl McpBridge {
    pub fn new() -> Self {
        Self
    }

    /// Ingestar un PDF y generar un quiz de prueba
    pub fn generate_quiz_from_pdf(&self, pdf_path: &str) -> Result<Vec<String>, String> {
        println!("📄 [MCP] Ingestando PDF desde: {}", pdf_path);
        Ok(vec![
            "Pregunta de ejemplo 1 sobre el PDF".to_string(),
            "Pregunta de ejemplo 2 sobre el PDF".to_string()
        ])
    }
}
