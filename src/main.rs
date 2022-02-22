#![warn(clippy::pedantic)]

mod configs;
mod heroku_cli;

use crate::heroku_cli::{HerokuCmd, Status};
use chrono::NaiveDateTime;
use clap::{Parser, Subcommand};
use simple_logger::SimpleLogger;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Activate Heroku's maintenance mode given a maintenance window.
    MaintenanceWindow {
        /// The start of the maintenance window, in UTC.
        #[clap(short, long)]
        start: NaiveDateTime,

        /// The end of the maintenance window, in UTC.
        #[clap(short, long)]
        end: NaiveDateTime,
    },

    Maintenance {
        /// Activate the maintenance mode on Heroku
        #[clap(long)]
        on: bool,

        /// Deactivate the maintenance mode on Heroku
        #[clap(long)]
        off: bool,
    },
}

fn main() {
    SimpleLogger::new().with_utc_timestamps().init().unwrap();

    let args = Args::parse();
    log::debug!("{args:?}");

    match &args.command {
        Command::MaintenanceWindow { start, end } => {
            heroku_cli::maintenance::window::execute(start, end);
        }
        Command::Maintenance { on: true, off: _ } => {
            let cmd = HerokuCmd::MaintenanceMode(Status::On);
            heroku_cli::execute_for_all_apps_in_configs(&cmd);
        }

        Command::Maintenance { on: _, off: true } => {
            let cmd = HerokuCmd::MaintenanceMode(Status::Off);
            heroku_cli::execute_for_all_apps_in_configs(&cmd);
        }

        Command::Maintenance { on: _, off: _ } => (),
    }
}
