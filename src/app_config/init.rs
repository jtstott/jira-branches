use crate::app_config::{AppConfig};
use crate::app_config::file_parser;
use crate::cli::cli_parser::Cli;
use crate::jira::auth::JiraAuth;

pub fn initialize_config(cli: &Cli) -> AppConfig {
    let config = file_parser::read_config_file();
    let mut auth = file_parser::read_auth_file();

    if let Some(cli_auth) = JiraAuth::from_cli(cli) {
        auth = cli_auth
    }

    AppConfig {
        auth,
        config
    }
}