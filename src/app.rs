use std::time::Duration;

use termint::{
    prelude::Event,
    term::{Action, Application, Frame, Term},
    widgets::Element,
};

use crate::{config::Config, timer::Timer, tui::screen::Screen};

#[derive(Debug)]
pub struct App {
    pub screen: Screen,
    pub config: Config,
    pub term: Term,
}

impl App {
    /// Creates new app, if timer is None it displays pomodoro interval
    /// selector, otherwise opens the timer page.
    pub fn new(timer: Option<Timer>) -> Self {
        let screen = match timer {
            Some(t) => Screen::timer(t),
            None => Screen::selector(),
        };
        Self {
            screen,
            config: Config::from_default_json(),
            term: Term::new(),
        }
    }
}

impl Application for App {
    type Message = crate::message::Message;

    fn view(&self, _frame: &Frame) -> Element<Self::Message> {
        match &self.screen {
            Screen::Selector(selector) => selector.view(),
            Screen::Timer(active) => active.view(),
            Screen::Overview(overview) => overview.view(),
        }
    }

    fn event(&mut self, event: Event) -> Action {
        let Event::Key(key) = event else {
            return Action::NONE;
        };

        let res = match &mut self.screen {
            Screen::Selector(selector) => selector.on_key(key),
            Screen::Timer(active) => active.on_key(key),
            Screen::Overview(overview) => overview.on_key(key),
        };

        // TODO: log the error
        let Ok((mut action, screen)) = res else {
            return Action::NONE;
        };

        if let Some(screen) = screen {
            self.screen = screen;
            action = Action::RENDER;
        }
        action
    }

    fn message(&mut self, message: Self::Message) -> Action {
        let (mut action, screen) = match &mut self.screen {
            Screen::Selector(selector) => selector.message(message),
            Screen::Timer(active) => active.message(message),
            Screen::Overview(_) => return Action::NONE,
        };
        if let Some(screen) = screen {
            self.screen = screen;
            action = Action::RENDER;
        }
        action
    }

    fn update(&mut self, _delta: Duration) -> Action {
        match &mut self.screen {
            Screen::Timer(active) => active.update(&self.config),
            _ => Action::NONE,
        }
    }
}
