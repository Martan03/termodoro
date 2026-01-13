use std::process::ExitCode;

use pareg::Pareg;
use termint::termal::eprintcln;

use crate::{
    args::{action::Action, args_struct::Args},
    error::Error,
};

pub mod app;
pub mod args;
pub mod error;
pub mod timer;
pub mod tui;

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintcln!("{'r}Error:{'_} {}", e);
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Error> {
    let args = Args::parse(Pareg::args())?;
    match args.action {
        Action::App(timer) => println!("{:?}", timer),
        Action::Help => println!("help wip"),
    }

    Ok(())
}
