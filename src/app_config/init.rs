use config::ConfigError;
use crate::app_config::{AppConfig, UserConfig};
use crate::app_config::file_parser;
use crate::cli::cli_parser::Cli;
use crate::jira::auth::JiraAuth;

pub fn initialize_config(cli: Option<&Cli>) -> Result<AppConfig, ConfigError> {
    Ok(AppConfig {
        auth: load_auth(cli)?,
        config: load_user_config()?,
    })
}

pub fn load_user_config() -> Result<UserConfig, ConfigError>  {
    let config = file_parser::read_config_file()?;

    if config.is_none() {
        return Err(ConfigError::Message(no_config_error()));
    };

    Ok(config.unwrap())
}

pub fn load_auth(cli: Option<&Cli>) -> Result<JiraAuth, ConfigError>  {
    let mut auth = file_parser::read_auth_file()?;

    if let Some(c) = cli {
        if let Some(cli_auth) = JiraAuth::from_cli(c) {
            auth = Some(cli_auth)
        }
    }


    if auth.is_none() {
        return Err(ConfigError::Message(no_auth_error()));
    };

    Ok(auth.unwrap())
}

fn no_auth_error() -> String {
    r"Authorization not found
To configure authorization create an auth.json file or run the configuration wizard.
Alternatively run pass the -u and -p arguments to pass authorization one time.".into()
}

fn no_config_error() -> String {
    r"Configuration not found
To configure jira branches create a config.json file or run the configuration wizard.".into()
}