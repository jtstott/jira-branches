use config::ConfigError;
use crate::app_config::{AppConfig};
use crate::app_config::config_loader;
use crate::cli::cli_parser::Cli;

pub fn initialize_config(cli: Option<&Cli>) -> Result<AppConfig, ConfigError> {
    Ok(AppConfig {
        auth: config_loader::load_auth(cli)?,
        config: config_loader::load_user_config()?,
    })
}
