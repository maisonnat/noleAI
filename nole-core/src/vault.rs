use anyhow::Result;
use chrono::Local;
use std::fs;
use std::path::Path;

/// Lee el archivo de materias y extrae una lista simple.
/// Formato esperado en Materias.md: Lista con viñetas ("- Materia")
pub fn parse_materias(vault_path: &Path) -> Result<Vec<String>> {
    let materias_path = vault_path.join("Config").join("Materias.md");
    
    if !materias_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(materias_path)?;
    let materias = content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with("- ") {
                Some(trimmed.trim_start_matches("- ").to_string())
            } else {
                None
            }
        })
        .collect();

    Ok(materias)
}

/// Genera el contenido inicial para el plan diario (HOY.md)
pub fn generate_daily_plan(materias: &[String]) -> String {
    let date = Local::now().format("%Y-%m-%d").to_string();
    let mut plan = format!("# Plan Diario: {}\n\n", date);
    
    plan.push_str("## Enfoque de Hoy\n");
    plan.push_str("> *Generado por NoleAI Core - Prioridad: Fricción Cero*\n\n");

    if materias.is_empty() {
        plan.push_str("- [ ] Definir materias en `Config/Materias.md`\n");
    } else {
        // Lógica básica de priorización: mostrar la primera materia como prioridad
        plan.push_str(&format!("- [ ] **Prioridad 1:** {} (Sesión de 25 mins)\n", materias[0]));
        for materia in materias.iter().skip(1) {
            plan.push_str(&format!("- [ ] {} \n", materia));
        }
    }

    plan
}

/// Escribe el plan diario en el archivo HOY.md
pub fn write_daily_plan(vault_path: &Path, content: &str) -> Result<()> {
    let hoy_path = vault_path.join("HOY.md");
    fs::write(hoy_path, content)?;
    Ok(())
}
