use crossterm::event::KeyEvent;
use termint::term::Term;

use crate::{
    error::Error,
    timer::Timer,
    tui::{active::Active, selector::Selector},
};

#[derive(Debug, Clone)]
pub enum Screen {
    Selector(Selector),
    Timer(Active),
}

impl Screen {
    pub fn selector() -> Self {
        Self::Selector(Selector::default())
    }

    pub fn timer(timer: Timer) -> Self {
        Self::Timer(Active::new(timer))
    }

    pub fn render(&mut self, term: &mut Term) -> Result<(), Error> {
        match self {
            Screen::Selector(selector) => selector.render(term),
            Screen::Timer(active) => active.render(term),
        }
    }

    pub fn update(&mut self, term: &mut Term) -> Result<(), Error> {
        match self {
            Screen::Selector(_) => Ok(()),
            Screen::Timer(active) => active.update(term),
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
        }
    }
}
