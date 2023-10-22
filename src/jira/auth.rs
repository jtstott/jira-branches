use crate::cli::cli_parser::Cli;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct JiraAuth {
    pub user: String,
    pub password: String,
}

impl JiraAuth {
    pub fn from_cli(cli: &Cli) -> Option<JiraAuth> {
        if let (Some(u), Some(p)) = (&cli.username, &cli.password) {
            return Some(JiraAuth {
                user: u.clone(),
                password: p.clone(),
            })
        }
        None
    }
}