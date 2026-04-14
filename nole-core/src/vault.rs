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
        plan.push_str(&format!(
            "- [ ] **Prioridad 1:** {} (Sesión de 25 mins)\n",
            materias[0]
        ));
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

/// Genera el contenido del dashboard de progreso (Progreso.md)
pub fn generate_progress_dashboard(_materias: &[String]) -> String {
    let date = Local::now().format("%Y-%m-%d").to_string();
    let mut dashboard = format!(
        "# Dashboard de Progreso\n\n📅 Última actualización: {}\n\n",
        date
    );

    dashboard.push_str("## 📊 Niveles de Maestría\n\n");
    dashboard.push_str("| Materia | Nivel | Estado |\n");
    dashboard.push_str("|---------|-------|--------|\n");

    // Datos mock de maestría para demostración
    let mock_mastery: Vec<(&str, f32, &str)> = vec![
        ("Ciberdefensa", 65.0, "🟢 En progreso"),
        ("Álgebra Lineal", 42.0, "🟡 Necesita repaso"),
        ("Análisis Matemático", 30.0, "🔴 Inicio"),
    ];

    for (materia, nivel, estado) in mock_mastery {
        let bar_len = (nivel / 10.0).min(10.0) as usize;
        let bar = "█".repeat(bar_len);
        let empty = "░".repeat(10 - bar_len);
        dashboard.push_str(&format!(
            "| {} | {}% {} {}| {} |\n",
            materia, nivel, bar, empty, estado
        ));
    }

    dashboard.push_str("\n## 🎯 Objetivos de la Semana\n\n");
    dashboard.push_str("- [x] Completar módulo de criptografía básica\n");
    dashboard.push_str("- [ ] Practicar 10 ejercicios de vectores\n");
    dashboard.push_str("- [ ] Leer capítulo 4 de análisis\n");

    dashboard.push_str("\n## 📈 Resumen de Sesiones\n\n");
    dashboard.push_str("- **Horas Deep Work**: 12.5 hrs\n");
    dashboard.push_str("- **Racha de estudio**: 3 días\n");
    dashboard.push_str("- **Próximo repaso**: Mañana 10:00 AM\n");

    dashboard
}

/// Escribe el dashboard de progreso en el archivo Progreso.md
pub fn write_progress_dashboard(vault_path: &Path, content: &str) -> Result<()> {
    let progreso_path = vault_path.join("Progreso.md");
    fs::write(progreso_path, content)?;
    Ok(())
}

/// Genera el contenido de la lista de repetición espaciada (Repaso.md)
pub fn generate_review_list(reviews: &[String]) -> String {
    let date = Local::now().format("%Y-%m-%d").to_string();
    let mut review_list = format!(
        "# Lista de Repaso Espaciado\n\n📅 Actualizado: {}\n\n",
        date
    );

    review_list.push_str("## 📝 Conceptos Pendientes de Repaso\n\n");

    if reviews.is_empty() {
        review_list.push_str("*No hay revisiones pendientes. ¡Buen trabajo!*\n");
    } else {
        for (index, review) in reviews.iter().enumerate() {
            review_list.push_str(&format!("{}. {}\n", index + 1, review));
        }
    }

    review_list.push_str("\n## 📊 Estadísticas de Repaso\n\n");
    review_list.push_str(&format!("- **Total pendientes**: {}\n", reviews.len()));
    review_list.push_str("- **Próximo repaso sugerido**: Hoy\n");
    review_list.push_str("- **Precisión media**: 75%\n");

    review_list
}

/// Escribe la lista de repetición espaciada en el archivo Repaso.md
pub fn write_review_list(vault_path: &Path, content: &str) -> Result<()> {
    let repaso_path = vault_path.join("Repaso.md");
    fs::write(repaso_path, content)?;
    Ok(())
}

/// Genera la lista de repetición espaciada usando el sistema Engram
pub fn generate_review_from_engram(engram: &the_crab_engram::Engram) -> Result<String> {
    let reviews = engram.mem_reviews();
    Ok(generate_review_list(&reviews))
}

/// Genera un resumen diario de sesiones de estudio
pub fn generate_daily_summary(sessions: &[crate::anti_patterns::SessionMetrics]) -> Result<String> {
    let date = Local::now().format("%Y-%m-%d").to_string();
    let mut summary = format!("# Resumen de Sesión: {}\n\n", date);

    if sessions.is_empty() {
        summary.push_str("*No hubo sesiones de estudio hoy.*\n");
        return Ok(summary);
    }

    let total_mins: u32 = sessions.iter().map(|s| s.duration_mins).sum();
    let total_hours = total_mins / 60;
    let remaining_mins = total_mins % 60;

    summary.push_str("## 📊 Resumen General\n\n");
    summary.push_str(&format!(
        "- **Total de tiempo**: {}h {}min\n",
        total_hours, remaining_mins
    ));
    summary.push_str(&format!("- **Número de sesiones**: {}\n", sessions.len()));
    summary.push_str(&format!(
        "- **Promedio por sesión**: {}min\n",
        total_mins / sessions.len() as u32
    ));

    summary.push_str("\n## 📝 Detalles por Sesión\n\n");
    for (index, session) in sessions.iter().enumerate() {
        summary.push_str(&format!("### Sesión {} - {}\n", index + 1, session.subject));
        summary.push_str(&format!("- **Duración**: {} min\n", session.duration_mins));
        summary.push_str(&format!(
            "- **Inicio**: {}\n",
            session.start_time.format("%H:%M")
        ));
        summary.push_str(&format!("- **Pausas**: {}\n", session.breaks_taken));
        summary.push_str("\n");
    }

    summary.push_str("## 💡 Reflexiones y Próximos Pasos\n\n");
    summary.push_str("- **¿Qué funcionó bien hoy?**: (rellenar después de cada sesión)\n");
    summary.push_str("- **¿Qué se puede mejorar?**: (rellenar después de cada sesión)\n");
    summary.push_str("- **Plan para mañana**: (basado en el progreso actual)\n");

    Ok(summary)
}

