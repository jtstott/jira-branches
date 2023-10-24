use std::collections::HashMap;
use serde::Deserialize;
use crate::jira::auth::JiraAuth;

pub mod file_parser;
pub mod init;
pub mod config_writer;
pub mod config_wizard;
mod autocomplete_template;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct UserConfig {
    pub base_url: String,
    pub branch_template: String,
    pub options: Option<Options>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Options {
    pub id_prefix: Option<String>,
    pub map_types: Option<HashMap<String, String>>,
    pub case: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct AppConfig {
    pub auth: JiraAuth,
    pub config: UserConfig,
}

// impl Clone for UserConfig {
//     fn clone(&self) -> UserConfig {
//         *self
//     }
// }
