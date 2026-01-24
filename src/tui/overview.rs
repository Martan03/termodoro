use std::time::Duration;

use crossterm::event::{KeyCode, KeyEvent};
use termint::{
    enums::{Color, Wrap},
    geometry::Constraint,
    term::Term,
    widgets::{Element, Layout, Paragraph, Spacer, ToSpan},
};

use crate::{error::Error, stat::Stat, tui::screen::Screen};

#[derive(Debug, Clone)]
pub struct Overview {
    stat: Stat,
}

impl Overview {
    pub fn new(stat: Stat) -> Self {
        Self { stat }
    }

    pub fn render(&self, term: &mut Term) -> Result<(), Error> {
        let focus = format!(
            "Focus time:    {} (+{} flow overtime)",
            format_time(&self.stat.total_focus),
            format_time(&self.stat.overtime_focus)
        );
        let rest = format!(
            "Recovery time: {} (+{} extended rest)",
            format_time(&self.stat.total_rest),
            format_time(&self.stat.overtime_rest)
        );

        let mut content = Layout::vertical();
        content.push("Session overview:", 0..);
        content.push(focus.wrap(Wrap::Letter), 1);
        content.push(rest, 1);

        let mut wrapper = Layout::horizontal().center();
        wrapper.push(content, 0..);

        let mut main = Layout::vertical();
        main.push(Spacer::new(), Constraint::Fill(1));
        main.push(wrapper, 0..);
        main.push(Spacer::new(), Constraint::Fill(1));
        main.push(self.help(), 1..);
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

    fn help(&self) -> Element {
        Paragraph::new(vec!["[Esc|q]Quit".fg(Color::Gray).into()])
            .separator(" ")
            .into()
    }
}

fn format_time(dur: &Duration) -> String {
    let total_secs = dur.as_secs();
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;
    let secs = total_secs % 60;

    match (hours, mins) {
        (0, 0) => format!("{}s", secs),
        (0, m) => format!("{}m", m),
        (h, 0) => format!("{}h", h),
        (h, m) => format!("{}h {:02}m", h, m),
    }
}
