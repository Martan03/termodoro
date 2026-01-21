use std::time::Duration;

use crate::timer::Timer;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AppArgs {
    pub work: Option<Duration>,
    pub rest: Option<Duration>,
    pub long_rest: Option<Duration>,
    pub long_rate: Option<usize>,
}

impl AppArgs {
    /// Exports the app arguments into actual timer struct, returns None if
    /// all argument are None.
    pub fn export(self) -> Option<Timer> {
        if self.work.is_none()
            && self.rest.is_none()
            && self.long_rest.is_none()
            && self.long_rate.is_none()
        {
            return None;
        }

        let work = self
            .work
            .or_else(|| self.rest.map(|r| r * 5))
            .or_else(|| self.long_rest.map(|lr| lr * 5 / 3))
            .unwrap_or_else(|| Duration::from_mins(25));
        let rest = self.rest.unwrap_or(work / 5);
        Some(Timer {
            work,
            rest,
            long_rest: self.long_rest.unwrap_or(rest * 3),
            long_rate: self.long_rate.unwrap_or(4),
        })
    }
}
