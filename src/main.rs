#![warn(clippy::pedantic)]

mod configs;
mod heroku_cli;

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
    /// Activates Heroku's maintenance mode for all apps listed in `src/heroku_apps.yml`
    /// given a maintenance window.
    MaintenanceMode {
        /// The start of the maintenance window, in UTC.
        #[clap(short, long)]
        start: NaiveDateTime,

        /// The end of the maintenance window, in UTC.
        #[clap(short, long)]
        end: NaiveDateTime,
    },
}

fn main() {
    SimpleLogger::new().with_utc_timestamps().init().unwrap();

    let args = Args::parse();
    log::debug!("{args:?}");

    match &args.command {
        Command::MaintenanceMode { start, end } => heroku_cli::maintenance::execute(start, end),
    }
}
