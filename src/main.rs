mod jira;
mod cli;
mod app_config;
mod branch;
mod git;

use std::error::Error;
use std::process::ExitCode;
use colored::Colorize;
use crate::cli::{cli_parser, command_handler};

#[tokio::main]
async fn main() -> ExitCode {
    let result = run().await;
    match result {
        Ok(_) => ExitCode::from(0),
        Err(err) => {
            eprintln!("{}", format!("ERROR: {:?}", err).red());
            ExitCode::from(1)
        },
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let cli = cli_parser::parse();
    command_handler::handle_command(cli).await?;

    Ok(())
}
