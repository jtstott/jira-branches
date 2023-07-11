mod jira;
mod cli;
mod app_config;

use std::collections::HashMap;
use crate::cli::command_handler;
use app_config::file_parser::{read_config_file, AUTH};

#[tokio::main]
async fn main() {
    let config = read_config_file();
    println!("{:#?}", config)
    // command_handler::handle_command().await
}
