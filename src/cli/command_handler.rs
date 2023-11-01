use std::error::Error;
use std::path::PathBuf;
use crate::app_config::{init};
use crate::app_config::config_wizard::config_wizard;
use crate::app_config::file_parser::{read_config_file, write_user_config};
use crate::branch::template;
use crate::cli::cli_parser::{self, Cli};
use crate::git::checkout;
use crate::jira::issue;

pub async fn handle_command(cli: Cli) -> Result<(), Box<dyn Error>> {
    match &cli.command {
        cli_parser::Commands::Checkout { issue } => { handle_checkout(&cli, issue.to_owned()).await?; }
        cli_parser::Commands::Config { file } => { handle_configure(file.to_owned())?; }
    }

    Ok(())
}

async fn handle_checkout(cli: &Cli, issue: String) -> Result<(), Box<dyn Error>> {
    let config = init::initialize_config(Some(cli))?;
    let issue_details = issue::get_issue(issue.as_str(), &config).await?;
    let branch_name = template::interpret_branch_template(&config.config, issue_details);

    checkout::checkout_branch(branch_name.as_str());
    Ok(())
}

fn handle_configure(file: Option<PathBuf>) -> Result<(), Box<dyn Error>> {
    if let Some(file) = file {
        let file_config = read_config_file(file.to_str())?;
        if let Some(uc) = file_config {
            write_user_config(&uc)?;
        }

        return Ok(())
    }

    config_wizard()?;

    Ok(())
}
