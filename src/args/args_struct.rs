use std::time::Duration;

use duration_str::parse;
use pareg::Pareg;

use crate::{
    args::{action::Action, app::AppArgs},
    error::Error,
};

#[derive(Debug, Clone, Default)]
pub struct Args {
    pub action: Action,
}

impl Args {
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
            None => {}
            _ => parsed.parse_app(&mut args)?,
        }
        Ok(parsed)
    }

    fn parse_app(&mut self, args: &mut Pareg) -> Result<(), Error> {
        let mut app = AppArgs::default();
        while let Some(arg) = args.next() {
            match arg {
                "-w" | "--work" => app.work = Some(Self::read_dur(args)?),
                "-r" | "--rest" => app.rest = Some(Self::read_dur(args)?),
                "-l" | "--long-rest" => {
                    app.long_rest = Some(Self::read_dur(args)?)
                }
                "-i" | "--intervals" => {
                    app.long_rate = Some(args.next_arg()?)
                }
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
}
