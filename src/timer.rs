use std::time::Duration;

/// Pomodoro timer settings
#[derive(Debug, Clone, PartialEq)]
pub struct Timer {
    pub work: Duration,
    pub rest: Duration,
    pub long_rest: Duration,
    pub long_rate: usize,
}

impl Timer {
    /// Creates new pomodoro timer with given work length, rest calculated
    /// as 1/5 of the work length and long rest as 2/5 of work length.
    pub fn new(work: Duration) -> Self {
        Self {
            work,
            rest: work / 5,
            long_rest: work / 5 * 3,
            long_rate: 4,
        }
    }

    /// Sets the work length to the given value.
    pub fn work(mut self, len: Duration) -> Self {
        self.work = len;
        self
    }

    /// Sets the rest length to the given value.
    pub fn rest(mut self, len: Duration) -> Self {
        self.rest = len;
        self
    }

    /// Sets the long rest length to the given value.
    pub fn long_rest(mut self, len: Duration) -> Self {
        self.long_rest = len;
        self
    }

    /// Sets the long rest rate - after how many intervals the long rest is
    /// used.
    pub fn long_rate(mut self, rate: usize) -> Self {
        self.long_rate = rate;
        self
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            work: Duration::from_mins(25),
            rest: Duration::from_mins(5),
            long_rest: Duration::from_mins(15),
            long_rate: 4,
        }
    }
}
