use std::collections::HashMap;
use serde::Deserialize;

pub mod file_parser;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    auth: Auth,
    base_url: String,
    branch_template: String,
    options: Options
}

#[derive(Debug, Deserialize)]
pub struct Auth {
    user: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct Options {
    id_prefix: String,
    map_types: HashMap<String, String>
}