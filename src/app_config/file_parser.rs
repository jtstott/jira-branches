use config::{Config, ConfigError};
use serde::Deserialize;
use crate::app_config::{AppConfig, UserConfig};
use crate::jira::auth::JiraAuth;
use std::error::Error;
use std::fs::File;

const CONFIG_FILE: &str = "config.json";
const AUTH_FILE: &str = "auth.json";

pub fn read_config_file(path: Option<&str>) -> Result<Option<UserConfig>, ConfigError> {
    let default = get_config_path(CONFIG_FILE);
    let file_path = path.unwrap_or(default.as_str());
    parse_file::<UserConfig>(file_path)
}

pub fn read_auth_file(path: Option<&str>) -> Result<Option<JiraAuth>, ConfigError> {
    let default = get_config_path(AUTH_FILE);
    let file_path = path.unwrap_or(default.as_str());
    parse_file::<JiraAuth>(file_path)
}

pub fn write_config(config: &AppConfig) -> Result<(), Box<dyn Error>> {
    write_user_config(&config.config)?;
    write_auth(&config.auth)?;
    Ok(())
}

pub fn write_user_config(config: &UserConfig) -> Result<(), Box<dyn Error>> {
    let mut file = create_file(CONFIG_FILE)?;
    serde_json::to_writer_pretty(&mut file, &config)?;

    Ok(())
}

pub fn write_auth(auth: &JiraAuth) -> Result<(), Box<dyn Error>> {
    let mut file = create_file(AUTH_FILE)?;
    serde_json::to_writer_pretty(&mut file, &auth)?;

    Ok(())
}

fn create_file(file_name: &str) -> Result<File, Box<dyn Error>> {
    Ok(File::create(get_config_path(file_name))?)
}

fn parse_file<'de, F: Deserialize<'de>>(path: &str) -> Result<Option<F>, ConfigError> {
    let config = Config::builder()
        .add_source(config::File::with_name(path))
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