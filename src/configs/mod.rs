use std::collections::BTreeMap;
use std::fs;

pub type Configs = BTreeMap<String, AppConfig>;
type AppConfig = BTreeMap<String, Vec<String>>;

const CONFIG_FILE_PATH: &str = "src/configs/heroku_apps.yml";

pub fn get() -> Configs {
    let file = fs::read_to_string(CONFIG_FILE_PATH).expect("Failure");
    serde_yaml::from_str(&file).expect("Failure")
}
