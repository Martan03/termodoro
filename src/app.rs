use std::{
    io::{Write, stdout},
    time::Duration,
};

use crossterm::{
    event::{Event, KeyEvent, poll},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use termint::term::Term;

use crate::{config::Config, error::Error, timer::Timer, tui::screen::Screen};

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

    /// Runs the app - does prep. work before main loop and then clean up
    pub fn run(&mut self) -> Result<(), Error> {
        enable_raw_mode()?;
        // Swaps print buffer, clears screen and hides cursor
        print!("\x1b[?1049h\x1b[2J\x1b[?25l");
        _ = stdout().flush();

        let res = self.main_loop();

        // Restores screen
        print!("\x1b[?1049l\x1b[?25h");
        _ = stdout().flush();
        disable_raw_mode()?;

        match res {
            Err(Error::Exit) => Ok(()),
            _ => res,
        }
    }

    fn main_loop(&mut self) -> Result<(), Error> {
        self.render()?;
        loop {
            if poll(Duration::from_millis(100))? {
                self.event()?;
            }

            self.screen.update(&mut self.term, &self.config)?;
        }
    }

    fn render(&mut self) -> Result<(), Error> {
        self.screen.render(&mut self.term)
    }

    fn event(&mut self) -> Result<(), Error> {
        match crossterm::event::read()? {
            Event::Key(e) => self.key_handler(e),
            Event::Resize(_, _) => self.render(),
            _ => Ok(()),
        }
    }

    fn key_handler(&mut self, event: KeyEvent) -> Result<(), Error> {
        if let Some(screen) = self.screen.on_key(&mut self.term, event)? {
            self.screen = screen;
            self.render()?;
        }
        Ok(())
    }
}
