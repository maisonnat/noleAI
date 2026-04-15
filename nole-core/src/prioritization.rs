use chrono::{DateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};

/// A time-of-day energy profile used for ADHD-aware task scheduling.
///
/// Example: `{ time_slot: "09:00-12:00", energy_level: 4 }`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyProfile {
    /// Time range in "HH:MM-HH:MM" format.
    pub time_slot: String,
    /// Energy level on a 1-5 scale (5 = highest).
    pub energy_level: u8,
}

/// A task with metadata for priority scoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskPriority {
    /// Subject or task name.
    pub subject: String,
    /// Optional deadline (used for urgency weighting).
    pub deadline: Option<DateTime<Utc>>,
    /// Current mastery level (1-5, lower = needs more attention).
    pub mastery_level: u8,
    /// When this subject was last reviewed.
    pub last_reviewed: Option<DateTime<Utc>>,
    /// Estimated task duration in minutes.
    pub estimated_minutes: u32,
}

/// Calculate current energy level based on time of day.
///
/// Returns a default profile: morning (4), afternoon (3), evening (2), other (1).
pub fn calculate_current_energy() -> u8 {
    let now = Utc::now();
    let hour = now.hour();

    match hour {
        9..=11 => 4,
        14..=16 => 3,
        19..=21 => 2,
        _ => 1,
    }
}

/// Calculate energy using user-configured profiles, falling back to defaults.
///
/// If no profile matches the current time, delegates to `calculate_current_energy`.
pub fn calculate_current_energy_with_profile(profiles: &[EnergyProfile]) -> u8 {
    if profiles.is_empty() {
        return calculate_current_energy();
    }

    let now = Utc::now();
    let current_minutes = now.hour() as u32 * 60 + now.minute() as u32;

    for profile in profiles {
        if let Some((start, end)) = parse_time_slot(&profile.time_slot) {
            if current_minutes >= start && current_minutes < end {
                return profile.energy_level;
            }
        }
    }

    calculate_current_energy()
}

fn parse_time_slot(slot: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = slot.split('-').collect();
    if parts.len() != 2 {
        return None;
    }
    let start = parse_hhmm(parts[0].trim())?;
    let end = parse_hhmm(parts[1].trim())?;
    Some((start, end))
}

fn parse_hhmm(s: &str) -> Option<u32> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return None;
    }
    let hours: u32 = parts[0].parse().ok()?;
    let minutes: u32 = parts[1].parse().ok()?;
    Some(hours * 60 + minutes)
}

/// Compute a priority score using FSRS-based mastery and review freshness.
///
/// Higher scores mean higher priority. Low mastery and long review gaps increase score.
pub fn fsrs_priority_score(task: &TaskPriority) -> f64 {
    let mastery_score = (6 - task.mastery_level) as f64 * 2.0;

    let review_freshness = match task.last_reviewed {
        Some(last) => {
            let days_since = (Utc::now() - last).num_days().max(0) as f64;
            (days_since / 7.0).min(3.0)
        }
        None => 3.0,
    };

    mastery_score + review_freshness
}

/// Compute a deadline urgency weight (0.0 to 100.0).
///
/// Overdue tasks get the highest weight; tasks due in over a week get minimal weight.
pub fn deadline_weight(task: &TaskPriority) -> f64 {
    match task.deadline {
        Some(deadline) => {
            let days_until = (deadline - Utc::now()).num_days();
            if days_until <= 0 {
                100.0
            } else if days_until <= 1 {
                50.0
            } else if days_until <= 3 {
                30.0
            } else if days_until <= 7 {
                15.0
            } else {
                5.0
            }
        }
        None => 0.0,
    }
}

/// Sort tasks by combined FSRS score + deadline weight (highest priority first).
pub fn prioritize_tasks(tasks: Vec<TaskPriority>) -> Vec<TaskPriority> {
    let mut prioritized = tasks;

    prioritized.sort_by(|a, b| {
        let score_a = fsrs_priority_score(a) + deadline_weight(a);
        let score_b = fsrs_priority_score(b) + deadline_weight(b);
        score_b
            .partial_cmp(&score_a)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    prioritized
}

/// Get the recommended maximum task count based on current energy level.
pub fn get_energy_based_task_count(energy_level: u8) -> usize {
    match energy_level {
        5 => 6,
        4 => 5,
        3 => 4,
        2 => 2,
        1 => 1,
        _ => 3,
    }
}
