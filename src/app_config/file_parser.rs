use config::{Config, ConfigError};
use serde::Deserialize;
// use serde_json::Error;
use crate::app_config::UserConfig;
use crate::jira::auth::JiraAuth;
use std::error::Error;

pub fn read_config_file() -> Result<Option<UserConfig>, ConfigError> {
    parse_file::<UserConfig>("config.json")
}

pub fn read_auth_file() -> Result<Option<JiraAuth>, ConfigError> {
    parse_file::<JiraAuth>("auth.json")
}

pub fn write_config_file(config: &UserConfig) -> Result<(), Box<dyn Error>> {
    let mut file = std::fs::File::create(get_config_path("config.json"))?;
    serde_json::to_writer_pretty(&mut file, &config)?;

    Ok(())
}

fn parse_file<'de, F: Deserialize<'de>>(file_name: &str) -> Result<Option<F>, ConfigError> {
    let path = get_config_path(file_name);
    let config = Config::builder()
        .add_source(config::File::with_name(path.as_str()))
        .build()
        .ok();

    if let Some(conf) = config {
        let c = conf.try_deserialize::<F>()?;
        return Ok(Some(c));
    }
    Ok(None)
}

fn get_config_path(file: &str) -> String {
    format!("{}/.config/jira-branches/{}", dirs::home_dir().unwrap().as_path().to_str().unwrap(), file)
}