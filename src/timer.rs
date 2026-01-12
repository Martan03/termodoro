use std::time::Duration;

/// Pomodoro timer settings
#[derive(Debug, Clone, PartialEq)]
pub struct Timer {
    pub work: Duration,
    pub rest: Duration,
}

impl Timer {
    /// Creates new timer configuration with given work and rest durations
    pub fn new(work: Duration, rest: Duration) -> Self {
        Self { work, rest }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            work: Duration::from_mins(25),
            rest: Duration::from_mins(5),
        }
    }
}
