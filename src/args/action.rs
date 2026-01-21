use crate::args::app::AppArgs;

/// CLI arguments available actions
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    App(AppArgs),
    Help,
}

impl Default for Action {
    fn default() -> Self {
        Self::App(AppArgs::default())
    }
}
