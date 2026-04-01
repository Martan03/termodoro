use crate::{
    stat::Stat,
    timer::Timer,
    tui::{active::Active, overview::Overview, selector::Selector},
};

#[derive(Debug)]
pub enum Screen {
    Selector(Selector),
    Timer(Active),
    Overview(Overview),
}

impl Screen {
    pub fn selector() -> Self {
        Self::Selector(Selector::default())
    }

    pub fn timer(timer: Timer) -> Self {
        Self::Timer(Active::new(timer))
    }

    pub fn overview(stat: Stat) -> Self {
        Self::Overview(Overview::new(stat))
    }
}
