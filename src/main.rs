use std::{
    fs::create_dir_all,
    process::{Command, ExitCode},
};

use pareg::Pareg;
use termint::{term::Term, termal::eprintcln};

use crate::{
    app::App,
    args::{action::Action, app::AppArgs, args_struct::Args},
    config::Config,
    error::Error,
};

pub mod app;
pub mod args;
pub mod audio;
pub mod config;
pub mod error;
pub mod message;
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

    Term::default().setup()?.with_mouse().run(&mut app)?;
    Ok(())
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
