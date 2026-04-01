use std::{cell::RefCell, rc::Rc};

use termint::{
    enums::Color,
    geometry::Constraint,
    prelude::{KeyCode, KeyEvent},
    style::Style,
    term::Action,
    widgets::{Layout, List, ListState, Paragraph, Spacer, ToSpan},
};

use crate::{
    error::Error,
    message::Message,
    timer::Timer,
    tui::{Element, screen::Screen},
};

#[derive(Debug, Clone)]
pub struct Selector {
    state: Rc<RefCell<ListState>>,
}

impl Selector {
    pub fn view(&self) -> Element {
        let items = vec!["25/5 (15@4)", "50/10 (30@4)"];
        let list = List::new(items, self.state.clone())
            .auto_scroll()
            .highlight_symbol("> ")
            .highlight_style(Style::new().fg(Color::Cyan))
            .selected_style(Style::new().fg(Color::Cyan))
            .on_click(Message::SplitSelect);

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
        main.into()
    }

    pub fn on_key(
        &mut self,
        event: KeyEvent,
    ) -> Result<(Action, Option<Screen>), Error> {
        match event.code {
            KeyCode::Up | KeyCode::Char('k') => self.select_prev(),
            KeyCode::Down | KeyCode::Char('j') => self.select_next(),
            KeyCode::Enter => return Ok((Action::RENDER, self.pick())),
            KeyCode::Esc | KeyCode::Char('q') => {
                return Ok((Action::QUIT, None));
            }
            _ => return Ok((Action::NONE, None)),
        }
        return Ok((Action::RENDER, None));
    }

    pub fn message(&mut self, message: Message) -> (Action, Option<Screen>) {
        match message {
            Message::SplitSelect(id) => {
                if let Some(t) = Timer::from_index(id) {
                    return (Action::RENDER, Some(Screen::timer(t)));
                }
            }
            _ => return (Action::NONE, None),
        }
        (Action::NONE, None)
    }

    fn help(&self) -> Element {
        Paragraph::new(vec![
            "[↑|k]Prev. sel.".fg(Color::Gray),
            "[↓|j]Next sel.".fg(Color::Gray),
            "[Enter]Select".fg(Color::Gray),
            "[Esc|q]Quit".fg(Color::Gray),
        ])
        .separator(" ")
        .into()
    }

    fn pick(&self) -> Option<Screen> {
        let state = self.state.borrow();
        let Some(sel) = state.selected else {
            return None;
        };
        Timer::from_index(sel).map(|t| Screen::timer(t))
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
