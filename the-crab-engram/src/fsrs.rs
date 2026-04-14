// FSRS (Free Spaced Repetition Scheduler) algorithm
use chrono::{DateTime, Duration, Utc};

pub fn calculate_next_review(
    last_reviewed: DateTime<Utc>,
    current_interval_days: u32,
    performance_rating: u8, // 1-5 scale, where 5 is perfect recall
) -> (DateTime<Utc>, u32) {
    // Simplified FSRS implementation
    let new_interval = match performance_rating {
        5 => (current_interval_days as f64 * 2.5) as u32,
        4 => (current_interval_days as f64 * 1.5) as u32,
        3 => current_interval_days,
        2 => current_interval_days / 2,
        1 => 1,
        _ => current_interval_days,
    };

    let next_review = last_reviewed + Duration::days(new_interval as i64);
    (next_review, new_interval.max(1))
}

pub fn calculate_mastery_level(
    successful_reviews: u32,
    total_reviews: u32,
    current_interval_days: u32,
) -> u8 {
    if total_reviews == 0 {
        return 1;
    }

    let success_rate = successful_reviews as f64 / total_reviews as f64;

    match current_interval_days {
        0..=1 => 1,
        2..=7 => {
            if success_rate > 0.8 {
                2
            } else {
                1
            }
        }
        8..=21 => {
            if success_rate > 0.8 {
                3
            } else {
                2
            }
        }
        22..=60 => {
            if success_rate > 0.8 {
                4
            } else {
                3
            }
        }
        _ => {
            if success_rate > 0.8 {
                5
            } else {
                4
            }
        }
    }
}
