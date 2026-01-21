use crossterm::event::{KeyCode, KeyEvent};
use termint::{term::Term, widgets::Layout};

use crate::{error::Error, tui::screen::Screen};

#[derive(Debug, Clone, PartialEq)]
pub struct Selector;

impl Selector {
    pub fn render(&self, term: &mut Term) -> Result<(), Error> {
        let mut main = Layout::vertical();
        main.push("Pomodoro selector", 1..);
        Ok(term.render(main)?)
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
