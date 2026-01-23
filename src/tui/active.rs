use std::time::{Duration, Instant};

use chrono::{DateTime, Local};
use crossterm::event::{KeyCode, KeyEvent};
use termint::{
    geometry::Constraint,
    term::Term,
    widgets::{Layout, Spacer},
};

use crate::{
    error::Error,
    timer::Timer,
    tui::{IntervalType, screen::Screen, widgets::asci_timer::AsciTimer},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Active {
    timer: Timer,
    deadline: Instant,
    wall_deadline: DateTime<Local>,
    interval: IntervalType,
    reps: usize,
    pause_at: Option<Instant>,
    asci: AsciTimer,
}

impl Active {
    pub fn new(timer: Timer) -> Self {
        let deadline = Instant::now() + timer.work;
        let wall_deadline =
            Local::now() + chrono::Duration::from_std(timer.work).unwrap();
        Self {
            timer,
            deadline,
            wall_deadline,
            interval: IntervalType::Work,
            reps: 0,
            pause_at: None,
            asci: AsciTimer::regular(),
        }
    }

    pub fn render(&self, term: &mut Term) -> Result<(), Error> {
        term.clear_cache();
        let time = self.asci.element(self.format_remaining());

        let mut content = Layout::vertical();
        content.push(time, self.asci.height);

        let mut wrapper = Layout::horizontal().center();
        wrapper.push(content, 0..);

        let mut main = Layout::vertical();
        main.push(Spacer::new(), Constraint::Fill(1));
        main.push(wrapper, 0..);
        main.push(Spacer::new(), Constraint::Fill(1));
        Ok(term.render(main)?)
    }

    pub fn update(&self, term: &mut Term) -> Result<(), Error> {
        self.render(term)
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

    fn total(&self) -> Duration {
        match self.interval {
            IntervalType::Work => self.timer.work,
            IntervalType::Rest if self.reps % self.timer.long_rate == 0 => {
                self.timer.long_rest
            }
            _ => self.timer.rest,
        }
    }

    fn remaining(&self) -> Duration {
        match self.pause_at {
            Some(t) => self.deadline.saturating_duration_since(t),
            None => self.deadline.saturating_duration_since(Instant::now()),
        }
    }

    fn format_remaining(&self) -> String {
        let secs = self.remaining().as_secs();
        format!("{:02}:{:02}", secs / 60, secs % 60)
    }

    fn format_deadline(&self) -> String {
        self.wall_deadline.format("%H:%M").to_string()
    }
}
