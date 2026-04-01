use std::{
    cell::Cell,
    rc::Rc,
    time::{Duration, Instant},
};

use chrono::{DateTime, Local};
use termint::{
    enums::{Color, Wrap},
    geometry::Constraint,
    prelude::{KeyCode, KeyEvent},
    style::Style,
    term::Action,
    widgets::{Button, Layout, Paragraph, ProgressBar, Spacer, ToSpan},
};

use crate::{
    audio::player::Player,
    config::Config,
    error::Error,
    message::Message,
    stat::Stat,
    timer::Timer,
    tui::{
        Element, IntervalType, screen::Screen, widgets::asci_timer::AsciTimer,
    },
};

#[derive(Debug)]
pub struct Active {
    timer: Timer,
    deadline: Instant,
    wall_deadline: DateTime<Local>,
    interval: IntervalType,
    reps: usize,
    pause_at: Option<Instant>,
    asci: AsciTimer,
    dialog_opt: bool,
    focus_overtime: Duration,
    rest_overtime: Duration,
    player: Player,
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
            dialog_opt: true,
            focus_overtime: Duration::ZERO,
            rest_overtime: Duration::ZERO,
            player: Player::new(),
        }
    }

    pub fn view(&self) -> Element {
        let (content, help) = match self.interval {
            IntervalType::Work => (self.render_timer(), self.timer_help()),
            IntervalType::Pending(rest) => {
                (self.render_pending(rest), self.pending_help())
            }
            IntervalType::Rest => (self.render_timer(), self.timer_help()),
        };

        let mut main = Layout::vertical();
        main.push(Spacer::new(), Constraint::Fill(1));
        main.push(content, 0..);
        main.push(Spacer::new(), Constraint::Fill(1));
        main.push(help, 1..);
        main.into()
    }

    pub fn update(&mut self, conf: &Config) -> Action {
        if !self.interval.is_pending() && self.remaining().is_zero() {
            let rest = self.interval == IntervalType::Work;
            self.reps += rest as usize;
            self.interval = IntervalType::Pending(rest);
            // TODO: log the error
            _ = self.play_sound(conf, rest);
        }
        Action::RENDER
    }

    pub fn on_key(
        &mut self,
        event: KeyEvent,
    ) -> Result<(Action, Option<Screen>), Error> {
        match event.code {
            KeyCode::Esc | KeyCode::Char('q') => Ok((Action::QUIT, None)),
            _ => match self.interval {
                IntervalType::Work => self.listen_timer(event),
                IntervalType::Pending(rest) => {
                    self.listen_pending(event, rest)
                }
                IntervalType::Rest => self.listen_timer(event),
            },
        }
    }

    pub fn message(&mut self, message: Message) -> (Action, Option<Screen>) {
        let IntervalType::Pending(rest) = &self.interval else {
            return (Action::NONE, None);
        };

        let rest = *rest;
        match message {
            Message::Continue if rest => self.start_rest(),
            Message::Continue => {
                self.rest_overtime += self.overtime();
                self.set_deadline(self.timer.work);
                self.interval = IntervalType::Work;
            }
            Message::Finish => {
                return (
                    Action::RENDER,
                    Some(Screen::overview(self.finish_session(rest))),
                );
            }
            _ => return (Action::NONE, None),
        }
        (Action::RENDER, None)
    }
}

impl Active {
    pub fn render_timer(&self) -> Element {
        let rem = Self::format_duration(&self.remaining());
        let (time, width) = self.asci.element(rem);
        let progress = self.progress();
        let pb = ProgressBar::new(Rc::new(Cell::new(progress)))
            .style(Style::new().bg(Color::Gray))
            .thumb_chars(['█']);

        let mut pb_label = Layout::horizontal();
        pb_label.push(format!("{}%", (progress * 100.) as usize), 2..);
        pb_label.push(Spacer::new(), Constraint::Fill(1));
        pb_label.push(self.format_deadline(), 0..);

        let mut content = Layout::<Message>::vertical();
        content.push(time, self.asci.height);
        content.push(Spacer::new(), 1);
        content.push(pb, 1);
        content.push(pb_label, 1);

        let mut wrapper = Layout::horizontal().center();
        wrapper.push(content, width + 1);
        wrapper.into()
    }

