use std::time::Duration;

use termint::{
    enums::{Color, Modifier, Wrap},
    geometry::Constraint,
    prelude::{KeyCode, KeyEvent},
    term::Action,
    widgets::{Block, Layout, Paragraph, Spacer, ToSpan},
};

use crate::{
    error::Error,
    stat::Stat,
    tui::{Element, screen::Screen},
};

#[derive(Debug, Clone)]
pub struct Overview {
    stat: Stat,
}

impl Overview {
    pub fn new(stat: Stat) -> Self {
        Self { stat }
    }

    pub fn view(&self) -> Element {
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
        let total =
            format!("Total time:    {}", format_time(&self.stat.total()));

        let mut content = Layout::vertical();
        let title = "Session overview:"
            .modifier(Modifier::UNDERLINED)
            .fg(Color::Cyan);
        content.push(title, 0..);
        content.push(Spacer::new(), 1);
        content.push(focus.wrap(Wrap::Letter), 1);
        content.push(rest, 1);
        content.push(Block::empty(), 1);
        content.push(total.wrap(Wrap::Letter), 1);

        let mut wrapper = Layout::horizontal().center();
        wrapper.push(content, 0..);

        let mut main = Layout::vertical();
        main.push(Spacer::new(), Constraint::Fill(1));
        main.push(wrapper, 0..);
        main.push(Spacer::new(), Constraint::Fill(1));
        main.push(self.help(), 1..);
        main.into()
    }

    pub fn on_key(
        &mut self,
        event: KeyEvent,
    ) -> Result<(Action, Option<Screen>), Error> {
        match event.code {
            KeyCode::Esc | KeyCode::Char('q') => Ok((Action::QUIT, None)),
            _ => Ok((Action::NONE, None)),
        }
    }

    fn help(&self) -> Element {
        Paragraph::new(vec!["[Esc|q]Quit".fg(Color::Gray)])
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
