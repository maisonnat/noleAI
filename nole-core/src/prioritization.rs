// Prioritization module for intelligent task scheduling
use chrono::{DateTime, Timelike, Utc, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyProfile {
    pub time_slot: String, // e.g., "09:00-12:00", "14:00-17:00"
    pub energy_level: u8,  // 1-5 scale
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskPriority {
    pub subject: String,
    pub deadline: Option<DateTime<Utc>>,
    pub mastery_level: u8,
    pub last_reviewed: Option<DateTime<Utc>>,
    pub estimated_minutes: u32,
}

pub fn calculate_current_energy() -> u8 {
    let now = Utc::now();
    let hour = now.hour();

    // Simple energy profile based on time of day
    match hour {
        9..=11 => 4,  // Morning - high energy
        14..=16 => 3, // Afternoon - medium energy
        19..=21 => 2, // Evening - low energy
        _ => 1,       // Other times - very low energy
    }
}

pub fn prioritize_tasks(tasks: Vec<TaskPriority>) -> Vec<TaskPriority> {
    let mut prioritized = tasks;

    prioritized.sort_by(|a, b| {
        // Priority 1: Deadline urgency
        match (&a.deadline, &b.deadline) {
            (Some(da), Some(db)) => da.cmp(db),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    });

    prioritized
}

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
