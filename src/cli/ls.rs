use std::error::Error;
use crate::git::list::list_branches;
use crate::jira::issue;

pub fn handle(issue_ref: String) -> Result<(), Box<dyn Error>>{
    let issue_id = issue::extract_id(issue_ref.as_str())?;
    list_branches(issue_id)?;
    Ok(())
}