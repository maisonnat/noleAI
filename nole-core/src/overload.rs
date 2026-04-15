// Overload mode - emergency plan generator
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyPlan {
    pub tasks: Vec<String>,
    pub simplified: bool,
    pub energy_level: u8,
}

pub struct OverloadPlanGenerator {
    current_energy: u8,
}

impl OverloadPlanGenerator {
    pub fn new(current_energy: u8) -> Self {
        Self { current_energy }
    }

    pub fn generate_emergency_plan(&self, all_tasks: Vec<String>) -> EmergencyPlan {
        // In overload mode, return only 1-2 low-complexity tasks
        let simplified_tasks = if self.current_energy <= 2 {
            // Very low energy - single simple task
            vec![all_tasks
                .first()
                .cloned()
                .unwrap_or_else(|| "Take a 10-minute break".to_string())]
        } else {
            // Low to medium energy - 2 simple tasks
            all_tasks.into_iter().take(2).collect()
        };

        EmergencyPlan {
            tasks: simplified_tasks,
            simplified: true,
            energy_level: self.current_energy,
        }
    }

    pub fn generate_recovery_plan(&self) -> EmergencyPlan {
        EmergencyPlan {
            tasks: vec![
                "Take a 15-minute walk".to_string(),
                "Do 5 minutes of breathing exercises".to_string(),
                "Review one simple concept".to_string(),
            ],
            simplified: true,
            energy_level: self.current_energy,
        }
    }
}
