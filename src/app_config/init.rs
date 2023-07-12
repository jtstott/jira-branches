use crate::app_config::AppConfig;
use crate::app_config::file_parser;
use crate::cli::cli_parser::Cli;
use crate::jira::auth::JiraAuth;

pub fn initialize_config(cli: &Cli) -> AppConfig {
    let mut config = file_parser::read_config_file();

    if let Some(cli_auth) = JiraAuth::from_cli(cli) {
        config.auth = cli_auth
    }

    println!("{:#?}", config);

    config
}