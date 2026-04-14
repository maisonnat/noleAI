use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetrics {
    pub start_time: chrono::DateTime<chrono::Local>,
    pub duration_mins: u32,
    pub breaks_taken: u32,
    pub subject: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AntiPattern {
    LongSessionWithoutBreak { duration_mins: u32 },
    ExcessiveContextSwitching { switches: u32 },
    InconsistentSchedule { variance_mins: u32 },
    LowEngagement { session_mins: u32 },
}

pub struct AntiPatternDetector {
    thresholds: HashMap<String, f32>,
}

impl AntiPatternDetector {
    pub fn new() -> Self {
        let mut thresholds = HashMap::new();

        // Configuración de umbrales para detección de anti-patrones
        thresholds.insert("max_session_mins".to_string(), 45.0); // Máximo 45 minutos sin break
        thresholds.insert("max_context_switches".to_string(), 5.0); // Máximo 5 cambios de materia por día
        thresholds.insert("min_session_mins".to_string(), 15.0); // Mínimo 15 minutos por sesión
        thresholds.insert("schedule_variance_mins".to_string(), 30.0); // Variación máxima de 30 min

        Self { thresholds }
    }

    pub fn detect_anti_patterns(&self, sessions: &[SessionMetrics]) -> Vec<AntiPattern> {
        let mut patterns = Vec::new();

        for session in sessions {
            // Detectar sesiones muy largas sin breaks
            if session.duration_mins > self.thresholds["max_session_mins"] as u32 {
                patterns.push(AntiPattern::LongSessionWithoutBreak {
                    duration_mins: session.duration_mins,
                });
            }

            // Detectar sesiones muy cortas (baja concentración)
            if session.duration_mins < self.thresholds["min_session_mins"] as u32 {
                patterns.push(AntiPattern::LowEngagement {
                    session_mins: session.duration_mins,
                });
            }
        }

        // Detectar cambios excesivos de contexto
        let context_switches = self.count_context_switches(sessions);
        if context_switches > self.thresholds["max_context_switches"] as u32 {
            patterns.push(AntiPattern::ExcessiveContextSwitching {
                switches: context_switches,
            });
        }

        patterns
    }

    fn count_context_switches(&self, sessions: &[SessionMetrics]) -> u32 {
        if sessions.len() < 2 {
            return 0;
        }

        let mut switches = 0;
        let mut last_subject = &sessions[0].subject;

        for session in sessions.iter().skip(1) {
            if session.subject != *last_subject {
                switches += 1;
                last_subject = &session.subject;
            }
        }

        switches
    }

    pub fn generate_alert(&self, pattern: &AntiPattern) -> String {
        match pattern {
            AntiPattern::LongSessionWithoutBreak { duration_mins } => {
                format!(
                    "⚠️ **Sesión larga detectada**: Has estado estudiando por {} minutos sin break. \
                    Sugiero tomar un descanso de 5 minutos para mantener el enfoque.",
                    duration_mins
                )
            }
            AntiPattern::ExcessiveContextSwitching { switches } => {
                format!(
                    "⚠️ **Cambios de contexto excesivos**: Has cambiado de materia {} veces hoy. \
                    Intenta enfocarte en una materia por sesión más larga.",
                    switches
                )
            }
            AntiPattern::InconsistentSchedule { variance_mins } => {
                format!(
                    "⚠️ **Horario inconsistente**: La variación en tus horarios de estudio es de {} minutos. \
                    Consistencia ayuda con hábitos TDAH.",
                    variance_mins
                )
            }
            AntiPattern::LowEngagement { session_mins } => {
                format!(
                    "⚠️ **Baja engagement**: Sesión muy corta de {} minutos. \
                    ¿Hay algo que te esté distrayendo? Considera ajustar el ambiente.",
                    session_mins
                )
            }
        }
    }

    pub fn get_recommendations(&self, pattern: &AntiPattern) -> Vec<String> {
        match pattern {
            AntiPattern::LongSessionWithoutBreak { .. } => vec![
                "Usa la técnica Pomodoro: 25 min trabajo, 5 min descanso".to_string(),
                "Levántate y estírate durante el break".to_string(),
                "Bebe agua o hidrátate".to_string(),
            ],
            AntiPattern::ExcessiveContextSwitching { .. } => vec![
                "Prioriza materias y enfócate en una por sesión".to_string(),
                "Usa la técnica de time-blocking en tu calendario".to_string(),
                "Agrupa tareas similares".to_string(),
            ],
            AntiPattern::InconsistentSchedule { .. } => vec![
                "Establece horarios fijos de estudio".to_string(),
                "Usa recordatorios para mantener consistencia".to_string(),
                "Considera tu nivel de energía al programar".to_string(),
            ],
            AntiPattern::LowEngagement { .. } => vec![
                "Elimina distracciones (notificaciones, redes sociales)".to_string(),
                "Asegúrate de dormir bien y comer adecuadamente".to_string(),
                "Intenta cambiar de materia si el actual no te motiva".to_string(),
            ],
        }
    }
}

impl Default for AntiPatternDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;

    #[test]
    fn test_long_session_detection() {
        let detector = AntiPatternDetector::new();
        let sessions = vec![SessionMetrics {
            start_time: Local::now(),
            duration_mins: 60, // Over the 45 min threshold
            breaks_taken: 0,
            subject: "Math".to_string(),
        }];

        let patterns = detector.detect_anti_patterns(&sessions);
        assert_eq!(patterns.len(), 1);
        assert!(matches!(
            patterns[0],
            AntiPattern::LongSessionWithoutBreak { .. }
        ));
    }

    #[test]
    fn test_low_engagement_detection() {
        let detector = AntiPatternDetector::new();
        let sessions = vec![SessionMetrics {
            start_time: Local::now(),
            duration_mins: 10, // Under the 15 min threshold
            breaks_taken: 0,
            subject: "Math".to_string(),
        }];

        let patterns = detector.detect_anti_patterns(&sessions);
        assert_eq!(patterns.len(), 1);
        assert!(matches!(patterns[0], AntiPattern::LowEngagement { .. }));
    }

    #[test]
    fn test_context_switching() {
        let detector = AntiPatternDetector::new();
        let sessions = vec![
            SessionMetrics {
                start_time: Local::now(),
                duration_mins: 20,
                breaks_taken: 0,
                subject: "Math".to_string(),
            },
            SessionMetrics {
                start_time: Local::now(),
                duration_mins: 20,
                breaks_taken: 0,
                subject: "Physics".to_string(),
            },
            SessionMetrics {
                start_time: Local::now(),
                duration_mins: 20,
                breaks_taken: 0,
                subject: "Chemistry".to_string(),
            },
            SessionMetrics {
                start_time: Local::now(),
                duration_mins: 20,
                breaks_taken: 0,
                subject: "Biology".to_string(),
            },
            SessionMetrics {
                start_time: Local::now(),
                duration_mins: 20,
                breaks_taken: 0,
                subject: "History".to_string(),
            },
            SessionMetrics {
                start_time: Local::now(),
                duration_mins: 20,
                breaks_taken: 0,
                subject: "Geography".to_string(),
            },
            SessionMetrics {
                start_time: Local::now(),
                duration_mins: 20,
                breaks_taken: 0,
                subject: "Art".to_string(), // Extra subject to exceed the threshold
            },
        ];

        let patterns = detector.detect_anti_patterns(&sessions);
        // Check if there's at least one excessive context switching pattern
        let has_excessive_switching = patterns
            .iter()
            .any(|p| matches!(p, AntiPattern::ExcessiveContextSwitching { .. }));
        assert!(
            has_excessive_switching,
            "Expected to find excessive context switching pattern, but found: {:?}",
            patterns
        );
    }
}
