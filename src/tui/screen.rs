use crossterm::event::KeyEvent;
use termint::term::Term;

use crate::{
    config::Config,
    error::Error,
    stat::Stat,
    timer::Timer,
    tui::{active::Active, overview::Overview, selector::Selector},
};

#[derive(Debug, Clone)]
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

    pub fn render(&mut self, term: &mut Term) -> Result<(), Error> {
        match self {
            Self::Selector(selector) => selector.render(term),
            Self::Timer(active) => active.render(term),
            Self::Overview(overview) => overview.render(term),
        }
    }

    pub fn update(
        &mut self,
        term: &mut Term,
        conf: &Config,
    ) -> Result<(), Error> {
        match self {
            Self::Selector(_) => Ok(()),
            Self::Timer(active) => active.update(term, conf),
            Self::Overview(_) => Ok(()),
        }
    }

    pub fn on_key(
        &mut self,
        term: &mut Term,
        event: KeyEvent,
    ) -> Result<Option<Self>, Error> {
        match self {
            Screen::Selector(selector) => selector.on_key(term, event),
            Screen::Timer(active) => active.on_key(term, event),
            Screen::Overview(overview) => overview.on_key(term, event),
        }
    }
}
