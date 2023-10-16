mod jira;
mod cli;
mod app_config;
mod branch;
mod git;

use std::error::Error;
use std::fmt::format;
use std::process::ExitCode;
use colored::Colorize;
use config::ConfigError;
use crate::cli::{cli_parser, command_handler};
use crate::app_config::{AppConfig, init};

#[tokio::main]
async fn main() -> ExitCode {
    // let cli = cli_parser::parse();
    // let config = init::initialize_config(Some(&cli));
    //
    // match config {
    //     Ok(_) => {}
    //     Err(_) => {}
    // }

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
    let config = init::initialize_config(Some(&cli))?;
    command_handler::handle_command(cli, config).await?;

    Ok(())
}
