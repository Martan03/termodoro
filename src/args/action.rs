use crate::timer::Timer;

/// CLI arguments available actions
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    App(Option<Timer>),
    Help,
}

impl Default for Action {
    fn default() -> Self {
        Self::App(None)
    }
}
