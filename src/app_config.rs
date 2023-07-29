use std::collections::HashMap;
use serde::Deserialize;
use crate::jira::auth::JiraAuth;

pub mod file_parser;
pub mod init;

#[derive(Debug, Deserialize)]
pub struct UserConfig {
    pub base_url: String,
    pub branch_template: String,
    pub options: Option<Options>
}

#[derive(Debug, Deserialize)]
pub struct Options {
    pub id_prefix: Option<String>,
    pub map_types: Option<HashMap<String, String>>,
    pub case: Option<HashMap<String, String>>
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub auth: JiraAuth,
    pub config: UserConfig
}