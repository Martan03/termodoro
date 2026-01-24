use std::{cell::RefCell, rc::Rc, time::Duration};

use crossterm::event::{KeyCode, KeyEvent};
use termint::{
    enums::Color,
    geometry::Constraint,
    style::Style,
    term::Term,
    widgets::{Element, Layout, List, ListState, Paragraph, Spacer, ToSpan},
};

use crate::{error::Error, timer::Timer, tui::screen::Screen};

#[derive(Debug, Clone)]
pub struct Selector {
    state: Rc<RefCell<ListState>>,
}

impl Selector {
    pub fn render(&self, term: &mut Term) -> Result<(), Error> {
        let items = vec!["25/5 (15@4)", "50/10 (30@4)"];
        let list = List::new(items, self.state.clone())
            .auto_scroll()
            .highlight_symbol("> ")
            .highlight_style(Style::new().fg(Color::Cyan))
            .selected_style(Style::new().fg(Color::Cyan));

        let mut content = Layout::vertical();
        content.push("Choose a Pomodoro split:", 1..);
        content.push(list, 0..);

        let mut wrapper = Layout::horizontal().center();
        wrapper.push(content, 0..);

        let mut main = Layout::vertical();
        main.push(Spacer::new(), Constraint::Fill(1));
        main.push(wrapper, 0..);
        main.push(Spacer::new(), Constraint::Fill(1));
        main.push(self.help(), 0..);
        Ok(term.render(main)?)
    }

    pub fn on_key(
        &mut self,
        term: &mut Term,
        event: KeyEvent,
    ) -> Result<Option<Screen>, Error> {
        match event.code {
            KeyCode::Up | KeyCode::Char('k') => self.select_prev(),
            KeyCode::Down | KeyCode::Char('j') => self.select_next(),
            KeyCode::Enter => return Ok(self.pick()),
            KeyCode::Esc | KeyCode::Char('q') => return Err(Error::Exit),
            _ => return Ok(None),
        }
        self.render(term)?;
        return Ok(None);
    }

    fn help(&self) -> Element {
        Paragraph::new(vec![
            "[↑|k]Prev. sel.".fg(Color::Gray).into(),
            "[↓|j]Next sel.".fg(Color::Gray).into(),
            "[Enter]Select".fg(Color::Gray).into(),
            "[Esc|q]Quit".fg(Color::Gray).into(),
        ])
        .separator(" ")
        .into()
    }

    fn pick(&self) -> Option<Screen> {
        let state = self.state.borrow();
        let Some(sel) = state.selected else {
            return None;
        };
        match sel {
            0 => Some(Screen::timer(Timer::new(Duration::from_mins(25)))),
            1 => Some(Screen::timer(Timer::new(Duration::from_mins(50)))),
            _ => None,
        }
    }

    fn select_prev(&mut self) {
        self.select(|s, id| s.selected = Some(id.saturating_sub(1)));
    }

    fn select_next(&mut self) {
        self.select(|s, id| {
            if id + 1 < 2 {
                s.selected = Some(id + 1);
            }
        });
    }

    fn select<F>(&mut self, set_id: F)
    where
        F: Fn(&mut ListState, usize),
    {
        let mut state = self.state.borrow_mut();
        let Some(sel) = state.selected else {
            return;
        };

        set_id(&mut state, sel);
    }
}

impl Default for Selector {
    fn default() -> Self {
        Self {
            state: Rc::new(RefCell::new(ListState::selected(0, 0))),
        }
    }
}