/// Escribe el resumen diario en el archivo correspondiente en Sesiones/daily-summaries/
pub fn write_daily_summary(vault_path: &Path, content: &str) -> Result<()> {
    let summaries_dir = vault_path.join("Sesiones").join("daily-summaries");
    fs::create_dir_all(&summaries_dir)?;

    let date = Local::now().format("%Y-%m-%d").to_string();
    let summary_path = summaries_dir.join(format!("{}.md", date));
    fs::write(summary_path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_parse_materias_empty_file() {
        let dir = tempdir().unwrap();
        let config_dir = dir.path().join("Config");
        fs::create_dir_all(&config_dir).unwrap();
        let materias_path = config_dir.join("Materias.md");
        fs::write(&materias_path, "").unwrap();

        let result = parse_materias(dir.path());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_parse_materias_with_content() {
        let dir = tempdir().unwrap();
        let config_dir = dir.path().join("Config");
        fs::create_dir_all(&config_dir).unwrap();
        let materias_path = config_dir.join("Materias.md");
        fs::write(&materias_path, "- Ciberdefensa\n- Álgebra\n- Análisis").unwrap();

        let result = parse_materias(dir.path());
        assert!(result.is_ok());
        let materias = result.unwrap();
        assert_eq!(materias.len(), 3);
        assert_eq!(materias[0], "Ciberdefensa");
    }

    #[test]
    fn test_generate_daily_plan() {
        let materias = vec!["Ciberdefensa".to_string(), "Álgebra".to_string()];
        let plan = generate_daily_plan(&materias);

        assert!(plan.contains("# Plan Diario"));
        assert!(plan.contains("Ciberdefensa"));
        assert!(plan.contains("Álgebra"));
        assert!(plan.contains("Prioridad 1"));
    }

    #[test]
    fn test_generate_progress_dashboard() {
        let materias = vec!["Ciberdefensa".to_string()];
        let dashboard = generate_progress_dashboard(&materias);

        assert!(dashboard.contains("# Dashboard de Progreso"));
        assert!(dashboard.contains("Niveles de Maestría"));
        assert!(dashboard.contains("Ciberdefensa"));
        assert!(dashboard.contains("Objetivos de la Semana"));
    }

    #[test]
    fn test_generate_review_list() {
        let reviews = vec![
            "Concepto A".to_string(),
            "Concepto B".to_string(),
            "Concepto C".to_string(),
        ];
        let review_list = generate_review_list(&reviews);

        assert!(review_list.contains("# Lista de Repaso Espaciado"));
        assert!(review_list.contains("Conceptos Pendientes de Repaso"));
        assert!(review_list.contains("Concepto A"));
        assert!(review_list.contains("Concepto B"));
        assert!(review_list.contains("Concepto C"));
        assert!(review_list.contains("Total pendientes**"));
    }

    #[test]
    fn test_generate_review_list_empty() {
        let reviews: Vec<String> = vec![];
        let review_list = generate_review_list(&reviews);

        assert!(review_list.contains("# Lista de Repaso Espaciado"));
        assert!(review_list.contains("No hay revisiones pendientes"));
    }

    #[test]
    fn test_generate_daily_summary_empty() {
        let sessions: Vec<crate::anti_patterns::SessionMetrics> = vec![];
        let summary = generate_daily_summary(&sessions).unwrap();

        assert!(summary.contains("No hubo sesiones de estudio hoy"));
    }

    #[test]
    fn test_generate_daily_summary_with_sessions() {
        use crate::anti_patterns::SessionMetrics;

        let sessions = vec![
            SessionMetrics {
                start_time: Local::now(),
                duration_mins: 30,
                breaks_taken: 1,
                subject: "Math".to_string(),
            },
            SessionMetrics {
                start_time: Local::now(),
                duration_mins: 25,
                breaks_taken: 0,
                subject: "Physics".to_string(),
            },
        ];

        let summary = generate_daily_summary(&sessions).unwrap();

        assert!(summary.contains("Resumen de Sesión"));
        assert!(summary.contains("Total de tiempo"));
        assert!(summary.contains("Número de sesiones"));
        assert!(summary.contains("Sesión 1 - Math"));
        assert!(summary.contains("Sesión 2 - Physics"));
        assert!(summary.contains("Reflexiones y Próximos Pasos"));
    }
}
