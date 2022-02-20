use crate::configs;
use crate::heroku_cli;
use chrono::{offset::TimeZone, DateTime, NaiveDateTime, Utc};
use std::process::Output;
use std::str;
use std::thread;

pub enum HerokuCmd {
    MaintenanceMode(Status),
}

pub enum Status {
    On,
    Off,
}

struct MaintenanceWindow {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl MaintenanceWindow {
    fn new(start: &NaiveDateTime, end: &NaiveDateTime) -> Self {
        let start: DateTime<Utc> = Utc.from_local_datetime(start).unwrap();
        let end: DateTime<Utc> = Utc.from_local_datetime(end).unwrap();

        assert!(
            !(start < Utc::now() || end < Utc::now()),
            "The maintenance window must not be in the past"
        );
        assert!(!(start > end), "Invalid maintenance window");

        MaintenanceWindow { start, end }
    }
}

pub fn execute(start: &NaiveDateTime, end: &NaiveDateTime) {
    let configs = configs::get();
    let window = MaintenanceWindow::new(start, end);

    sleep_until(&window.start);
    _execute(&configs, &HerokuCmd::MaintenanceMode(Status::On));

    sleep_until(&window.end);
    _execute(&configs, &HerokuCmd::MaintenanceMode(Status::Off));
}

fn get_cmd(mode: &HerokuCmd) -> &'static str {
    match mode {
        HerokuCmd::MaintenanceMode(Status::On) => "maintenance:on",
        HerokuCmd::MaintenanceMode(Status::Off) => "maintenance:off",
    }
}

fn _execute(configs: &configs::Configs, mode: &HerokuCmd) {
    let cmd = get_cmd(mode);
    log::debug!("MAINTENANCE_MODE: {cmd:?}");

    println!("\n");
    for (app, envs) in &configs["apps"] {
        for env in envs {
            let app_name = heroku_cli::build_heroku_app_name_from_env(app, env);
            let output = heroku_cli::execute_cmd(cmd, app_name);
            print_info(&output);
            println!("\n");
        }
    }
}

fn sleep_until(date_time: &DateTime<Utc>) {
    let duration = date_time
        .signed_duration_since(Utc::now())
        .to_std()
        .expect("Invalid duration");

    log::debug!("DURATION: {duration:?}");
    thread::sleep(duration);
}

fn print_info(output: &Output) {
    let stdout = str::from_utf8(&output.stdout).expect("Failed to read STDOUT");
    log::debug!("STDOUT: {stdout:?}");

    let stderr = str::from_utf8(&output.stderr).expect("Failed to read STDIN");
    log::debug!("STDERR: {stderr:?}");
}
