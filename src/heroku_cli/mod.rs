use std::process::{Command, Output};

pub mod maintenance;

pub fn execute_cmd(arg: &str, app_name: String) -> Output {
    log::debug!("EXECUTING COMMAND: `heroku {arg:?} --app {app_name:?}");

    Command::new("heroku")
        .arg(arg)
        .arg("--app")
        .arg(app_name)
        .output()
        .expect("failed to execute command")
}

pub fn build_heroku_app_name_from_env(app: &str, env: &str) -> String {
    format!("{}-{}", app, env)
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
