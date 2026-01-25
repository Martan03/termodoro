use std::{
    fs::create_dir_all,
    io::{Write, stdout},
    panic::{set_hook, take_hook},
    process::{Command, ExitCode},
};

use crossterm::terminal::{disable_raw_mode, is_raw_mode_enabled};
use pareg::Pareg;
use termint::termal::eprintcln;

use crate::{
    app::App,
    args::{action::Action, app::AppArgs, args_struct::Args},
    config::Config,
    error::Error,
};

pub mod app;
pub mod args;
pub mod config;
pub mod error;
pub mod player;
pub mod stat;
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
        Action::Config => config()?,
        Action::Help => Args::help(),
    }

    Ok(())
}

fn run_app(args: AppArgs) -> Result<(), Error> {
    let timer = args.export();
    let mut app = App::new(timer);
    app.run()
}

fn config() -> Result<(), Error> {
    let editor = std::env::var("EDITOR").unwrap_or("vi".to_string());
    create_dir_all(Config::dir())?;
    let file = Config::file();
    if !file.exists() {
        Config::default().to_default_json()?;
    }

    Command::new(editor).arg(file).spawn()?.wait()?;
    Ok(())
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
