mod jira;
mod cli;
mod app_config;
mod branch;
mod git;

use crate::cli::{cli_parser, command_handler};
use crate::app_config::init;

#[tokio::main]
async fn main() {
    let cli = cli_parser::parse();
    let config = init::initialize_config(&cli);
    command_handler::handle_command(cli, config).await
}
