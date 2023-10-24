use std::error::Error;
use config::ConfigError;
// use std::collections::HashMap;
use crate::app_config::{AppConfig, init};
use crate::app_config::config_wizard::config_wizard;
use crate::branch::template;
use crate::cli::cli_parser::{self, Cli};
use crate::git::checkout;
// use crate::jira::auth::JiraAuth;
use crate::jira::issue;

pub async fn handle_command(cli: Cli) -> Result<(), Box<dyn Error>> {
    match &cli.command {
        cli_parser::Commands::Checkout { issue } => { handle_checkout(&cli, issue.to_owned()).await?; }
        cli_parser::Commands::Config { .. } => { handle_configure()?; }
    }

    Ok(())
}

async fn handle_checkout(cli: &Cli, issue: String) -> Result<(), Box<dyn Error>> {
    let config = init::initialize_config(Some(cli))?;
    let issue_details = issue::get_issue(issue.as_str(), &config).await?;
    let branch_name = template::interpret_branch_template(&config.config, issue_details);

    println!("{}", branch_name);
    // checkout::checkout_branch(branch_name.as_str());
    Ok(())
}

fn handle_configure() -> Result<(), ConfigError> {
    let wizard_config = config_wizard();
    // println!("Config: {:#?}", wizard_config);

    Ok(())
}
