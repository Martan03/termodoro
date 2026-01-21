use std::{
    io::{Write, stdout},
    panic::{set_hook, take_hook},
    process::ExitCode,
};

use crossterm::terminal::{disable_raw_mode, is_raw_mode_enabled};
use pareg::Pareg;
use termint::termal::eprintcln;

use crate::{
    app::App,
    args::{action::Action, app::AppArgs, args_struct::Args},
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
    register_panic_hook();

    let args = Args::parse(Pareg::args())?;
    match args.action {
        Action::App(args) => run_app(args)?,
        Action::Help => println!("help wip"),
    }

    Ok(())
}

fn run_app(args: AppArgs) -> Result<(), Error> {
    let timer = args.export();
    let mut app = App::new(timer);
    app.run()
}

fn register_panic_hook() {
    let hook = take_hook();
    set_hook(Box::new(move |pi| {
        if is_raw_mode_enabled().unwrap_or_default() {
            // Restores screen
            print!("\x1b[?1049l\x1b[?25h");
            _ = stdout().flush();
            _ = disable_raw_mode();
        }
        hook(pi);
    }));
}
