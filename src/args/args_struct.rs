use std::time::Duration;

use duration_str::parse;
use pareg::Pareg;
use termint::termal::{self, printcln};

use crate::{
    args::{action::Action, app::AppArgs},
    error::Error,
};

#[derive(Debug, Clone, Default)]
pub struct Args {
    pub action: Action,
}

impl Args {
    pub const VERSION_NUMBER: &str = {
        let v = option_env!("CARGO_PKG_VERSION");
        if let Some(v) = v { v } else { "unknown" }
    };

    /// Returns parsed CLI arguments
    ///
    /// # Errors
    /// Returns an [`Error`] if an issue with parsing arguments occures.
    pub fn parse(mut args: Pareg) -> Result<Self, Error> {
        let mut parsed = Self::default();
        match args.peek() {
            Some("-h") | Some("--help") | Some("help") => {
                parsed.action = Action::Help
            }
            Some("config") => parsed.action = Action::Config,
            None => {}
            _ => parsed.parse_app(&mut args)?,
        }
        Ok(parsed)
    }

    fn parse_app(&mut self, args: &mut Pareg) -> Result<(), Error> {
        let mut app = AppArgs::default();
        while let Some(arg) = args.next() {
            match arg {
                "-f" | "--focus" => app.work = Some(Self::read_dur(args)?),
                "-r" | "--rest" => app.rest = Some(Self::read_dur(args)?),
                "-l" | "--long-rest" => {
                    app.long_rest = Some(Self::read_dur(args)?)
                }
                "-c" | "--cycle" => app.long_rate = Some(args.next_arg()?),
                _ => {
                    return Err(
                        format!("invalid app argument: {}", arg).into()
                    );
                }
            }
        }
        self.action = Action::App(app);
        Ok(())
    }

    fn read_dur(args: &mut Pareg) -> Result<Duration, Error> {
        Ok(parse(args.next_arg::<&str>()?)?)
    }

    pub fn help() {
        printcln!(
            "Welcome to {'g}termodoro{'_} by {}{'_}
{'bl}Version {}{'_}

Pomodoro TUI implementation written in Rust.

{'g}Usage{'_}:
  {'c}termodoro{'_}
    Opens the TUI Pomodoro interval picker.

  {'c}termodoro{'_} [{'y}flags{'_}]
    Starts the TUI app according to the flags.

{'g}Flags{'_}:
  {'y}-h  --help{'_}
    Displays this help.

  {'y}-f  --focus{'_}
    Sets the focus timer length.

  {'y}-r  --rest{'_}
    Sets the rest timer length.

  {'y}-l  --long-rest{'_}
    Sets the long rest timer length.

  {'y}-c  --cycles{'_}
    Sets after how many cycles the long rest happens.",
            termal::gradient("Martan03", (0, 220, 255), (175, 80, 255)),
            Self::VERSION_NUMBER
        );
    }
}
