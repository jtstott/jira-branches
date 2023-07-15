use crate::app_config::AppConfig;
use crate::branch::template;
use crate::cli::cli_parser::{self, Cli};
use crate::git::checkout::checkout_branch;
use crate::jira::issue;

pub async fn handle_command(cli: Cli, config: AppConfig) {
    // let cli = cli_parser::parse();

    match cli.command {
        cli_parser::Commands::Checkout { ref issue_id } => { handle_checkout(&config, &issue_id).await; }
        // None => {}
    }

    // if let Some(config_path) = cli.config.as_deref() {
    //     println!("Value for config: {}", config_path.display());
    // }
}

async fn handle_checkout(config: &AppConfig, issue_id: &String) {
    println!("Value for ticket ID: {}", issue_id.as_str());

    let issue_details = issue::get_issue(issue_id, &config).await;
    let branch_name = template::interpret_branch_template(config, issue_details);
    checkout_branch(branch_name);
}