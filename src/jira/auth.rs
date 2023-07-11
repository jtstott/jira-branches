use crate::cli::cli_parser::Cli;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JiraAuth {
    pub username: String,
    pub password: Option<String>,
}

impl JiraAuth {
    pub fn from_cli(cli: &Cli) -> JiraAuth {
        JiraAuth {
            username: cli.username.clone().unwrap(),
            password: cli.password.clone(),
        }
    }
}