use crate::app_config::AppConfig;
use crate::branch::template;
use crate::cli::cli_parser::{self, Cli};
use crate::git::checkout;
use crate::jira::issue;

pub async fn handle_command(cli: Cli, config: AppConfig) -> Result<(), String> {
    match cli.command {
        cli_parser::Commands::Checkout { issue } => { handle_checkout(&config, issue).await?; }
    }

    Ok(())
}

async fn handle_checkout(config: &AppConfig, issue: String) -> Result<(), String> {
    let issue_details = issue::get_issue(issue.as_str(), &config).await;
    let branch_name = template::interpret_branch_template(&config.config, issue_details?);
    // checkout::checkout_branch(branch_name.as_str());
    Ok(())
}