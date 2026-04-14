use chrono::{DateTime, Utc};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Vault path not found: {0}")]
    VaultNotFound(PathBuf),
}

pub type VaultResult<T> = Result<T, VaultError>;

#[derive(Debug, Clone)]
pub struct Subject {
    pub name: String,
    pub mastery_level: u8, // 1-5 scale
    pub topics: Vec<String>,
    pub last_studied: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct StudyTask {
    pub subject: String,
    pub topic: String,
    pub estimated_minutes: u32,
    pub priority: u8, // 1-5 scale
}

#[derive(Debug, Clone)]
pub struct DailyPlan {
    pub date: String,
    pub tasks: Vec<StudyTask>,
    pub total_estimated_minutes: u32,
}

pub struct VaultParser {
    vault_path: PathBuf,
}

impl VaultParser {
    pub fn new<P: AsRef<Path>>(vault_path: P) -> VaultResult<Self> {
        let path = vault_path.as_ref();
        if !path.exists() {
            return Err(VaultError::VaultNotFound(path.to_path_buf()));
        }
        Ok(Self {
            vault_path: path.to_path_buf(),
        })
    }

    pub fn parse_config(&self) -> VaultResult<Vec<Subject>> {
        let config_path = self.vault_path.join("Config").join("Materias.md");
        let content = fs::read_to_string(&config_path)?;
        self.parse_subjects(&content)
    }

    fn parse_subjects(&self, content: &str) -> VaultResult<Vec<Subject>> {
        let mut subjects = Vec::new();
        let subject_re = Regex::new(r"##\s+(.+)\s+\|\s+Nivel:\s*(\d+)").unwrap();
        let topic_re = Regex::new(r"-\s+(.+)").unwrap();

        let lines: Vec<&str> = content.lines().collect();
        let mut current_subject: Option<Subject> = None;

        for line in lines {
            if let Some(caps) = subject_re.captures(line) {
                if let Some(subject) = current_subject.take() {
                    subjects.push(subject);
                }

                let name = caps.get(1).unwrap().as_str().to_string();
                let mastery_level = caps.get(2).unwrap().as_str().parse().unwrap_or(3);

                current_subject = Some(Subject {
                    name,
                    mastery_level,
                    topics: Vec::new(),
                    last_studied: None,
                });
            } else if let Some(caps) = topic_re.captures(line) {
                if let Some(ref mut subject) = current_subject {
                    subject
                        .topics
                        .push(caps.get(1).unwrap().as_str().to_string());
                }
            }
        }

        if let Some(subject) = current_subject {
            subjects.push(subject);
        }

        Ok(subjects)
    }

    pub fn generate_daily_plan(&self, subjects: &[Subject]) -> VaultResult<DailyPlan> {
        let date = Utc::now().format("%Y-%m-%d").to_string();
        let mut tasks = Vec::new();
        let mut total_minutes = 0u32;

        // Simple implementation: take one topic from each subject
        for subject in subjects {
            if let Some(topic) = subject.topics.first() {
                tasks.push(StudyTask {
                    subject: subject.name.clone(),
                    topic: topic.clone(),
                    estimated_minutes: 25, // Default Pomodoro length
                    priority: self.calculate_priority(subject),
                });
                total_minutes += 25;
            }
        }

        Ok(DailyPlan {
            date,
            tasks,
            total_estimated_minutes: total_minutes,
        })
    }

    fn calculate_priority(&self, subject: &Subject) -> u8 {
        // Lower mastery = higher priority
        match subject.mastery_level {
            1 => 5,
            2 => 4,
            3 => 3,
            4 => 2,
            5 => 1,
            _ => 3,
        }
    }

    pub fn write_hoy(&self, plan: &DailyPlan) -> VaultResult<()> {
        let hoy_path = self
            .vault_path
            .join("HOY")
            .join(format!("{}.md", plan.date));

        let mut content = format!("# Plan de Estudio - {}\n\n", plan.date);
        content.push_str(&format!(
            "**Total estimado:** {} minutos\n\n",
            plan.total_estimated_minutes
        ));
        content.push_str("## Tareas\n\n");

        for (i, task) in plan.tasks.iter().enumerate() {
            content.push_str(&format!(
                "{}. [ ] {} - {} ({} min)\n",
                i + 1,
                task.subject,
                task.topic,
                task.estimated_minutes
            ));
        }

        fs::write(&hoy_path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_calculation() {
        let parser = VaultParser::new("/tmp").unwrap();

        let low_mastery = Subject {
            name: "Math".to_string(),
            mastery_level: 1,
            topics: vec!["Algebra".to_string()],
            last_studied: None,
        };

        let high_mastery = Subject {
            name: "Physics".to_string(),
            mastery_level: 5,
            topics: vec!["Quantum".to_string()],
            last_studied: None,
        };

        assert_eq!(parser.calculate_priority(&low_mastery), 5);
        assert_eq!(parser.calculate_priority(&high_mastery), 1);
    }
}
