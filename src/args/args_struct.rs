use duration_str::parse;
use pareg::Pareg;

use crate::{args::action::Action, error::Error, timer::Timer};

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
        while let Some(arg) = args.next() {
            match arg {
                "-t" | "--timer" => {
                    let work = parse(args.next_arg::<&str>()?)?;
                    let rest = parse(args.next_arg::<&str>()?)?;
                    self.action = Action::App(Some(Timer::new(work, rest)));
                }
                _ => {
                    return Err(
                        format!("invalid app argument: {}", arg).into()
                    );
                }
            }
        }
        Ok(())
    }
}
