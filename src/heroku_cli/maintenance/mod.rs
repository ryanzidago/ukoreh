use crate::configs;
use crate::heroku_cli;

use std::process::Output;
use std::str;
pub mod window;

pub enum HerokuCmd {
    MaintenanceMode(Status),
}

pub enum Status {
    On,
    Off,
}

pub fn execute(mode: &HerokuCmd) {
    let configs = configs::get();

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

fn get_cmd(mode: &HerokuCmd) -> &'static str {
    match mode {
        HerokuCmd::MaintenanceMode(Status::On) => "maintenance:on",
        HerokuCmd::MaintenanceMode(Status::Off) => "maintenance:off",
    }
}

fn print_info(output: &Output) {
    let stdout = str::from_utf8(&output.stdout).expect("Failed to read STDOUT");
    log::debug!("STDOUT: {stdout:?}");

    let stderr = str::from_utf8(&output.stderr).expect("Failed to read STDIN");
    log::debug!("STDERR: {stderr:?}");
}
