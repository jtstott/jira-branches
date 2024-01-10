use std::error::Error;
use crate::app_config::init;
use crate::cli::cli_parser;
use crate::git::list;
use crate::jira::issue;

pub async fn handle(cli: &cli_parser::Cli, issue_ref: String) -> Result<(), Box<dyn Error>>{
    let config = init::initialize_config(Some(cli))?;
    let issue_id = issue::get_ticket_id(issue_ref.as_str(), &config)?;
    list::list_branches(issue_id, &config).await?;
    Ok(())
}