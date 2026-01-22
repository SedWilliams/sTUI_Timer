/*********************************
 * TimeLog Type
 *********************************/

use chrono::Local;
use rand::prelude::*;

//stored time log type
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TimeLog {
    pub id: u32,
    pub time_spent: [u64; 3],
    pub date: String,
}

impl TimeLog {
    pub fn generate_id() -> u32 {
        let mut rng = rand::rng();
        rng.random_range(1000..9999)
    }
}

impl std::fmt::Display for TimeLog {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Session ID: {}\n\rTime Spent: {} hours, {} minutes, {} seconds\n\rDate: {}",
            self.id, self.time_spent[0], self.time_spent[1], self.time_spent[2], self.date
        )
    }
}

impl From<std::time::Duration> for TimeLog {
    fn from(time: std::time::Duration) -> TimeLog {
        let seconds = time.as_secs();
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let seconds = seconds % 60;

        TimeLog {
            id: Self::generate_id(),
            time_spent: [hours, minutes, seconds],
            date: Local::now().format("%m-%d-%Y").to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_generate_id() {
        let id = TimeLog::generate_id();
        assert!(id >= 1000 && id < 9999);
    }

    #[test]
    fn test_from_duration() {
        //1 hour, 30 minutes, 45 seconds = 5445 seconds
        let duration = Duration::from_secs(5445);
        let time_log = TimeLog::from(duration);

        assert_eq!(time_log.time_spent[0], 1); //hours
        assert_eq!(time_log.time_spent[1], 30); //minutes
        assert_eq!(time_log.time_spent[2], 45); //seconds
        assert!(time_log.id >= 1000 && time_log.id < 9999);
        assert!(!time_log.date.is_empty());
    }

    #[test]
    fn test_from_duration_zero() {
        let duration = Duration::from_secs(0);
        let time_log = TimeLog::from(duration);

        assert_eq!(time_log.time_spent, [0, 0, 0]);
    }

    #[test]
    fn test_from_duration_large() {
        //test with 25 hours, 59 minutes, 59 seconds
        let duration = Duration::from_secs(25 * 3600 + 59 * 60 + 59);
        let time_log = TimeLog::from(duration);

        assert_eq!(time_log.time_spent[0], 25); //hours
        assert_eq!(time_log.time_spent[1], 59); //minutes
        assert_eq!(time_log.time_spent[2], 59); //seconds
    }
}
