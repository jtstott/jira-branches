use std::env;
use std::error::Error;
use colored::Colorize;
use git2::{Repository};
use crate::app_config::AppConfig;
use crate::branch;
use crate::jira::issue;

pub async fn list_branches(issue_id: String, config: &AppConfig) -> Result<(), Box<dyn Error>> {
    let repo = Repository::discover(env::current_dir().unwrap())
        .expect("Not a git repository");

    println!("🔎 Finding branches for issue: {}...\n", issue_id);

    let issue_details = issue::get_issue(issue_id.as_str(), config).await?;
    let exact_branch_name = branch::template::interpret_branch_template(&config.config, issue_details);

    let branches = repo.branches(None).unwrap();

    let filtered = branches.filter(|b| {
        let branch_name = b.as_ref().unwrap().0.name();
        branch_name.unwrap().unwrap().contains(issue_id.as_str())
    });

    let mut count = 0;
    for branch in filtered {
        let b = branch.unwrap();
        count += 1;
        let mut branch_str = b.0.name()?.unwrap_or_default().to_string();
        if b.0.name()?.unwrap_or_default() == exact_branch_name {
            branch_str = branch_str.green().to_string()
        }
        println!("{}", branch_str);
    }

    if count == 0 {
        eprintln!("{}", format!("No branches match the issue ID: {}", issue_id).red());
    }

    Ok(())
}