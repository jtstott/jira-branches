use std::collections::HashMap;
use serde::Deserialize;
use crate::jira::auth::JiraAuth;

pub mod file_parser;
pub mod init;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub auth: JiraAuth,
    pub base_url: String,
    pub branch_template: String,
    pub options: Options
}

#[derive(Debug, Deserialize)]
pub struct Options {
    pub id_prefix: String,
    pub map_types: HashMap<String, String>
}