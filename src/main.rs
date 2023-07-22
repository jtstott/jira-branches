mod jira;
mod cli;
mod app_config;
mod branch;
mod git;

use std::process::ExitCode;
use crate::cli::{cli_parser, command_handler};
use crate::app_config::init;

#[tokio::main]
async fn main() -> ExitCode {
    let cli = cli_parser::parse();
    let config = init::initialize_config(&cli);
    let result = command_handler::handle_command(cli, config).await;
    match result {
        Ok(_) => ExitCode::from(0),
        Err(err) => {
            eprintln!("ERROR: {}", err);
            ExitCode::from(1)
        },
    }
}