    pub fn render_pending(&self, rest: bool) -> Element {
        let title = if rest { "Rest" } else { "Focus" };

        let overtime = self.overtime();
        let overtime_text = match overtime.is_zero() {
            true => String::new(),
            false => format!("+{}", Self::format_duration(&overtime)),
        };

        let mut ops = Layout::horizontal();
        let mut op1 =
            format!("  {title}  ").wrap(Wrap::Letter).bg(Color::Gray);
        let mut op2 = " Finish ".wrap(Wrap::Letter).bg(Color::Gray);
        match self.dialog_opt {
            true => op1 = op1.bg(Color::Cyan).fg(Color::Black),
            false => op2 = op2.bg(Color::Cyan).fg(Color::Black),
        }
        ops.push(Button::new(op1).on_click(Message::Continue), 0..);
        ops.push(Spacer::new(), 1);
        ops.push(Button::new(op2).on_click(Message::Finish), 0..);

        let mut content = Layout::vertical();
        content.push(overtime_text.fg(Color::Gray), 1..);
        content.push(format!("Ready to {}?", title.to_lowercase()), 1..);
        content.push(Spacer::new(), 1);
        content.push(ops, 1);

        let mut wrapper = Layout::horizontal().center();
        wrapper.push(content, 0..);
        wrapper.into()
    }

    fn listen_timer(
        &mut self,
        event: KeyEvent,
    ) -> Result<(Action, Option<Screen>), Error> {
        match event.code {
            KeyCode::Char(' ') => {
                self.toggle_pause();
                Ok((Action::NONE, None))
            }
            _ => Ok((Action::NONE, None)),
        }
    }

    fn listen_pending(
        &mut self,
        event: KeyEvent,
        rest: bool,
    ) -> Result<(Action, Option<Screen>), Error> {
        match event.code {
            KeyCode::Left | KeyCode::Char('h') => self.dialog_opt = true,
            KeyCode::Right | KeyCode::Char('l') => self.dialog_opt = false,
            KeyCode::Enter if !self.dialog_opt => {
                return Ok((
                    Action::RENDER,
                    Some(Screen::overview(self.finish_session(rest))),
                ));
            }
            KeyCode::Enter if rest => self.start_rest(),
            KeyCode::Enter => {
                self.rest_overtime += self.overtime();
                self.set_deadline(self.timer.work);
                self.interval = IntervalType::Work;
            }
            _ => return Ok((Action::NONE, None)),
        }
        Ok((Action::RENDER, None))
    }

    fn finish_session(&mut self, rest: bool) -> Stat {
        match rest {
            true => self.focus_overtime += self.overtime(),
            false => self.rest_overtime += self.overtime(),
        }

        let focus = self.timer.work * self.reps as u32;
        let rests = self.reps.saturating_sub(rest as usize);
        let lr = (rests / self.timer.long_rate) as u32;
        let sr = rests as u32 - lr;
        let rt = lr * self.timer.long_rest + sr * self.timer.rest;
        Stat::new(focus, self.focus_overtime, rt, self.rest_overtime)
    }

    fn toggle_pause(&mut self) {
        match self.pause_at {
            Some(i) => {
                self.set_deadline(self.deadline.saturating_duration_since(i));
                self.pause_at = None;
            }
            None => self.pause_at = Some(Instant::now()),
        }
    }

    fn start_rest(&mut self) {
        self.focus_overtime += self.overtime();
        let rest = match self.reps % self.timer.long_rate == 0 {
            true => self.timer.long_rest,
            false => self.timer.rest,
        };
        self.set_deadline(rest);
        self.interval = IntervalType::Rest;
    }

    fn play_sound(&mut self, conf: &Config, rest: bool) -> Result<(), Error> {
        let source = match rest {
            true => &conf.focus_end_sound,
            false => &conf.rest_end_sound,
        };
        source.play(&mut self.player, rest)
    }

    fn set_deadline(&mut self, rem: Duration) {
        self.deadline = Instant::now() + rem;
        self.wall_deadline =
            Local::now() + chrono::Duration::from_std(rem).unwrap();
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

    fn overtime(&self) -> Duration {
        Instant::now()
            .saturating_duration_since(self.deadline)
            .saturating_sub(Duration::from_secs(5))
    }

    fn progress(&self) -> f64 {
        1. - (self.remaining().as_secs_f64() / self.total().as_secs_f64())
    }

    fn format_duration(dur: &Duration) -> String {
        let secs = dur.as_secs();
        format!("{:02}:{:02}", secs / 60, secs % 60)
    }

    fn format_deadline(&self) -> String {
        self.wall_deadline.format("%H:%M").to_string()
    }

    fn timer_help(&self) -> Element {
        Paragraph::new(vec![
            "[Space]Resume/pause".fg(Color::Gray),
            "[Esc|q]Quit".fg(Color::Gray),
        ])
        .separator(" ")
        .into()
    }

    fn pending_help(&self) -> Element {
        Paragraph::new(vec![
            "[←|h]Prev. sel.".fg(Color::Gray),
            "[→|l]Next sel.".fg(Color::Gray),
            "[Enter]Select".fg(Color::Gray),
            "[Esc|q]Quit".fg(Color::Gray),
        ])
        .separator(" ")
        .into()
    }
}
