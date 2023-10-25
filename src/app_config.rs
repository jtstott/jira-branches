use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::jira::auth::JiraAuth;

pub mod file_parser;
pub mod init;
pub mod config_writer;
pub mod config_wizard;
mod token_completer;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct UserConfig {
    pub base_url: String,
    pub branch_template: String,
    pub options: Option<Options>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Options {
    pub id_prefix: Option<String>,
    pub map_types: Option<HashMap<String, String>>,
    pub case: Option<HashMap<String, String>>,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct AppConfig {
    pub auth: JiraAuth,
    pub config: UserConfig,
}

// impl Clone for UserConfig {
//     fn clone(&self) -> UserConfig {
//         *self
//     }
// }
