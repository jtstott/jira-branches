use std::error::Error;
use crate::cli::cli_parser;
use crate::app_config::init;
use crate::git;
use crate::jira::issue;

pub async fn handle(cli: &cli_parser::Cli, issue: String) -> Result<(), Box<dyn Error>> {
    let config = init::initialize_config(Some(cli))?;
    let issue_details = issue::get_issue(issue.as_str(), &config).await?;
    let branch_name = crate::branch::template::interpret_branch_template(&config.config, issue_details);

    git::checkout::checkout_branch(branch_name.as_str());
    Ok(())
}