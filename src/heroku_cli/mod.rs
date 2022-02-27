pub mod maintenance;

use crate::configs;
use crate::heroku_cli::maintenance::Status;
use std::process::{Command, Output};
use std::str;

pub enum HerokuCmd {
    Maintenance(maintenance::Status),
}

pub fn execute(arg: &str, app_name: String) -> Output {
    log::debug!("EXECUTING COMMAND: `heroku {arg:?} --app {app_name:?}");

    Command::new("heroku")
        .arg(arg)
        .arg("--app")
        .arg(app_name)
        .output()
        .expect("failed to execute command")
}

pub fn execute_for_all_apps_in_configs(mode: &HerokuCmd) {
    let configs = configs::get();

    let cmd = get_cmd(mode);
    log::debug!("CMD: {cmd:?}");

    println!("\n");
    for (app, envs) in &configs["apps"] {
        for env in envs {
            let app_name = build_heroku_app_name_from_env(app, env);
            let output = execute(cmd, app_name);
            print_info(&output);
            println!("\n");
        }
    }
}

pub fn build_heroku_app_name_from_env(app: &str, env: &str) -> String {
    format!("{}-{}", app, env)
}

fn get_cmd(mode: &HerokuCmd) -> &'static str {
    match mode {
        HerokuCmd::Maintenance(Status::On) => "maintenance:on",
        HerokuCmd::Maintenance(Status::Off) => "maintenance:off",
    }
}

fn print_info(output: &Output) {
    let stdout = str::from_utf8(&output.stdout).expect("Failed to read STDOUT");
    log::debug!("STDOUT: {stdout:?}");

    let stderr = str::from_utf8(&output.stderr).expect("Failed to read STDIN");
    log::debug!("STDERR: {stderr:?}");
}

mod tests {

    #[test]
    fn build_heroku_app_name_from_env_test() {
        assert_eq!(
            super::build_heroku_app_name_from_env("some-app", "staging"),
            "some-app-staging"
        );
        assert_eq!(
            super::build_heroku_app_name_from_env("some-app", "production"),
            "some-app-production"
        );
    }
}
