use config::Config;
use serde::Deserialize;
use crate::app_config::UserConfig;
use crate::jira::auth::JiraAuth;

pub fn read_config_file() -> UserConfig {
    parse_file::<UserConfig>("config.json")
}

pub fn read_auth_file() -> JiraAuth {
    parse_file::<JiraAuth>("auth.json")
}

fn parse_file<'de, F: Deserialize<'de>>(file_name: &str) -> F {
    let path = get_config_path(file_name);
    let config = Config::builder()
        .add_source(config::File::with_name(path.as_str()))
        .build()
        .unwrap();

    config
        .try_deserialize::<F>()
        .unwrap()
}

fn get_config_path(file: &str) -> String {
    format!("{}/.config/jira-branches/{}", dirs::home_dir().unwrap().as_path().to_str().unwrap(), file)
}