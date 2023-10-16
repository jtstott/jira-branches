// use std::collections::HashMap;
use crate::app_config::{AppConfig};
use crate::app_config::config_wizard::config_wizard;
use crate::app_config::config_wizard_inquire::config_wizard_inq;
use crate::branch::template;
use crate::cli::cli_parser::{self, Cli};
use crate::git::checkout;
// use crate::jira::auth::JiraAuth;
use crate::jira::issue;

pub async fn handle_command(cli: Cli, config: AppConfig) -> Result<(), String> {
    match cli.command {
        cli_parser::Commands::Checkout { issue } => { handle_checkout(&config, issue).await?; }
        cli_parser::Commands::Config { .. } => { handle_configure() }
    }

    Ok(())
}

async fn handle_checkout(config: &AppConfig, issue: String) -> Result<(), String> {
    let issue_details = issue::get_issue(issue.as_str(), &config).await;
    let branch_name = template::interpret_branch_template(&config.config, issue_details?);

    println!("{}", branch_name);
    checkout::checkout_branch(branch_name.as_str());
    Ok(())
}

fn handle_configure() {
    // let wizard_config = config_wizard();
    let wizard_config = config_wizard_inq();
    println!("Config: {:#?}", wizard_config)

    // println!("Handling configure: {:?}", wizard_config);
}
