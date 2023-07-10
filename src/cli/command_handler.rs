use crate::cli::cli_parser::{self, Cli};
use crate::jira::auth::Auth;
use crate::jira::issue;

pub async fn handle_command() {
    let cli = cli_parser::parse();

    match cli.command {
        cli_parser::Commands::Checkout { ref issue_id } => { handle_checkout(&cli, &issue_id).await; }
        // None => {}
    }

    // if let Some(config_path) = cli.config.as_deref() {
    //     println!("Value for config: {}", config_path.display());
    // }
}

async fn handle_checkout(cli: &Cli, issue_id: &String) {
    println!("Value for ticket ID: {}", issue_id.as_str());

    let auth = get_authentication(cli);

    issue::get_issue("CORG-10568", &auth).await;
}

fn get_authentication(cli: &Cli) -> Auth {
    Auth {
        username: cli.username.clone().unwrap_or(String::new()),
        password: cli.password.clone(),
    }
}