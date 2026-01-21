use crossterm::event::{KeyCode, KeyEvent};
use termint::{term::Term, widgets::Layout};

use crate::{error::Error, timer::Timer, tui::screen::Screen};

#[derive(Debug, Clone, PartialEq)]
pub struct Active {
    pub timer: Timer,
}

impl Active {
    pub fn new(timer: Timer) -> Self {
        Self { timer }
    }

    pub fn render(&self, term: &mut Term) -> Result<(), Error> {
        let mut main = Layout::vertical();
        main.push("Pomodoro :)", 1..);
        Ok(term.render(main)?)
    }

    pub fn update(&mut self, _term: &mut Term) -> Result<(), Error> {
        // TODO: timer updating
        Ok(())
    }

    pub fn on_key(
        &mut self,
        _term: &mut Term,
        event: KeyEvent,
    ) -> Result<Option<Screen>, Error> {
        match event.code {
            KeyCode::Esc | KeyCode::Char('q') => Err(Error::Exit),
            _ => Ok(None),
        }
    }
}
