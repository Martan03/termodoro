use std::time::Duration;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Stat {
    pub total_focus: Duration,
    pub overtime_focus: Duration,
    pub total_rest: Duration,
    pub overtime_rest: Duration,
}

impl Stat {
    pub fn new(f: Duration, of: Duration, r: Duration, or: Duration) -> Self {
        Self {
            total_focus: f,
            overtime_focus: of,
            total_rest: r,
            overtime_rest: or,
        }
    }

    pub fn total(&self) -> Duration {
        self.total_focus
            + self.overtime_focus
            + self.total_rest
            + self.overtime_rest
    }
}
